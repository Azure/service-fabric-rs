# BoxPool and GetRawWithBoxPool Guide

## Problem

Service Fabric COM APIs accept C structs containing raw pointers to other structs.
In Rust, these pointed-to structs must live on the heap and stay alive for the
duration of the COM call. Without a helper, each FFI conversion needs a dedicated
`*Raw` wrapper struct whose only job is holding `Box` fields to extend lifetimes.

## BoxPool

`BoxPool` (defined in `crate::mem`) is a simple arena that owns heap allocations
and keeps them alive until the pool is dropped.

```rust
use crate::mem::BoxPool;

let mut pool = BoxPool::new();

// Push a Box, get back a raw pointer valid for the pool's lifetime.
let ptr: *const MyStruct = pool.push(Box::new(MyStruct { ... }));

// Push a Vec, get back (length, raw pointer).
let (len, ptr): (usize, *const Item) = pool.push_vec(vec![item1, item2]);

// All pointers remain valid here.
// When `pool` is dropped, all allocations are freed.
```

Key properties:
- `push` takes a `Box<T>` and returns `*const T`.
- `push_vec` takes a `Vec<T>` and returns `(usize, *const T)`.
- All returned pointers are valid as long as the `BoxPool` is alive.

## GetRawWithBoxPool Trait

```rust
pub trait GetRawWithBoxPool<T> {
    fn get_raw_with_pool(&self, pool: &mut BoxPool) -> T;
}
```

Implement this trait to convert a Rust type into its FFI representation. The pool
holds any intermediate heap allocations needed by the returned struct's pointer
fields.

### Basic Example — Simple Struct

When the FFI struct has no pointer fields that need heap backing, the pool is unused:

```rust
impl GetRawWithBoxPool<FABRIC_SOME_SIMPLE_DESCRIPTION> for MySimpleType {
    fn get_raw_with_pool(&self, _pool: &mut BoxPool) -> FABRIC_SOME_SIMPLE_DESCRIPTION {
        FABRIC_SOME_SIMPLE_DESCRIPTION {
            Name: self.name.as_pcwstr(),
            Value: self.value,
            Reserved: std::ptr::null_mut(),
        }
    }
}
```

### Extension Chain Pattern

Service Fabric uses `Reserved` pointer chains for versioning (EX1 → EX2 → …).
Build from the tail and use `pool.push` for each extension struct:

```rust
impl GetRawWithBoxPool<FABRIC_STATELESS_SERVICE_DESCRIPTION> for StatelessServiceDescription {
    fn get_raw_with_pool(
        &self,
        pool: &mut BoxPool,
    ) -> FABRIC_STATELESS_SERVICE_DESCRIPTION {
        // Build the chain from the tail (ex4 → ex3 → ex2 → ex1 → base).
        let ex4 = pool.push(Box::new(FABRIC_STATELESS_SERVICE_DESCRIPTION_EX4 {
            ServiceScalingPolicies: std::ptr::null_mut(),
            ScalingPolicyCount: 0,
            Reserved: std::ptr::null_mut(),
        }));
        let ex3 = pool.push(Box::new(FABRIC_STATELESS_SERVICE_DESCRIPTION_EX3 {
            ServicePackageActivationMode: self.service_package_activation_mode.into(),
            Reserved: ex4 as *const _ as *mut c_void,
        }));
        let ex2 = pool.push(Box::new(FABRIC_STATELESS_SERVICE_DESCRIPTION_EX2 {
            IsDefaultMoveCostSpecified: self.default_move_cost.is_some(),
            DefaultMoveCost: self.default_move_cost.unwrap_or(MoveCost::Zero).into(),
            Reserved: ex3 as *const _ as *mut c_void,
        }));
        let ex1 = pool.push(Box::new(FABRIC_STATELESS_SERVICE_DESCRIPTION_EX1 {
            PolicyList: std::ptr::null_mut(),
            Reserved: ex2 as *const _ as *mut c_void,
        }));

        // Return the base struct by value — no Box needed for it.
        FABRIC_STATELESS_SERVICE_DESCRIPTION {
            ApplicationName: self.application_name.as_raw(),
            ServiceName: self.service_name.as_raw(),
            InstanceCount: self.instance_count,
            Reserved: ex1 as *const _ as *mut c_void,
            // ... other fields
        }
    }
}
```

### Composing Implementations

When one type nests another, call the inner type's `get_raw_with_pool` with the
same pool. If the COM API expects a pointer to the inner struct, box the result
through the pool:

```rust
impl GetRawWithBoxPool<FABRIC_SERVICE_DESCRIPTION> for ServiceDescription {
    fn get_raw_with_pool(&self, pool: &mut BoxPool) -> FABRIC_SERVICE_DESCRIPTION {
        match self {
            ServiceDescription::Stateless(desc) => {
                let raw = desc.get_raw_with_pool(pool);
                // Box the inner struct so we can take a pointer to it.
                let raw_ptr = pool.push(Box::new(raw));
                FABRIC_SERVICE_DESCRIPTION {
                    Kind: FABRIC_SERVICE_DESCRIPTION_KIND_STATELESS,
                    Value: raw_ptr as *const _ as *mut c_void,
                }
            }
            // ...
        }
    }
}
```

### Optional Inner Structs

When a field is conditionally present, return `Option<T>` from the trait and let
the caller decide whether to box it:

```rust
impl GetRawWithBoxPool<Option<FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS>>
    for StatefulServiceFailoverSettings
{
    fn get_raw_with_pool(
        &self,
        pool: &mut BoxPool,
    ) -> Option<FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS> {
        if self.is_empty() {
            return None;
        }
        // ... build extension chain with pool ...
        Some(FABRIC_STATEFUL_SERVICE_FAILOVER_SETTINGS { ... })
    }
}

// Caller converts Option to a pointer:
let failover_raw = self.failover_settings.get_raw_with_pool(pool);
let failover_ptr = failover_raw
    .map(|s| pool.push(Box::new(s)) as *const _ as *mut _)
    .unwrap_or(std::ptr::null_mut());
```

### Using push_vec for Arrays

When a COM struct takes a count + pointer pair, use `pool.push_vec`:

```rust
fn get_raw_with_pool(&self, pool: &mut BoxPool) -> FABRIC_SOME_LIST {
    let items: Vec<FABRIC_ITEM> = self.items.iter()
        .map(|item| item.get_raw_with_pool(pool))
        .collect();
    let (count, items_ptr) = pool.push_vec(items);
    FABRIC_SOME_LIST {
        Count: count as u32,
        Items: items_ptr as *mut _,
        Reserved: std::ptr::null_mut(),
    }
}
```

## Caller Pattern

At the call site, create a pool, convert, and pass to the COM API. The pool must
stay alive until the async begin call captures the FFI data:

```rust
pub async fn create_service(
    &self,
    desc: &ServiceDescription,
    timeout: Duration,
    cancellation_token: Option<BoxedCancelToken>,
) -> crate::Result<()> {
    {
        let mut pool = BoxPool::new();
        let ffi_raw = desc.get_raw_with_pool(&mut pool);
        self.create_service_internal(&ffi_raw, timeout.as_millis() as u32, cancellation_token)
    }
    .await?
    .map_err(crate::Error::from)
}
```

The `{ ... }` block ensures `pool` (and all its allocations) lives long enough for
`create_service_internal` to capture the data in its `Begin*` call, but is dropped
before the `.await` suspend point — avoiding unnecessarily holding heap memory
across the await.

## When to Use GetRaw vs GetRawWithBoxPool

| Trait | Use when |
|---|---|
| `GetRaw<T>` | The FFI struct has no pointer fields that need heap backing (simple value copy). |
| `GetRawWithBoxPool<T>` | The FFI struct contains pointers to other heap-allocated structs. |

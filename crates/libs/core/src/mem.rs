// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

/// A pool to hold boxes to extend their lifetime.
/// This is typically useful to build raw COM structures that require
/// raw pointers in struct fields.
#[derive(Debug, Default)]
pub struct BoxPool {
    inner: Vec<Box<dyn 'static + std::any::Any>>,
}

impl BoxPool {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    /// Push a box into the pool, and return a raw pointer to the boxed value.
    /// The pointer is valid as long as the pool is alive.
    #[must_use]
    pub fn push<T: 'static>(&mut self, b: Box<T>) -> *const T {
        let raw = b.as_ref() as *const T;
        self.inner.push(b);
        raw
    }

    /// Push a Vec<T> into the pool, and return its length and a raw pointer to its data.
    /// The pointer is valid as long as the pool is alive.
    #[must_use]
    pub fn push_vec<T: 'static>(&mut self, v: Vec<T>) -> (usize, *const T) {
        let len = v.len();
        let raw = v.as_ptr();
        // Convert Vec<T> to Box<dyn Any>.
        let boxed_v = Box::new(v);
        self.inner.push(boxed_v);
        (len, raw)
    }
}

/// Trait to get a raw pointer from a type, using a BoxPool to hold the box.
/// This is useful to implement conversions to raw COM types that require
/// raw pointers in struct fields.
pub trait GetRawWithBoxPool<T> {
    fn get_raw_with_pool(&self, pool: &mut BoxPool) -> T;
}

/// Trait to get a raw pointer from a type.
/// Type should implement this trait if it can return a raw pointer without BoxPool.
pub trait GetRaw<T> {
    fn get_raw(&self) -> T;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_pool() {
        let mut pool = BoxPool::new();
        let b = Box::new(42);
        let raw = pool.push(b);
        assert_eq!(unsafe { *raw }, 42);

        let v = vec![1, 2, 3];
        let (len, raw_v) = pool.push_vec(v);
        assert_eq!(len, 3);
        assert_eq!(unsafe { *raw_v }, 1);
    }
}

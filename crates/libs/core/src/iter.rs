// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

// iterator implementation
// Iter infrastructure to convert Fabric raw list into rust safe wrappers.
// Raw lists needs to be wrapped in FabricListAccessor, and raw item needs to
// implement From<T> trait to convert to rust safe struct, then the FabricIter
// enables the mechanism to convert item one by one while iterating.

use std::marker::PhantomData;

// Access fabric list metadata
// T is the fabric raw type that needs to iterate through by pointer arithmetic
pub trait FabricListAccessor<T> {
    fn get_count(&self) -> u32;
    fn get_first_item(&self) -> *const T;
}

// T is the raw fabric type
// R is the safe type to convert to
// O is the memory owner reference
// R can be converted to T using the From trait
pub struct FabricIter<'b, T, R, O>
where
    R: for<'a> std::convert::From<&'a T>,
{
    _owner: &'b O, // owns the memory that the curr ptr points to. Typically this is a COM obj.
    count: u32,    // total
    index: u32,
    curr: *const T,
    phantom: PhantomData<R>, // R is the converted type
}

impl<'b, T, R, O> FabricIter<'b, T, R, O>
where
    R: for<'a> std::convert::From<&'a T>,
{
    pub fn new(accessor: &impl FabricListAccessor<T>, owner: &'b O) -> Self {
        let count = accessor.get_count();
        let first = accessor.get_first_item();
        Self {
            count,
            index: 0,
            curr: first,
            phantom: PhantomData {},
            _owner: owner,
        }
    }
}

impl<T, R, O> Iterator for FabricIter<'_, T, R, O>
where
    R: for<'a> std::convert::From<&'a T>,
{
    type Item = R;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.count {
            return None;
        }
        // get the curr out
        let raw = unsafe { self.curr.as_ref().unwrap() };

        let res: R = raw.into();
        self.index += 1;
        self.curr = unsafe { self.curr.offset(1) };
        Some(res)
    }
}

/// Convert a raw pointer and length into a Vec of safe type.
pub(crate) fn vec_from_raw_com<T, V>(len: usize, raw: *const T) -> Vec<V>
where
    V: for<'a> std::convert::From<&'a T>,
{
    if len == 0 || raw.is_null() {
        return vec![];
    }
    if raw.is_aligned() {
        unsafe {
            std::slice::from_raw_parts(raw, len)
                .iter()
                .map(|x| x.into())
                .collect()
        }
    } else {
        // Sometimes SF COM ptr is not aligned, but is verified to be correct during testing.
        // Ptr not aligned, need to copy one by one
        let mut v = Vec::with_capacity(len);
        for i in 0..len {
            let p = unsafe { raw.add(i) };
            let r = unsafe { p.as_ref().unwrap() };
            v.push(r.into());
        }
        v
    }
}

#[cfg(test)]
mod test {

    use super::{FabricIter, FabricListAccessor};

    struct MyVal {
        val: String,
    }

    struct MyVal2 {
        val: String,
    }

    impl From<&MyVal> for MyVal2 {
        fn from(value: &MyVal) -> Self {
            Self {
                val: value.val.clone() + "Suffix",
            }
        }
    }

    struct MyVec {
        v: Vec<MyVal>,
    }

    impl FabricListAccessor<MyVal> for MyVec {
        fn get_count(&self) -> u32 {
            self.v.len() as u32
        }

        fn get_first_item(&self) -> *const MyVal {
            self.v.as_ptr()
        }
    }

    type MyVecIter<'a> = FabricIter<'a, MyVal, MyVal2, MyVec>;

    impl MyVec {
        fn get_iter(&self) -> MyVecIter<'_> {
            MyVecIter::new(self, self)
        }
    }

    #[test]
    fn test_vector() {
        let v = MyVec {
            v: vec![
                MyVal {
                    val: "hi".to_string(),
                },
                MyVal {
                    val: "hi2".to_string(),
                },
            ],
        };

        let it = v.get_iter();
        let vv = it.collect::<Vec<_>>();
        assert_eq!(vv.len(), 2);
        assert_eq!(vv.first().unwrap().val, "hiSuffix");
        assert_eq!(vv.last().unwrap().val, "hi2Suffix");
    }
}

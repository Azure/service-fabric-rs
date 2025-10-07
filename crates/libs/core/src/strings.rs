// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use crate::WString;
use mssf_com::FabricCommon::{
    IFabricStringListResult, IFabricStringResult, IFabricStringResult_Impl,
};
use windows_core::implement;

// Basic implementation of fabric string result
// usually used as string return value to fabric runtime.
#[derive(Debug)]
#[implement(IFabricStringResult)]
pub struct StringResult {
    data: WString,
}

// Recommend to use WStringWrap to construct this and convert to
// IFabricStringResult.
impl StringResult {
    pub fn new(data: WString) -> StringResult {
        StringResult { data }
    }

    /// Get the inner WString
    pub fn into_inner(self) -> WString {
        self.data
    }
}

impl IFabricStringResult_Impl for StringResult_Impl {
    fn get_String(&self) -> crate::PCWSTR {
        // This is some hack to get the raw pointer out.
        crate::PCWSTR::from_raw(self.data.as_ptr())
    }
}

impl From<&IFabricStringResult> for StringResult {
    fn from(value: &IFabricStringResult) -> Self {
        let content = unsafe { value.get_String() };
        Self {
            data: WString::from(content),
        }
    }
}

// IFabricStringListResult
pub struct WStringList {
    data: Vec<WString>,
}

impl WStringList {
    pub fn into_vec(self) -> Vec<WString> {
        self.data
    }
}

impl From<&IFabricStringListResult> for WStringList {
    fn from(value: &IFabricStringListResult) -> Self {
        // cpp code should not error if the parameters are not null.
        let mut itemcount = 0_u32;
        let first_str = unsafe {
            value
                .GetStrings(std::ptr::addr_of_mut!(itemcount))
                .expect("cannot get strings")
        };
        let data = crate::iter::vec_from_raw_com(itemcount as usize, first_str);
        Self { data }
    }
}

#[cfg(test)]
mod test {

    use super::StringResult;
    use crate::WString;
    use mssf_com::FabricCommon::IFabricStringResult;

    #[test]
    fn test_str_addr() {
        // Test the addr returned to SF is right.
        let addr = "1.2.3.4:1234";

        // Check wstring len.
        let haddr = WString::from(addr);
        let haddr_slice = haddr.as_wide();
        assert_eq!(haddr_slice.len(), 12);

        // check StringResult len.
        let com_addr: IFabricStringResult = StringResult::new(haddr.clone()).into();
        let raw = unsafe { com_addr.get_String() };
        let slice = unsafe { raw.as_wide() };
        assert_eq!(slice.len(), 12);

        // check StringResult conversion is right
        let haddr2: WString = StringResult::from(&com_addr).into_inner();
        assert_eq!(haddr, haddr2);
    }
}

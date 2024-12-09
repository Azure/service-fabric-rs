// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use crate::{HSTRING, PCWSTR};
use mssf_com::FabricCommon::{IFabricStringResult, IFabricStringResult_Impl};
use windows_core::implement;

// Basic implementation of fabric string result
// usually used as string return value to fabric runtime.
#[derive(Debug)]
#[implement(IFabricStringResult)]
pub struct StringResult {
    data: HSTRING,
}

// Recommend to use HSTRINGWrap to construct this and convert to
// IFabricStringResult.
impl StringResult {
    pub fn new(data: HSTRING) -> StringResult {
        StringResult { data }
    }
}

impl IFabricStringResult_Impl for StringResult_Impl {
    fn get_String(&self) -> windows_core::PCWSTR {
        // This is some hack to get the raw pointer out.
        windows_core::PCWSTR::from_raw(self.data.as_ptr())
    }
}

// If nullptr returns empty string.
// requires the PCWSTR points to a valid buffer with null terminatior
fn safe_pwstr_to_hstring(raw: PCWSTR) -> HSTRING {
    if raw.is_null() {
        return HSTRING::new();
    }
    HSTRING::from_wide(unsafe { raw.as_wide() })
}

// Convert helper for HSTRING and PCWSTR and IFabricStringResult
pub struct HSTRINGWrap {
    h: HSTRING,
}

impl HSTRINGWrap {
    pub fn into_hstring(self) -> HSTRING {
        self.h
    }
}

impl From<HSTRING> for HSTRINGWrap {
    fn from(value: HSTRING) -> Self {
        Self { h: value }
    }
}

impl From<PCWSTR> for HSTRINGWrap {
    fn from(value: PCWSTR) -> Self {
        let h = safe_pwstr_to_hstring(value);
        Self { h }
    }
}

impl From<HSTRINGWrap> for HSTRING {
    fn from(val: HSTRINGWrap) -> Self {
        val.h
    }
}

impl From<&IFabricStringResult> for HSTRINGWrap {
    fn from(value: &IFabricStringResult) -> Self {
        let content = unsafe { value.get_String() };
        let h = safe_pwstr_to_hstring(content);
        Self { h }
    }
}

impl From<HSTRINGWrap> for IFabricStringResult {
    fn from(value: HSTRINGWrap) -> Self {
        StringResult::new(value.h).into()
    }
}

// note that hstring must be valid for pcwstr lifetime
pub fn get_pcwstr_from_opt(opt: &Option<HSTRING>) -> PCWSTR {
    match opt {
        Some(x) => PCWSTR(x.as_ptr()),
        None => PCWSTR::null(),
    }
}

#[cfg(test)]
mod test {
    use crate::strings::HSTRINGWrap;

    use super::StringResult;
    use crate::HSTRING;
    use mssf_com::FabricCommon::IFabricStringResult;

    #[test]
    fn test_str_addr() {
        // Test the addr returned to SF is right.
        let addr = "1.2.3.4:1234";

        // Check hstring len.
        let haddr = HSTRING::from(addr);
        let haddr_slice = haddr.as_wide();
        assert_eq!(haddr_slice.len(), 12);

        // check StringResult len.
        let com_addr: IFabricStringResult = StringResult::new(haddr.clone()).into();
        let raw = unsafe { com_addr.get_String() };
        let slice = unsafe { raw.as_wide() };
        assert_eq!(slice.len(), 12);

        // check StringResult conversion is right
        let haddr2: HSTRING = HSTRINGWrap::from(&com_addr).into();
        assert_eq!(haddr, haddr2);
    }
}

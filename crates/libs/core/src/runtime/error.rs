// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use crate::{HRESULT, WString};

// Fills the error info as string for better debugging.
// SF has separate last error set and get from windows.
// Not all error strings are set by SF. This is not very useful in practice.
pub fn fill_fabric_hresult(code: HRESULT) -> crate::WinError {
    // in rs, this function always succeed. The fail case is that the return ptr is null.
    let sf_err = crate::API_TABLE.fabric_get_last_error_message().unwrap();
    let err_str_raw = unsafe { sf_err.get_String() };
    let err_str = if err_str_raw.is_null() {
        &[]
    } else {
        unsafe { err_str_raw.as_wide() }
    };
    println!("debug std: {}", WString::from_wide(err_str));
    crate::WinError::new(code, WString::from_wide(err_str).to_string())
}

pub fn fill_fabric_error(e: crate::WinError) -> crate::WinError {
    fill_fabric_hresult(e.code())
}

#[cfg(test)]
#[cfg(windows)] // linux error propagate is not working yet
mod test {
    use crate::{WString, WinError};
    use mssf_com::FabricTypes::FABRIC_E_GATEWAY_NOT_REACHABLE;

    #[test]
    fn test_win_error() {
        let s = WString::from("MyError");
        let e = WinError::new(
            crate::HRESULT(FABRIC_E_GATEWAY_NOT_REACHABLE.0),
            s.clone().to_string(),
        );
        assert_eq!(e.message(), s.to_string_lossy());
    }
}

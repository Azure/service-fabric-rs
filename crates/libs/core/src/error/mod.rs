// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use super::HRESULT;
use mssf_com::FabricTypes::{
    FABRIC_ERROR_CODE, FABRIC_E_OPERATION_NOT_COMPLETE, FABRIC_E_OPERATION_NOT_SUPPORTED,
};
use windows::Win32::Foundation::{
    E_ABORT, E_ACCESSDENIED, E_FAIL, E_INVALIDARG, E_NOTIMPL, E_OUTOFMEMORY, E_POINTER, S_OK,
};

/// Make passing error code to SF api easier.
/// Provides conversion from windows errors or fabric error code
/// to windows_core::Error.
#[derive(Debug, Clone)]
pub struct FabricError(super::HRESULT);

impl FabricError {
    pub fn new(code: HRESULT) -> Self {
        Self(code)
    }
}

impl From<HRESULT> for FabricError {
    fn from(value: HRESULT) -> Self {
        Self::new(value)
    }
}

impl From<FABRIC_ERROR_CODE> for FabricError {
    fn from(value: FABRIC_ERROR_CODE) -> Self {
        Self::new(HRESULT(value.0))
    }
}

impl From<FabricError> for super::Error {
    fn from(val: FabricError) -> Self {
        super::Error::from_hresult(val.0)
    }
}

impl From<FabricError> for HRESULT {
    fn from(value: FabricError) -> Self {
        value.0
    }
}

/// SF uses win32 hresult code together with the fabric error code.
/// See: https://github.com/microsoft/service-fabric/blob/master/src/prod/src/Common/ErrorCodeValue.h
/// We provide the common win32 hresult code that SF uses. They are helpful
/// when returning from Rust back into SF com api.
pub enum FabricErrorCode {
    Success = S_OK.0 as isize,
    InvalidArgument = E_INVALIDARG.0 as isize,
    AccessDenied = E_ACCESSDENIED.0 as isize,
    ArgumentNull = E_POINTER.0 as isize,
    OperationCanceled = E_ABORT.0 as isize,
    OperationFailed = E_FAIL.0 as isize,
    OutOfMemory = E_OUTOFMEMORY.0 as isize,
    NotImplemented = E_NOTIMPL.0 as isize,
    // Some common errors from raw fabric code
    AsyncOperationNotComplete = FABRIC_E_OPERATION_NOT_COMPLETE.0 as isize,
    OperationNotSupported = FABRIC_E_OPERATION_NOT_SUPPORTED.0 as isize, // TODO: maybe all fabric error constants should be defined here as well in future.
}

impl From<FabricErrorCode> for FabricError {
    fn from(value: FabricErrorCode) -> Self {
        FabricError(HRESULT(value as i32))
    }
}

// other conversions goes through FabricError
impl From<FabricErrorCode> for HRESULT {
    fn from(value: FabricErrorCode) -> Self {
        FabricError::from(value).into()
    }
}

impl From<FabricErrorCode> for super::Error {
    fn from(value: FabricErrorCode) -> Self {
        FabricError::from(value).into()
    }
}

#[cfg(test)]
mod test {

    use super::{FabricError, FabricErrorCode};
    use mssf_com::FabricTypes::FABRIC_E_CODE_PACKAGE_NOT_FOUND;
    use windows::Win32::Foundation::{E_ACCESSDENIED, E_POINTER};
    use windows_core::{Error, HRESULT};

    #[test]
    fn test_fabric_error() {
        let fe = FabricError::from(FABRIC_E_CODE_PACKAGE_NOT_FOUND);
        let e = crate::Error::from(fe.clone());
        assert_eq!(e.code(), fe.into());
    }

    #[test]
    fn test_hresult_error() {
        let err1: HRESULT = FabricError::from(FabricErrorCode::AccessDenied).into();
        let err2 = E_ACCESSDENIED;
        assert_eq!(err1, err2);

        let e: Error = FabricErrorCode::ArgumentNull.into();
        assert_eq!(e, E_POINTER.into());
    }
}

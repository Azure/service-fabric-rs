// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use crate::HRESULT;
use mssf_com::FabricTypes::FABRIC_ERROR_CODE;

mod errorcode;
pub use errorcode::FabricErrorCode;

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

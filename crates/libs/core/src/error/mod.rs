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
#[derive(Clone)]
pub struct FabricError(super::HRESULT);

impl FabricError {
    pub fn new(code: HRESULT) -> Self {
        Self(code)
    }

    /// Convert to fabric error code if possible.
    pub fn try_as_fabric_error_code(&self) -> Result<FabricErrorCode, &str> {
        FabricErrorCode::try_from(FABRIC_ERROR_CODE(self.0 .0))
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

impl From<crate::Error> for FabricError {
    fn from(error: crate::Error) -> Self {
        Self(error.into())
    }
}

impl core::fmt::Debug for FabricError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut debug = fmt.debug_struct("FabricError");
        let str_code = match FabricErrorCode::try_from(FABRIC_ERROR_CODE(self.0 .0)) {
            Ok(c) => Some(c),
            Err(_) => None,
        };
        debug.field("code", &self.0 .0);
        match str_code {
            Some(c) => debug.field("message", &c),
            None => debug.field("message", &"unknown fabric error"),
        };

        debug.finish()
    }
}

impl core::fmt::Display for FabricError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let str_code = match FabricErrorCode::try_from(FABRIC_ERROR_CODE(self.0 .0)) {
            Ok(c) => Some(c),
            Err(_) => None,
        };
        match str_code {
            Some(c) => core::write!(fmt, "{} ({})", c, self.0 .0),
            None => core::write!(fmt, "{}", self.0 .0),
        }
    }
}

#[cfg(test)]
mod test {

    use super::{FabricError, FabricErrorCode};
    use mssf_com::FabricTypes::FABRIC_E_CODE_PACKAGE_NOT_FOUND;
    use windows_core::Win32::Foundation::{E_ACCESSDENIED, E_POINTER};
    use windows_core::{Error, HRESULT};

    #[test]
    fn test_fabric_error() {
        let fe = FabricError::from(FABRIC_E_CODE_PACKAGE_NOT_FOUND);
        // check debug string
        assert_eq!(
            format!("{:?}", fe),
            "FabricError { code: -2147017733, message: FABRIC_E_CODE_PACKAGE_NOT_FOUND }"
        );
        // check display string
        assert_eq!(
            format!("{}", fe),
            "FABRIC_E_CODE_PACKAGE_NOT_FOUND (-2147017733)"
        );
        let e = crate::Error::from(fe.clone());
        assert_eq!(e.code(), fe.into());
        let ec = FabricError::from(e)
            .try_as_fabric_error_code()
            .expect("unknown code");
        assert_eq!(ec, FabricErrorCode::FABRIC_E_CODE_PACKAGE_NOT_FOUND);
    }

    #[test]
    fn test_hresult_error() {
        let err1: HRESULT = FabricError::from(FabricErrorCode::E_ACCESSDENIED).into();
        let err2 = E_ACCESSDENIED;
        assert_eq!(err1, err2);

        let e: Error = FabricErrorCode::E_POINTER.into();
        assert_eq!(e, E_POINTER.into());

        const SEC_E_INTERNAL_ERROR: crate::HRESULT = crate::HRESULT(0x80090304_u32 as _);
        // use an error that is not fabric error
        let fe = FabricError::from(SEC_E_INTERNAL_ERROR);
        // check display string
        assert_eq!(format!("{}", fe), "-2146893052");
        assert_eq!(
            format!("{:?}", fe),
            "FabricError { code: -2146893052, message: \"unknown fabric error\" }"
        );
    }
}

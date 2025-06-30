// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use crate::HRESULT;
use mssf_com::FabricTypes::FABRIC_ERROR_CODE;

mod errorcode;
pub use errorcode::ErrorCode;

/// Result containing mssf Error.
pub type Result<T> = core::result::Result<T, Error>;

/// Make passing error code to SF api easier.
/// Provides conversion from windows errors or fabric error code
/// to windows_core::Error.
/// All safe code uses this Error, and bridge and proxy code needs to
/// convert this Error into/from WinError.
#[derive(Clone, PartialEq)]
pub struct Error(pub super::HRESULT);

impl Error {
    pub fn new(code: HRESULT) -> Self {
        Self(code)
    }

    /// Convert to fabric error code if possible.
    pub fn try_as_fabric_error_code(&self) -> std::result::Result<ErrorCode, &str> {
        ErrorCode::try_from(FABRIC_ERROR_CODE(self.0.0))
    }
}

impl From<HRESULT> for Error {
    fn from(value: HRESULT) -> Self {
        Self::new(value)
    }
}

impl From<FABRIC_ERROR_CODE> for Error {
    fn from(value: FABRIC_ERROR_CODE) -> Self {
        Self::new(HRESULT(value.0))
    }
}

impl From<Error> for super::WinError {
    fn from(val: Error) -> Self {
        super::WinError::from_hresult(val.0)
    }
}

impl From<Error> for HRESULT {
    fn from(value: Error) -> Self {
        value.0
    }
}

impl From<crate::WinError> for Error {
    fn from(error: crate::WinError) -> Self {
        Self(error.into())
    }
}

impl core::fmt::Debug for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut debug = fmt.debug_struct("FabricError");
        let str_code = ErrorCode::try_from(FABRIC_ERROR_CODE(self.0.0)).ok();
        debug.field("code", &self.0.0);
        match str_code {
            Some(c) => debug.field("message", &c),
            None => debug.field("message", &"unknown fabric error"),
        };

        debug.finish()
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let str_code = ErrorCode::try_from(FABRIC_ERROR_CODE(self.0.0)).ok();
        match str_code {
            Some(c) => core::write!(fmt, "{} ({})", c, self.0.0),
            None => core::write!(fmt, "{}", self.0.0),
        }
    }
}

impl std::error::Error for Error {}

// conversion from common error types in std

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        // Use the windows implementation to convert
        crate::WinError::from(value).into()
    }
}

impl From<Error> for std::io::Error {
    fn from(value: Error) -> Self {
        // Use windows implementation
        Self::from(crate::WinError::from(value))
    }
}

impl From<core::num::TryFromIntError> for Error {
    fn from(value: core::num::TryFromIntError) -> Self {
        // Use windows implementation
        crate::WinError::from(value).into()
    }
}

#[cfg(test)]
mod test {

    use super::{Error, ErrorCode};
    use crate::HRESULT;
    use mssf_com::FabricTypes::FABRIC_E_CODE_PACKAGE_NOT_FOUND;
    use windows_core::Win32::Foundation::{E_ACCESSDENIED, E_POINTER};

    #[test]
    fn test_fabric_error() {
        let fe = Error::from(FABRIC_E_CODE_PACKAGE_NOT_FOUND);
        // check debug string
        assert_eq!(
            format!("{fe:?}"),
            "FabricError { code: -2147017733, message: FABRIC_E_CODE_PACKAGE_NOT_FOUND }"
        );
        // check display string
        assert_eq!(
            format!("{fe}"),
            "FABRIC_E_CODE_PACKAGE_NOT_FOUND (-2147017733)"
        );
        let e = crate::WinError::from(fe.clone());
        assert_eq!(e.code(), fe.into());
        let ec = Error::from(e)
            .try_as_fabric_error_code()
            .expect("unknown code");
        assert_eq!(ec, ErrorCode::FABRIC_E_CODE_PACKAGE_NOT_FOUND);
    }

    #[test]
    fn test_hresult_error() {
        let err1: HRESULT = Error::from(ErrorCode::E_ACCESSDENIED).into();
        let err2 = E_ACCESSDENIED;
        assert_eq!(err1, err2);

        let e: crate::WinError = ErrorCode::E_POINTER.into();
        assert_eq!(e, E_POINTER.into());

        const SEC_E_INTERNAL_ERROR: crate::HRESULT = crate::HRESULT(0x80090304_u32 as _);
        // use an error that is not fabric error
        let fe = Error::from(SEC_E_INTERNAL_ERROR);
        // check display string
        assert_eq!(format!("{fe}"), "-2146893052");
        assert_eq!(
            format!("{fe:?}"),
            "FabricError { code: -2146893052, message: \"unknown fabric error\" }"
        );
    }
}

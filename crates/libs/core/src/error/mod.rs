// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use crate::HRESULT;
use mssf_com::FabricTypes::FABRIC_ERROR_CODE;

mod errorcode;
pub use errorcode::ErrorCode;
use windows_core::WString;

/// Result containing mssf Error.
pub type Result<T> = core::result::Result<T, Error>;

/// Make passing error code to SF api easier.
/// Provides conversion from windows errors or fabric error code
/// to windows_core::Error.
/// All safe code uses this Error, and bridge and proxy code needs to
/// convert this Error into/from WinError.
#[derive(Clone, PartialEq)]
pub struct Error {
    code: super::HRESULT,
    msg: Option<WString>,
}

impl Error {
    pub fn new(code: HRESULT, msg: Option<WString>) -> Self {
        Self { code, msg }
    }

    /// Create error from HRESULT code only, with no message.
    pub fn from_hresult(code: HRESULT) -> Self {
        Self::new(code, None)
    }

    /// Convert to fabric error code if possible.
    pub fn try_as_fabric_error_code(&self) -> std::result::Result<ErrorCode, &str> {
        ErrorCode::try_from(FABRIC_ERROR_CODE(self.code.0))
    }

    /// Create error from current thread last error code and message.
    pub fn from_thread(code: HRESULT) -> Self {
        let msg = get_last_error_message();
        Self::new(code, msg)
    }

    pub fn code(&self) -> HRESULT {
        self.code
    }
}

impl From<HRESULT> for Error {
    fn from(value: HRESULT) -> Self {
        Self::from_hresult(value)
    }
}

impl From<FABRIC_ERROR_CODE> for Error {
    fn from(value: FABRIC_ERROR_CODE) -> Self {
        Self::from_hresult(HRESULT(value.0))
    }
}

impl From<Error> for super::WinError {
    fn from(val: Error) -> Self {
        super::WinError::from_hresult(val.code)
    }
}

impl From<Error> for HRESULT {
    fn from(value: Error) -> Self {
        value.code
    }
}

impl From<crate::WinError> for Error {
    fn from(error: crate::WinError) -> Self {
        Self::from_thread(error.code())
    }
}

impl core::fmt::Debug for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut debug = fmt.debug_struct("FabricError");
        let code_str = ErrorCode::try_from(FABRIC_ERROR_CODE(self.code.0)).ok();
        debug.field("code", &self.code.0);
        match code_str {
            Some(c) => debug.field("code_str", &c),
            None => debug.field("code_str", &"unknown fabric error"),
        };
        match &self.msg {
            Some(m) => debug.field("message", m),
            None => debug.field("message", &"none"),
        };
        debug.finish()
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let str_code = ErrorCode::try_from(FABRIC_ERROR_CODE(self.code.0)).ok();
        match str_code {
            Some(c) => core::write!(fmt, "{} ({})", c, self.code.0),
            None => core::write!(fmt, "{}", self.code.0),
        }?;
        match &self.msg {
            Some(m) => core::write!(fmt, ": {m}"),
            None => Ok(()),
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

// Get last error message from current thread from SF.
// Call this after an SF API call failure.
fn get_last_error_message() -> Option<WString> {
    // The call returns error only when input COM holder ptr is null:
    // https://github.com/microsoft/service-fabric/blob/ddfed33de371857f8fdb92287a8e0497297c8ccf/src/prod/src/retail/native/FabricCommon/FabricCommon.cpp#L233
    // Our COM layer always provides a valid pointer, so we can safely ignore here.
    let msg = crate::api::API_TABLE.fabric_get_last_error_message().ok()?;
    // If message is not set, the returned string is be empty c string from cpp side.
    // We convert null or empty string to None, and non-empty string to Some.
    let smsg = crate::strings::StringResult::from(&msg).into_inner();
    // Intetional check len instead of is_empty because WString can be Nul and not empty.
    #[allow(clippy::len_zero)]
    match smsg.len() == 0 {
        true => None,
        false => Some(smsg),
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
            "FabricError { code: -2147017733, code_str: FABRIC_E_CODE_PACKAGE_NOT_FOUND, message: \"none\" }"
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
            "FabricError { code: -2146893052, code_str: \"unknown fabric error\", message: \"none\" }"
        );
    }

    #[test]
    fn test_error_from_thread() {
        // Call an obvious failing API to get error from thread.
        // In this call SF does not set the last error message.
        let err = crate::runtime::create_com_runtime().unwrap_err();
        let err_thread = Error::from_thread(err.code());
        assert_eq!(
            err_thread.code(),
            ErrorCode::FABRIC_INTERNAL_E_CANNOT_CONNECT.into()
        );
        assert_eq!(
            format!("{err_thread}"),
            "FABRIC_INTERNAL_E_CANNOT_CONNECT (-2147017536)"
        );
    }
}

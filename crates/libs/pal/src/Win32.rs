// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

pub mod Foundation {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct FILETIME {
        pub dwLowDateTime: u32,
        pub dwHighDateTime: u32,
    }
    impl windows_core::TypeKind for FILETIME {
        type TypeKind = windows_core::CopyType;
    }

    // Error codes used by SF from windows.
    pub const S_OK: windows_core::HRESULT = windows_core::HRESULT(0x0_u32 as _);
    pub const E_ABORT: windows_core::HRESULT = windows_core::HRESULT(0x80004004_u32 as _);
    pub const E_ACCESSDENIED: windows_core::HRESULT = windows_core::HRESULT(0x80070005_u32 as _);
    pub const E_FAIL: windows_core::HRESULT = windows_core::HRESULT(0x80004005_u32 as _);
    pub const E_HANDLE: windows_core::HRESULT = windows_core::HRESULT(0x80070006_u32 as _);
    pub const E_INVALIDARG: windows_core::HRESULT = windows_core::HRESULT(0x80070057_u32 as _);
    pub const E_NOINTERFACE: windows_core::HRESULT = windows_core::HRESULT(0x80004002_u32 as _);
    pub const E_NOTIMPL: windows_core::HRESULT = windows_core::HRESULT(0x80004001_u32 as _);
    pub const E_OUTOFMEMORY: windows_core::HRESULT = windows_core::HRESULT(0x8007000E_u32 as _);
    pub const E_POINTER: windows_core::HRESULT = windows_core::HRESULT(0x80004003_u32 as _);
    pub const E_UNEXPECTED: windows_core::HRESULT = windows_core::HRESULT(0x8000FFFF_u32 as _);

    pub struct WIN32_ERROR(pub u32);
    impl WIN32_ERROR {
        /// Maps a Win32 error code to an HRESULT value. Mirrors
        /// `windows_result::WIN32_ERROR::to_hresult` (which the flat windows-core
        /// exposes but is not usable on linux) so mssf error mapping keeps working.
        pub const fn to_hresult(self) -> windows_core::HRESULT {
            windows_core::HRESULT(if self.0 as i32 <= 0 {
                self.0
            } else {
                (self.0 & 0x0000_FFFF) | (7 << 16) | 0x8000_0000
            } as i32)
        }
    }
    pub const ERROR_FILE_EXISTS: WIN32_ERROR = WIN32_ERROR(80u32);
    pub const ERROR_DIR_NOT_EMPTY: WIN32_ERROR = WIN32_ERROR(145u32);
    pub const ERROR_NOT_FOUND: WIN32_ERROR = WIN32_ERROR(1168u32);
}

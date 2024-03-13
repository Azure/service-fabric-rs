// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

#![allow(non_camel_case_types, non_snake_case, dead_code)]
use std::ffi::c_void;

use libc::{__errno_location, malloc};
use windows::core::imp::LOAD_LIBRARY_FLAGS;
use windows::Win32::Foundation::{ERROR_NOT_ENOUGH_MEMORY, STATUS_HEAP_CORRUPTION};
use windows::{
    core::{HRESULT, PCSTR, PWSTR},
    Win32::Foundation::{HANDLE, HMODULE},
};

static DUMMY_HEAP: isize = 0x01020304;

/// # Safety
///
/// safe
#[no_mangle]
pub unsafe extern "system" fn GetLastError() -> u32 {
    let pe = __errno_location();
    if !pe.is_null() {
        *pe as u32
    } else {
        0
    }
}

/// # Safety
///
/// safe
#[no_mangle]
pub unsafe extern "system" fn SetLastError(dwerrcode: u32) {
    let pe = __errno_location();
    if !pe.is_null() {
        *pe = dwerrcode as i32
    }
}

/// # Safety
///
/// safe
#[no_mangle]
pub unsafe extern "system" fn GetProcessHeap() -> isize {
    DUMMY_HEAP
}

/// # Safety
///
/// safe
#[no_mangle]
pub unsafe extern "system" fn HeapAlloc(heap: isize, _flags: u32, len: usize) -> *mut c_void {
    if heap != DUMMY_HEAP {
        return std::ptr::null_mut();
    }

    let p = malloc(len);
    if p.is_null() {
        SetLastError(ERROR_NOT_ENOUGH_MEMORY.0)
    }
    p
}

/// # Safety
///
/// safe
#[no_mangle]
pub unsafe extern "system" fn HeapFree(heap: isize, _flags: u32, ptr: *const c_void) -> i32 {
    if heap != DUMMY_HEAP {
        SetLastError(STATUS_HEAP_CORRUPTION.0 as u32);
        return 0; // fail to free
    }

    libc::free(ptr as *mut c_void);
    1 // success
}

/// # Safety
///
/// safe
#[no_mangle]
pub unsafe extern "system" fn GetErrorInfo(_reserved: u32, _info: *mut *mut c_void) -> HRESULT {
    HRESULT(0)
}

/// # Safety
///
/// safe
#[no_mangle]
pub unsafe extern "system" fn SetErrorInfo(_reserved: u32, _info: *const c_void) -> HRESULT {
    HRESULT(0)
}

/// # Safety
///
/// safe
#[no_mangle]
pub unsafe extern "system" fn LoadLibraryA(_name: PCSTR) -> isize {
    0
}

/// # Safety
///
/// safe
#[no_mangle]
pub unsafe extern "system" fn LoadLibraryExA(
    _lplibfilename: PCSTR,
    _hfile: HANDLE,
    _dwflags: LOAD_LIBRARY_FLAGS,
) -> HMODULE {
    windows::Win32::Foundation::HMODULE(0)
}

/// # Safety
///
/// safe
#[no_mangle]
pub unsafe extern "system" fn FreeLibrary(_library: isize) -> i32 {
    0
}

/// # Safety
///
/// safe
#[no_mangle]
pub unsafe extern "system" fn GetProcAddress(_library: isize, _name: PCSTR) -> *const c_void {
    std::ptr::null()
}

/// # Safety
///
/// safe
#[no_mangle]
pub unsafe extern "system" fn SysFreeString(_bstr: *const u16) {}

/// # Safety
///
/// safe
#[no_mangle]
pub unsafe extern "system" fn SysStringLen(_bstr: *const u16) -> u32 {
    0
}

/// # Safety
///
/// safe
#[no_mangle]
pub unsafe extern "system" fn FormatMessageW(
    _flags: u32,
    _source: *const c_void,
    _code: u32,
    _lang: u32,
    _buffer: PWSTR,
    _len: u32,
    _args: *const *const i8,
) -> u32 {
    0
}

#[allow(non_camel_case_types, non_snake_case, dead_code)]
use std::ffi::c_void;

use windows::core::imp::LOAD_LIBRARY_FLAGS;
use windows::{
    core::{HRESULT, PCSTR, PWSTR},
    Win32::Foundation::{HANDLE, HMODULE},
};

#[no_mangle]
pub unsafe extern "system" fn GetLastError() -> u32 {
    0
}

#[no_mangle]
pub unsafe extern "system" fn GetProcessHeap() -> isize {
    0
}

#[no_mangle]
pub unsafe extern "system" fn HeapAlloc(_heap: isize, _flags: u32, _len: usize) -> *mut c_void {
    std::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "system" fn HeapFree(_heap: isize, _flags: u32, _ptr: *const c_void) -> i32 {
    0
}

#[no_mangle]
pub unsafe extern "system" fn GetErrorInfo(_reserved: u32, _info: *mut *mut c_void) -> HRESULT {
    HRESULT(0)
}

#[no_mangle]
pub unsafe extern "system" fn SetErrorInfo(_reserved: u32, _info: *const c_void) -> HRESULT {
    HRESULT(0)
}

#[no_mangle]
pub unsafe extern "system" fn LoadLibraryA(_name: PCSTR) -> isize {
    0
}

#[no_mangle]
pub unsafe extern "system" fn LoadLibraryExA(
    _lplibfilename: PCSTR,
    _hfile: HANDLE,
    _dwflags: LOAD_LIBRARY_FLAGS,
) -> HMODULE {
    windows::Win32::Foundation::HMODULE(0)
}

#[no_mangle]
pub unsafe extern "system" fn FreeLibrary(_library: isize) -> i32 {
    0
}

#[no_mangle]
pub unsafe extern "system" fn GetProcAddress(_library: isize, _name: PCSTR) -> *const c_void {
    std::ptr::null()
}

#[no_mangle]
pub unsafe extern "system" fn SysFreeString(_bstr: *const u16) {}

#[no_mangle]
pub unsafe extern "system" fn SysStringLen(_bstr: *const u16) -> u32 {
    0
}

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

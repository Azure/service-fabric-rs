// ------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
// Licensed under the MIT License (MIT). See License.txt in the repo root for license information.
// ------------------------------------------------------------

use std::fmt::Write;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PCWSTR(pub *const u16);

impl AsRef<PCWSTR> for PCWSTR {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl windows_core::TypeKind for PCWSTR {
    type TypeKind = windows_core::CopyType;
}

// Copied minimal impl from windows_core crate which is not available on linux.
// This is used on windows as well instead of the original defs if you use mssf-pal.
impl PCWSTR {
    /// Construct a new `PCWSTR` from a raw pointer
    pub const fn from_raw(ptr: *const u16) -> Self {
        Self(ptr)
    }

    /// Construct a null `PCWSTR`
    pub const fn null() -> Self {
        Self(core::ptr::null())
    }

    /// Returns a raw pointer to the `PCWSTR`
    pub const fn as_ptr(&self) -> *const u16 {
        self.0
    }

    /// Checks whether the `PCWSTR` is null
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    /// String length without the trailing 0
    ///
    /// # Safety
    ///
    /// The `PCWSTR`'s pointer needs to be valid for reads up until and including the next `\0`.
    pub unsafe fn len(&self) -> usize {
        let mut len = 0;
        let mut ptr = self.0;
        while ptr.read() != 0 {
            len += 1;
            ptr = ptr.add(1);
        }
        len
    }

    /// Returns `true` if the string length is zero, and `false` otherwise.
    ///
    /// # Safety
    ///
    /// The `PCWSTR`'s pointer needs to be valid for reads up until and including the next `\0`.
    pub unsafe fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// String data without the trailing 0
    ///
    /// # Safety
    ///
    /// The `PCWSTR`'s pointer needs to be valid for reads up until and including the next `\0`.
    pub unsafe fn as_wide(&self) -> &[u16] {
        core::slice::from_raw_parts(self.0, self.len())
    }
}

impl Default for PCWSTR {
    fn default() -> Self {
        Self::null()
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PCSTR(pub *const u8);

impl AsRef<PCSTR> for PCSTR {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl windows_core::TypeKind for PCSTR {
    type TypeKind = windows_core::CopyType;
}

/// WString is the utf16 string, similar to std::wstring in cpp.
/// It is used for passing utf16 string buffers between Rust and COM.
// The inner buffer is null terminated u16 vec.
#[derive(Clone, PartialEq, Eq, Default)]
pub struct WString(Option<Vec<u16>>);
const EMPTY: [u16; 1] = [0];

impl WString {
    /// creates an empty string
    pub const fn new() -> Self {
        Self(None)
    }

    /// returns if the string is empty
    pub const fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    /// len is the utf16 len not including the null terminator bytes
    pub fn len(&self) -> usize {
        match self.0.as_ref() {
            Some(v) => v.len() - 1,
            None => 0,
        }
    }

    /// Get the string as 16-bit wide characters (wchars).
    pub fn as_wide(&self) -> &[u16] {
        match self.0.as_ref() {
            Some(v) => {
                // remove the last null terminator
                v.as_slice().split_last().unwrap().1
            }
            None => &[],
        }
    }

    /// Get the contents of this `WString` as a String lossily.
    pub fn to_string_lossy(&self) -> String {
        String::from_utf16_lossy(self.as_wide())
    }

    /// Returns a raw pointer to the `WString` buffer.
    pub fn as_ptr(&self) -> *const u16 {
        match self.0.as_ref() {
            Some(v) => v.as_ptr(),
            None => EMPTY.as_ptr(),
        }
    }

    pub fn as_pcwstr(&self) -> PCWSTR {
        PCWSTR::from_raw(self.as_ptr())
    }

    /// From slice without the null terminator.
    pub fn from_wide(value: &[u16]) -> Self {
        // TODO: avoid the clone for the iter.
        unsafe { Self::from_wide_iter(value.iter().cloned(), value.len()) }
    }

    unsafe fn from_wide_iter<I: Iterator<Item = u16>>(iter: I, len: usize) -> Self {
        if len == 0 {
            return Self::new();
        }
        // append a null terminator. collect should allocate efficiently from iter.
        let iter = iter.chain(EMPTY.as_ref().iter().cloned());
        let v = iter.collect::<Vec<_>>();
        Self(Some(v))
    }
}

impl From<&str> for WString {
    fn from(value: &str) -> Self {
        unsafe { Self::from_wide_iter(value.encode_utf16(), value.len()) }
    }
}

impl From<String> for WString {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}
impl From<&String> for WString {
    fn from(value: &String) -> Self {
        value.as_str().into()
    }
}

impl From<&PCWSTR> for WString {
    /// Requires value points to valid memory location
    /// Null is ok.
    fn from(value: &PCWSTR) -> Self {
        if value.is_null() {
            Self::new()
        } else {
            Self::from_wide(unsafe { value.as_wide() })
        }
    }
}

impl From<PCWSTR> for WString {
    fn from(value: PCWSTR) -> Self {
        Self::from(&value)
    }
}

impl core::fmt::Display for WString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // convert u16 to char gracefully and write to formatter.
        let wit = core::char::decode_utf16(self.as_wide().iter().cloned());
        for c in wit {
            match c {
                Ok(c) => f.write_char(c)?,
                Err(_) => f.write_char(core::char::REPLACEMENT_CHARACTER)?,
            }
        }
        Ok(())
    }
}

impl core::fmt::Debug for WString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{self}\"")
    }
}

#[cfg(test)]
mod tests {
    use crate::PCWSTR;

    use super::WString;

    #[test]
    fn string_test() {
        let test_case = |s: &str| {
            let h = WString::from(s);
            assert_eq!(s.len(), h.len());
            assert_eq!(s.is_empty(), h.is_empty());
            assert_eq!(format!("{h}"), s);
            assert_eq!(s, h.to_string_lossy());
            assert_eq!(h.as_wide().len(), s.len());
            let raw = h.as_ptr();
            let h2 = WString::from(PCWSTR(raw));
            assert_eq!(s, h2.to_string_lossy());
            assert_eq!(h, h2);
            assert_ne!(h, WString::from("dummy"));
        };

        test_case("hello");
        test_case("s");
        test_case("");
    }
}

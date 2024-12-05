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

// copied from windows crate
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

// HSTRING implementation that is not compatible with winrt standard.
// This will be renamed to WString in a separate PR.
// The inner buffer is null terminated u16 vec.
#[derive(Clone, PartialEq, Eq, Default)]
pub struct HSTRING(Option<Vec<u16>>);

impl HSTRING {
    pub const fn new() -> Self {
        Self(None)
    }

    pub const fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub fn len(&self) -> usize {
        match self.0.as_ref() {
            Some(v) => v.len(),
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
        //unsafe { core::slice::from_raw_parts(self.as_ptr(), self.len()) }
    }

    /// Get the contents of this `HSTRING` as a String lossily.
    pub fn to_string_lossy(&self) -> String {
        String::from_utf16_lossy(self.as_wide())
    }

    /// Returns a raw pointer to the `HSTRING` buffer.
    pub fn as_ptr(&self) -> *const u16 {
        match self.0.as_ref() {
            Some(v) => v.as_ptr(),
            None => {
                const EMPTY: [u16; 1] = [0];
                EMPTY.as_ptr()
            }
        }
    }

    pub fn as_pcwstr(&self) -> PCWSTR {
        PCWSTR::from_raw(self.as_ptr())
    }

    pub fn from_wide(value: &[u16]) -> Self {
        unsafe { Self::from_wide_iter(value.iter().copied(), value.len()) }
    }

    unsafe fn from_wide_iter<I: Iterator<Item = u16>>(iter: I, len: usize) -> Self {
        if len == 0 {
            return Self::new();
        }
        // TODO: not efficient
        let mut v = iter.collect::<Vec<_>>();
        v.push(0); // null terminator
        Self(Some(v))
    }
}

impl From<&str> for HSTRING {
    fn from(value: &str) -> Self {
        unsafe { Self::from_wide_iter(value.encode_utf16(), value.len()) }
    }
}

impl From<String> for HSTRING {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}
impl From<&String> for HSTRING {
    fn from(value: &String) -> Self {
        value.as_str().into()
    }
}

impl core::fmt::Display for HSTRING {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            self.to_string_lossy() // not efficient
        )
    }
}

impl core::fmt::Debug for HSTRING {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

#[cfg(test)]
mod tests {
    use crate::PCWSTR;

    use super::HSTRING;

    #[test]
    fn string_test() {
        let s = "hello";
        let h = HSTRING::from(s);
        assert_eq!("hello", h.to_string_lossy());
        assert_eq!(h.as_wide().len(), s.len());
        let raw = h.as_ptr();
        let h2 = HSTRING::from_wide(unsafe { PCWSTR(raw).as_wide() });
        assert_eq!("hello", h2.to_string_lossy());
        assert_eq!(h, h2);
        assert_ne!(h, HSTRING::from("dummy"));
    }
}

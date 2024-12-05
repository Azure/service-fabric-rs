pub mod Foundation {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct BOOLEAN(pub u8);

    impl windows_core::TypeKind for BOOLEAN {
        type TypeKind = windows_core::CopyType;
    }

    #[repr(C)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct FILETIME {
        pub dwLowDateTime: u32,
        pub dwHighDateTime: u32,
    }
    impl windows_core::TypeKind for FILETIME {
        type TypeKind = windows_core::CopyType;
    }

    #[must_use]
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct BOOL(pub i32);
    impl windows_core::TypeKind for BOOL {
        type TypeKind = windows_core::CopyType;
    }
}

use std::{ffi::CString, sync::LazyLock};

pub(crate) static EMPTY_TWINE: LazyLock<CString> = LazyLock::new(|| CString::new("").unwrap());

pub(crate) fn string_to_cstring(string: String) -> CString {
    CString::new(string).expect("failed to convert String to CString")
}

#[allow(unused)]
pub(crate) fn cstring_to_string(cstring: CString) -> String {
    cstring
        .to_str()
        .expect("failed to convert CString to String")
        .to_string()
}

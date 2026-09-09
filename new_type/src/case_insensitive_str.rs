use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone)]
pub struct CaseInsensitiveString(String);

impl CaseInsensitiveString {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// Custom equality behavior
impl PartialEq for CaseInsensitiveString {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_ignore_ascii_case(&other.0)
    }
}

impl Eq for CaseInsensitiveString {}

// The trailing 0xff marks where this value ends -- without it, two of these in
// a row feed one unbroken run of bytes and ("ab", "c") hashes the same as
// ("a", "bc"). `str` uses the same byte for the same reason.
impl Hash for CaseInsensitiveString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for byte in self.0.bytes() {
            state.write_u8(byte.to_ascii_lowercase());
        }
        state.write_u8(0xff);
    }
}

impl fmt::Display for CaseInsensitiveString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Debug for CaseInsensitiveString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("CaseInsensitiveString")
            .field(&self.0)
            .finish()
    }
}

impl From<String> for CaseInsensitiveString {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for CaseInsensitiveString {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}
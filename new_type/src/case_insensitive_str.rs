use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct CaseInsensitiveString(String);

impl CaseInsensitiveString {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl PartialEq for CaseInsensitiveString {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_lowercase() == other.0.to_lowercase()
    }
}

impl Eq for CaseInsensitiveString {}

impl Hash for CaseInsensitiveString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // `String::hash` already writes the 0xff terminator.
        self.0.to_lowercase().hash(state);
    }
}

impl fmt::Display for CaseInsensitiveString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::hash::{Hash, Hasher};

    fn hash_of(value: &CaseInsensitiveString) -> u64 {
        let mut hasher = std::hash::DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn unicode_letters_are_equal_and_hash_the_same() {
        let upper = CaseInsensitiveString::from("Ä");
        let lower = CaseInsensitiveString::from("ä");
        assert_eq!(upper, lower);
        assert_eq!(hash_of(&upper), hash_of(&lower));

        let mut map = HashMap::new();
        map.insert(upper, "ok");
        assert_eq!(map[&lower], "ok");
    }

    #[test]
    fn sharp_s_does_not_match_ss() {
        let sharp_s = CaseInsensitiveString::from("ß");
        let ss = CaseInsensitiveString::from("SS");
        assert_ne!(sharp_s, ss);
    }
}
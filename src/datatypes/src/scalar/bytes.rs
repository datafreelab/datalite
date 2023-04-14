use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringBytes(bytes::Bytes);

impl StringBytes {
    pub fn as_utf8(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}

impl From<String> for StringBytes {
    fn from(string: String) -> StringBytes {
        StringBytes(bytes::Bytes::from(string))
    }
}

impl From<&str> for StringBytes {
    fn from(string: &str) -> StringBytes {
        StringBytes(bytes::Bytes::copy_from_slice(string.as_bytes()))
    }
}

impl PartialEq<String> for StringBytes {
    fn eq(&self, other: &String) -> bool {
        self.0 == other.as_bytes()
    }
}

impl PartialEq<StringBytes> for String {
    fn eq(&self, other: &StringBytes) -> bool {
        self.as_bytes() == other.0
    }
}

impl PartialEq<str> for StringBytes {
    fn eq(&self, other: &str) -> bool {
        self.0 == other.as_bytes()
    }
}

impl PartialEq<StringBytes> for str {
    fn eq(&self, other: &StringBytes) -> bool {
        self.as_bytes() == other.0
    }
}

impl Serialize for StringBytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        self.as_utf8().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for StringBytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(StringBytes::from(s))
    }
}


#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bytes(bytes::Bytes);
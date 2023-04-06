use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringValue(bytes::Bytes);

impl StringValue {
    pub fn as_utf8(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}

impl From<String> for StringValue {
    fn from(string: String) -> StringValue {
        StringValue(bytes::Bytes::from(string))
    }
}

impl From<&str> for StringValue {
    fn from(string: &str) -> StringValue {
        StringValue(bytes::Bytes::copy_from_slice(string.as_bytes()))
    }
}

impl PartialEq<String> for StringValue {
    fn eq(&self, other: &String) -> bool {
        self.0 == other.as_bytes()
    }
}

impl PartialEq<StringValue> for String {
    fn eq(&self, other: &StringValue) -> bool {
        self.as_bytes() == other.0
    }
}

impl PartialEq<str> for StringValue {
    fn eq(&self, other: &str) -> bool {
        self.0 == other.as_bytes()
    }
}

impl PartialEq<StringValue> for str {
    fn eq(&self, other: &StringValue) -> bool {
        self.as_bytes() == other.0
    }
}

impl Serialize for StringValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        self.as_utf8().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for StringValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(StringValue::from(s))
    }
}

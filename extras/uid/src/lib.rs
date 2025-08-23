use std::{fmt, time::SystemTime};

use serde::{Deserialize, Serialize, Serializer};

#[macro_export]
macro_rules! uid {
    ($uid:expr) => {{
        const OUTPUT: $crate::Uid = match $crate::Uid::try_parse($uid) {
            Ok(u) => u,
            Err(_) => panic!("invalid UID"),
        };
        OUTPUT
    }};
}

#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Uid(ulid::Ulid);

impl Uid {
    pub fn new() -> Self {
        Self(ulid::Ulid::new())
    }

    pub fn from_datetime(datetime: SystemTime) -> Self {
        Self(ulid::Ulid::from_datetime(datetime))
    }

    pub const fn nil() -> Self {
        Self(ulid::Ulid::nil())
    }

    pub const fn is_nil(&self) -> bool {
        self.0.is_nil()
    }

    pub const fn as_u128(&self) -> u128 {
        self.0.0
    }

    pub const fn try_parse(s: &str) -> Result<Uid, ulid::DecodeError> {
        match ulid::Ulid::from_string(s) {
            Ok(v) => Ok(Self(v)),
            Err(e) => Err(e),
        }
    }

    pub const fn try_parse_ascii(input: &[u8]) -> Result<Uid, ulid::DecodeError> {
        let s = match std::str::from_utf8(input) {
            Ok(s) => s,
            Err(_) => return Err(ulid::DecodeError::InvalidChar),
        };
        Self::try_parse(s)
    }

    pub fn into_string(self) -> String {
        self.0.to_string()
    }
}

impl fmt::Debug for Uid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Uid").field(&self.into_string()).finish()
    }
}

impl fmt::Display for Uid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.into_string())
    }
}

impl From<ulid::Ulid> for Uid {
    fn from(value: ulid::Ulid) -> Self {
        Self(value)
    }
}

impl From<Uid> for ulid::Ulid {
    fn from(value: Uid) -> Self {
        value.0
    }
}

impl Serialize for Uid {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.into_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Uid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Uid::try_parse(&s).map_err(serde::de::Error::custom)
    }
}

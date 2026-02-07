use std::{
    fmt::{ Debug, Display, Formatter, Write },
    hash::*,
    str::FromStr,
};
use rkyv::*;

#[derive(Archive, Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct ResourceId {
    namespace: String,
    path: String,
}

impl ResourceId {
    pub const DEFAULT_NAMESPACE: &'static str = "bevycraft";

    #[inline]
    pub unsafe fn new_unchecked(namespace: &str, path: &str) -> Self {
        Self {
            namespace: namespace.to_owned(),
            path: path.to_owned(),
        }
    }

    #[inline]
    fn new(namespace: &str, path: &str) -> Result<Self, ResourceIdError> {
        if Self::valid_namespace(namespace) && Self::valid_path(path) {
            Ok(unsafe { Self::new_unchecked(namespace, path) })
        } else { Err(ResourceIdError) }
    }

    #[inline]
    pub fn default_namespace(path: &str) -> Result<Self, ResourceIdError> {
        if Self::valid_path(path) {
            Ok(unsafe { Self::new_unchecked(Self::DEFAULT_NAMESPACE, path) })
        } else { Err(ResourceIdError) }
    }

    #[inline]
    pub fn custom_namespace(namespace: &str, path: &str) -> Result<Self, ResourceIdError> {
        Self::new(namespace, path)
    }

    #[inline]
    pub fn parse(location: &str) -> Result<Self, ResourceIdError> {
        match location.split_once(':') {
            None => Self::default_namespace(location),
            Some((namespace, path)) => Self::new(namespace, path),
        }
    }

    #[inline]
    pub unsafe fn parse_unchecked(location: &str) -> Self {
        match location.split_once(':') {
            None => Self::new_unchecked(&Self::DEFAULT_NAMESPACE, location),
            Some((n, p)) => Self::new_unchecked(n, p),
        }
    }

    #[inline]
    fn valid_namespace(s: &str) -> bool {
        s.as_bytes()
            .iter()
            .all(|&b| Self::valid_namespace_byte(b))
    }

    #[inline]
    fn valid_path(s: &str) -> bool {
        s.as_bytes()
            .iter()
            .all(|&b| Self::valid_path_byte(b))
    }

    #[inline]
    const fn valid_namespace_byte(byte: u8) -> bool {
        byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-'
    }

    #[inline]
    const fn valid_path_byte(byte: u8) -> bool {
        byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-' || byte == b'/'
    }
}

impl Display for ResourceId {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.namespace())?;
        f.write_char(':')?;
        f.write_str(self.path())
    }
}

impl From<String> for ResourceId {
    #[inline]
    fn from(value: String) -> Self {
        Self::parse(&value)
            .expect("Conversion From<String> to ResourceId failed!")
    }
}

impl FromStr for ResourceId {
    type Err = ResourceIdError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl NamespacedIdentifier for ResourceId {
    #[inline]
    fn namespace(&self) -> &str {
        self.namespace.as_str()
    }

    #[inline]
    fn path(&self) -> &str {
        self.path.as_str()
    }
}

pub trait NamespacedIdentifier {
    fn namespace(&self) -> &str;

    fn path(&self) -> &str;
}

pub struct ResourceIdError;

impl Debug for ResourceIdError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to validate ResourceId bytes.")
    }
}
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::mem::transmute;
use std::ops::Deref;
use std::slice::from_raw_parts;
use std::str::{from_utf8_unchecked, FromStr};
use std::sync::RwLock;
use bevy::platform::collections::HashSet;

static GLOBAL_INTERN: StringInterner = StringInterner::new();

/// # IString
/// A blazingly-fast [`String`]-like type with built-in interning for better memory efficiency.
/// Acts exactly like a [`String`] and allows to be cast back into it if needed.
/// # Example
/// ```
/// use bevycraft_core::prelude::IString;
///
/// let s1 = IString::from("We're the same!");
/// let s2 = IString::from("We're the same!");
///
/// // Same slice in memory and never deallocates once dropped
/// assert_eq!(s1.as_ptr(), s2.as_ptr());
/// ```
/// # Usage
/// The interned Strings are never freed, even if they're not being used, potentially a memory leak.
/// It will come to great use if there are Strings that need to stay alive for the rest of the
/// program. No Strings are reference-counted, as it would cause atomic overhead and also that
/// performance is top priority.
pub struct IString {
    inner: *const u8,
    len: usize,
}

impl IString {

    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const u8 {
        self.inner
    }
}

impl Clone for IString {

    #[inline]
    fn clone(&self) -> Self {
        Self { inner: self.inner, len: self.len }
    }
}

impl Display for IString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self)
    }
}

impl Debug for IString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IString")
            .field("value", &self.as_ref())
            .field("addr", &self.inner)
            .field("len", &self.len)
            .finish()
    }
}

impl Hash for IString {

    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.as_bytes());
    }
}

impl Ord for IString {

    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}

impl PartialOrd for IString {

    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

impl Eq for IString {}

impl PartialEq for IString {

    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl FromStr for IString {
    type Err = ();

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl From<String> for IString {

    #[inline]
    fn from(value: String) -> Self {
        Self { inner: GLOBAL_INTERN.get_or_intern(value.as_str()), len: value.len() }
    }
}

impl From<&str> for IString {

    #[inline]
    fn from(value: &str) -> Self {
        Self { inner: GLOBAL_INTERN.get_or_intern(value), len: value.len() }
    }
}

impl Into<String> for IString {

    #[inline]
    fn into(self) -> String {
        self.as_ref()
            .to_owned()
    }
}

impl<'a> Into<&'a str> for IString {

    #[inline]
    fn into(self) -> &'a str {
        unsafe { from_raw_bytes(self.inner, self.len) }
    }
}

impl AsRef<str> for IString {

    #[inline]
    fn as_ref(&self) -> &str {
        &*self
    }
}

impl Deref for IString {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { from_raw_bytes(self.inner, self.len) }
    }
}

struct StringInterner {
    entries: RwLock<HashSet<&'static str>>,
}

impl StringInterner {
    const fn new() -> Self {
        Self {
            entries: RwLock::new(HashSet::new())
        }
    }

    #[inline]
    fn get_or_intern(&self, string: &str) -> *const u8 {
        {
            let read = self.entries.read().unwrap();

            if let Some(entry) = read.get(string) {
                return unsafe { transmute(entry) }
            }
        }

        let mut write = self.entries.write().unwrap();

        let leaked: &'static str = Box::leak(Box::from(string));

        write.insert(leaked);

        leaked.as_ptr()
    }
}

#[inline(always)]
const unsafe fn from_raw_bytes<'a>(
    src: *const u8,
    len: usize,
) -> &'a str {
    unsafe {
        from_utf8_unchecked(
            from_raw_parts(src, len)
        )
    }
}
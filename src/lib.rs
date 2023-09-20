//! Parsing span information.

#![deny(missing_docs, unreachable_pub, unused_must_use)]
#![deny(clippy::all)]

use std::{fmt::Debug, marker::PhantomData, ops::Range};

use bytes::Bytes;

pub(crate) mod helpers;
pub mod http;
pub mod json;

/// A parsing error.
#[derive(Debug, thiserror::Error)]
#[error("parsing error: {0}")]
pub struct ParseError(String);

impl<R: pest::RuleType> From<pest::error::Error<R>> for ParseError {
    fn from(value: pest::error::Error<R>) -> Self {
        Self(value.to_string())
    }
}

impl From<std::str::Utf8Error> for ParseError {
    fn from(value: std::str::Utf8Error) -> Self {
        Self(value.to_string())
    }
}

/// A spanned value.
pub trait Spanned<T: ?Sized = [u8]> {
    /// Get a reference to the span of the value.
    fn span(&self) -> &Span<T>;
}

/// A span of a source string.
#[derive(PartialEq, Eq)]
pub struct Span<T: ?Sized = [u8]> {
    pub(crate) data: Bytes,
    pub(crate) range: Range<usize>,
    _pd: PhantomData<T>,
}

impl Clone for Span<[u8]> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            range: self.range.clone(),
            _pd: PhantomData,
        }
    }
}

impl Clone for Span<str> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            range: self.range.clone(),
            _pd: PhantomData,
        }
    }
}

impl Debug for Span<[u8]> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Span")
            .field("span", &self.as_bytes())
            .field("range", &self.range)
            .finish()
    }
}

impl Debug for Span<str> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Span")
            .field("span", &self.as_str())
            .field("range", &self.range)
            .finish()
    }
}

impl<T: ?Sized> Span<T> {
    /// Returns a reference to the source bytes.
    pub fn src(&self) -> &[u8] {
        self.data.as_ref()
    }

    /// Returns the corresponding range within the source string.
    pub fn range(&self) -> Range<usize> {
        self.range.clone()
    }

    /// Returns the length of the span in bytes.
    ///
    /// Just like `str::len()`, this is not necessarily the number of characters.
    pub fn len(&self) -> usize {
        self.range.len()
    }

    /// Returns `true` if the span is empty.
    pub fn is_empty(&self) -> bool {
        self.range.is_empty()
    }
}

impl Span<str> {
    /// Create a new string span.
    ///
    /// # Panics
    ///
    /// Panics if the given range is not within the source bytes, or
    /// if the span is not a valid UTF-8 string.
    pub(crate) fn new_str(src: Bytes, range: Range<usize>) -> Self {
        assert!(
            std::str::from_utf8(&src[range.clone()]).is_ok(),
            "span is not a valid UTF-8 string"
        );

        Self {
            data: src.slice(range.clone()),
            range,
            _pd: PhantomData,
        }
    }

    /// Create a new string span from a string slice.
    ///
    /// # Panics
    ///
    /// Panics if the given slice is not within the source bytes.
    pub(crate) fn new_from_str(src: Bytes, span: &str) -> Self {
        let range = helpers::get_span_range(src.as_ref(), span.as_bytes());

        Self {
            data: src.slice(range.clone()),
            range,
            _pd: PhantomData,
        }
    }

    /// Converts this type to a string slice.
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }

    /// Returns the corresponding byte span.
    pub fn to_byte_span(&self) -> Span<[u8]> {
        self.into()
    }
}

impl AsRef<str> for Span<str> {
    fn as_ref(&self) -> &str {
        // # Safety
        // The span is guaranteed to be a valid UTF-8 string because it is not
        // possible to create a `Span<str>` from a non-UTF-8 string.
        unsafe { std::str::from_utf8_unchecked(&self.data) }
    }
}

impl AsRef<[u8]> for Span<str> {
    fn as_ref(&self) -> &[u8] {
        self.data.as_ref()
    }
}

impl Span<[u8]> {
    /// Create a new byte span.
    ///
    /// # Panics
    ///
    /// Panics if the given range is not within the source bytes.
    pub(crate) fn new_bytes(src: Bytes, range: Range<usize>) -> Self {
        assert!(src.len() >= range.end, "span is not within source bytes");

        Self {
            data: src.slice(range.clone()),
            range,
            _pd: PhantomData,
        }
    }

    /// Converts this type to a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        self.as_ref()
    }
}

impl AsRef<[u8]> for Span<[u8]> {
    fn as_ref(&self) -> &[u8] {
        self.data.as_ref()
    }
}

impl From<Span<str>> for Span<[u8]> {
    fn from(span: Span<str>) -> Self {
        Self {
            data: span.data,
            range: span.range,
            _pd: PhantomData,
        }
    }
}

impl From<&Span<str>> for Span<[u8]> {
    fn from(span: &Span<str>) -> Self {
        Self {
            data: span.data.clone(),
            range: span.range.clone(),
            _pd: PhantomData,
        }
    }
}

impl PartialEq<Span> for [u8] {
    fn eq(&self, other: &Span) -> bool {
        self == other.as_ref()
    }
}

impl PartialEq<[u8]> for Span {
    fn eq(&self, other: &[u8]) -> bool {
        self.as_ref() == other
    }
}

impl PartialEq<&[u8]> for Span {
    fn eq(&self, other: &&[u8]) -> bool {
        self.as_ref() == *other
    }
}

impl PartialEq<[u8]> for &Span {
    fn eq(&self, other: &[u8]) -> bool {
        self.as_ref() == other
    }
}

impl PartialEq<Span<str>> for str {
    fn eq(&self, other: &Span<str>) -> bool {
        self == other.as_str()
    }
}

impl PartialEq<str> for Span<str> {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for Span<str> {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<str> for &Span<str> {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<Range<usize>> for Span<str> {
    fn eq(&self, other: &Range<usize>) -> bool {
        &self.range == other
    }
}

impl<T: ?Sized> PartialEq<Span<T>> for Range<usize> {
    fn eq(&self, other: &Span<T>) -> bool {
        self == &other.range
    }
}

impl<T: ?Sized> PartialEq<Range<usize>> for &Span<T> {
    fn eq(&self, other: &Range<usize>) -> bool {
        &self.range == other
    }
}

impl<T: ?Sized> PartialEq<Span<T>> for &Range<usize> {
    fn eq(&self, other: &Span<T>) -> bool {
        **self == other.range
    }
}

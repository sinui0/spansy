//! Parsing span information.

#![deny(missing_docs, unreachable_pub, unused_must_use)]
#![deny(clippy::all)]

use std::{fmt::Debug, ops::Range};

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
    fn span(&self) -> &Span<'_, T>;
}

/// A span of a source string.
#[derive(PartialEq, Eq)]
pub struct Span<'a, T: ?Sized = [u8]> {
    pub(crate) src: &'a [u8],
    pub(crate) span: &'a T,
    pub(crate) range: Range<usize>,
}

impl<T: Debug + ?Sized> Debug for Span<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Span")
            .field("span", &self.span)
            .field("range", &self.range)
            .finish()
    }
}

impl Clone for Span<'_, str> {
    fn clone(&self) -> Self {
        Self {
            src: self.src,
            span: self.span,
            range: self.range.clone(),
        }
    }
}

impl Clone for Span<'_, [u8]> {
    fn clone(&self) -> Self {
        Self {
            src: self.src,
            span: self.span,
            range: self.range.clone(),
        }
    }
}

impl<'a, T: ?Sized> Span<'a, T> {
    /// Returns a reference to the source string.
    pub fn src(&self) -> &[u8] {
        self.src
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

    /// Offset the span ranges by the given number of bytes.
    ///
    /// This is useful when the source string is a substring of a larger string.
    pub fn offset(&mut self, offset: usize) {
        self.range.start += offset;
        self.range.end += offset;
    }
}

impl<'a> Span<'a, str> {
    /// Returns a reference to the string span.
    pub fn span(&self) -> &str {
        self.span
    }

    /// Returns the corresponding byte span.
    pub fn to_byte_span(&self) -> Span<'a, [u8]> {
        self.clone().into()
    }
}

impl AsRef<str> for Span<'_, str> {
    fn as_ref(&self) -> &str {
        self.span
    }
}

impl<'a> Span<'a, [u8]> {
    /// Returns a reference to the byte span.
    pub fn span(&self) -> &[u8] {
        self.span
    }
}

impl AsRef<[u8]> for Span<'_, [u8]> {
    fn as_ref(&self) -> &[u8] {
        self.span
    }
}

impl<'a> From<Span<'a, str>> for Span<'a, [u8]> {
    fn from(span: Span<'a, str>) -> Self {
        let Span { src, span, range } = span;

        Self {
            src,
            span: span.as_bytes(),
            range,
        }
    }
}

impl PartialEq<Span<'_>> for [u8] {
    fn eq(&self, other: &Span<'_>) -> bool {
        self == other.span
    }
}

impl PartialEq<[u8]> for Span<'_> {
    fn eq(&self, other: &[u8]) -> bool {
        self.span == other
    }
}

impl PartialEq<&[u8]> for Span<'_> {
    fn eq(&self, other: &&[u8]) -> bool {
        self.span == *other
    }
}

impl PartialEq<[u8]> for &Span<'_> {
    fn eq(&self, other: &[u8]) -> bool {
        self.span == other
    }
}

impl PartialEq<Span<'_, str>> for str {
    fn eq(&self, other: &Span<'_, str>) -> bool {
        self == other.span
    }
}

impl PartialEq<str> for Span<'_, str> {
    fn eq(&self, other: &str) -> bool {
        self.span == other
    }
}

impl PartialEq<&str> for Span<'_, str> {
    fn eq(&self, other: &&str) -> bool {
        self.span == *other
    }
}

impl PartialEq<str> for &Span<'_, str> {
    fn eq(&self, other: &str) -> bool {
        self.span == other
    }
}

impl PartialEq<Range<usize>> for Span<'_, str> {
    fn eq(&self, other: &Range<usize>) -> bool {
        &self.range == other
    }
}

impl<T: ?Sized> PartialEq<Span<'_, T>> for Range<usize> {
    fn eq(&self, other: &Span<'_, T>) -> bool {
        self == &other.range
    }
}

impl<T: ?Sized> PartialEq<Range<usize>> for &Span<'_, T> {
    fn eq(&self, other: &Range<usize>) -> bool {
        &self.range == other
    }
}

impl<T: ?Sized> PartialEq<Span<'_, T>> for &Range<usize> {
    fn eq(&self, other: &Span<'_, T>) -> bool {
        **self == other.range
    }
}

//! Parsing span information.

#![deny(missing_docs, unreachable_pub, unused_must_use)]
#![deny(clippy::all)]

use std::ops::Range;

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

/// A spanned value.
pub trait Spanned {
    /// Get a reference to the span of the value.
    fn span(&self) -> &Span<'_>;
}

/// A span of a source string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span<'a> {
    src: &'a str,
    range: Range<usize>,
}

impl<'a> Span<'a> {
    /// Create a new span from a source string and a range.
    pub(crate) fn new(src: &'a str, range: Range<usize>) -> Self {
        Self { src, range }
    }

    /// Returns a reference to the string span.
    pub fn src(&self) -> &str {
        self.src
    }

    /// Returns the corresponding range within the source string.
    pub fn range(&self) -> Range<usize> {
        self.range.clone()
    }

    /// Returns the length of the span.
    pub fn len(&self) -> usize {
        self.range.len()
    }

    /// Returns `true` if the span is empty.
    pub fn is_empty(&self) -> bool {
        self.range.is_empty()
    }

    /// Offset the span ranges by the given amount.
    ///
    /// This is useful when the source string is a substring of a larger string.
    pub fn offset(&mut self, offset: usize) {
        self.range.start += offset;
        self.range.end += offset;
    }
}

impl PartialEq<Span<'_>> for str {
    fn eq(&self, other: &Span<'_>) -> bool {
        self == other.src
    }
}

impl PartialEq<str> for Span<'_> {
    fn eq(&self, other: &str) -> bool {
        self.src == other
    }
}

impl PartialEq<&str> for Span<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.src == *other
    }
}

impl PartialEq<str> for &Span<'_> {
    fn eq(&self, other: &str) -> bool {
        self.src == other
    }
}

impl PartialEq<Range<usize>> for Span<'_> {
    fn eq(&self, other: &Range<usize>) -> bool {
        &self.range == other
    }
}

impl PartialEq<Span<'_>> for Range<usize> {
    fn eq(&self, other: &Span<'_>) -> bool {
        self == &other.range
    }
}

impl PartialEq<Range<usize>> for &Span<'_> {
    fn eq(&self, other: &Range<usize>) -> bool {
        &self.range == other
    }
}

impl PartialEq<Span<'_>> for &Range<usize> {
    fn eq(&self, other: &Span<'_>) -> bool {
        **self == other.range
    }
}

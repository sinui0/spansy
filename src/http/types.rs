use utils::range::{RangeDifference, RangeSet};

use crate::{Span, Spanned};

/// An HTTP header name.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HeaderName(pub(crate) Span<str>);

impl HeaderName {
    /// Returns the header name as a string slice.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Shifts the span range by the given offset.
    pub fn offset(&mut self, offset: usize) {
        self.0.offset(offset);
    }
}

impl Spanned<str> for HeaderName {
    fn span(&self) -> &Span<str> {
        &self.0
    }
}

/// An HTTP header value.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HeaderValue(pub(crate) Span);

impl HeaderValue {
    /// Returns the header value as a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// Shifts the span range by the given offset.
    pub fn offset(&mut self, offset: usize) {
        self.0.offset(offset);
    }
}

impl Spanned for HeaderValue {
    fn span(&self) -> &Span {
        &self.0
    }
}

/// An HTTP header, including the trailing CRLF.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Header {
    pub(crate) span: Span,
    /// The header name.
    pub name: HeaderName,
    /// The header value.
    pub value: HeaderValue,
}

impl Header {
    /// Returns the indices of the header excluding the value.
    pub fn without_value(&self) -> RangeSet<usize> {
        self.span.indices.difference(&self.value.span().indices)
    }

    /// Shifts the span range by the given offset.
    pub fn offset(&mut self, offset: usize) {
        self.span.offset(offset);
        self.name.offset(offset);
        self.value.offset(offset);
    }
}

impl Spanned for Header {
    fn span(&self) -> &Span {
        &self.span
    }
}

/// An HTTP request line, including the trailing CRLF.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RequestLine {
    pub(crate) span: Span<str>,

    /// The request method.
    pub method: Span<str>,
    /// The request path.
    pub path: Span<str>,
}

impl RequestLine {
    /// Returns the indices of the request line excluding the path.
    pub fn without_path(&self) -> RangeSet<usize> {
        self.span.indices.difference(&self.path.indices)
    }

    /// Shifts the span range by the given offset.
    pub fn offset(&mut self, offset: usize) {
        self.span.offset(offset);
        self.method.offset(offset);
        self.path.offset(offset);
    }
}

impl Spanned<str> for RequestLine {
    fn span(&self) -> &Span<str> {
        &self.span
    }
}

/// An HTTP request.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Request {
    pub(crate) span: Span,
    /// The request line.
    pub request: RequestLine,
    /// Request headers.
    pub headers: Vec<Header>,
    /// Request body.
    pub body: Option<Body>,
}

impl Request {
    /// Returns an iterator of request headers with the given name (case-insensitive).
    ///
    /// This method returns an iterator because it is valid for HTTP records to contain
    /// duplicate header names.
    pub fn headers_with_name<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a Header> {
        self.headers
            .iter()
            .filter(|h| h.name.0.as_str().eq_ignore_ascii_case(name))
    }

    /// Returns the indices of the request excluding the path, headers and body.
    pub fn without_data(&self) -> RangeSet<usize> {
        let mut indices = self.span.indices.difference(&self.request.path.indices);
        for header in &self.headers {
            indices = indices.difference(header.span.indices());
        }
        if let Some(body) = &self.body {
            indices = indices.difference(body.span.indices());
        }
        indices
    }

    /// Shifts the span range by the given offset.
    pub fn offset(&mut self, offset: usize) {
        self.span.offset(offset);
        self.request.offset(offset);
        for header in &mut self.headers {
            header.offset(offset);
        }
        if let Some(body) = &mut self.body {
            body.offset(offset);
        }
    }
}

impl Spanned for Request {
    fn span(&self) -> &Span {
        &self.span
    }
}

/// An HTTP response status.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Status {
    pub(crate) span: Span<str>,

    /// The response code.
    pub code: Span<str>,
    /// The reason phrase.
    pub reason: Span<str>,
}

impl Status {
    /// Shifts the span range by the given offset.
    pub fn offset(&mut self, offset: usize) {
        self.span.offset(offset);
        self.code.offset(offset);
        self.reason.offset(offset);
    }
}

impl Spanned<str> for Status {
    fn span(&self) -> &Span<str> {
        &self.span
    }
}

/// An HTTP response.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Response {
    pub(crate) span: Span,
    /// The response status.
    pub status: Status,
    /// Response headers.
    pub headers: Vec<Header>,
    /// Response body.
    pub body: Option<Body>,
}

impl Response {
    /// Returns an iterator of response headers with the given name (case-insensitive).
    ///
    /// This method returns an iterator because it is valid for HTTP records to contain
    /// duplicate header names.
    pub fn headers_with_name<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a Header> {
        self.headers
            .iter()
            .filter(|h| h.name.0.as_str().eq_ignore_ascii_case(name))
    }

    /// Shifts the span range by the given offset.
    pub fn offset(&mut self, offset: usize) {
        self.span.offset(offset);
        self.status.offset(offset);
        for header in &mut self.headers {
            header.offset(offset);
        }
        if let Some(body) = &mut self.body {
            body.offset(offset);
        }
    }
}

impl Spanned for Response {
    fn span(&self) -> &Span {
        &self.span
    }
}

/// An HTTP request or response body.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Body {
    pub(crate) span: Span,
}

impl Body {
    /// Returns the body as a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        self.span.as_bytes()
    }

    /// Shifts the span range by the given offset.
    pub fn offset(&mut self, offset: usize) {
        self.span.offset(offset);
    }
}

impl Spanned for Body {
    fn span(&self) -> &Span {
        &self.span
    }
}

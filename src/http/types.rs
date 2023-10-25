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

/// An HTTP header.
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

/// An HTTP request.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Request {
    pub(crate) span: Span,
    /// The request method.
    pub method: Span<str>,
    /// The request path.
    pub path: Span<str>,
    /// Request headers.
    pub headers: Vec<Header>,
    /// Request body.
    pub body: Option<Body>,
}

impl Request {
    /// Returns all the request headers with the given name (case-insensitive). Returns `None` if
    /// there are no matching headers. Returns `Some` if there is at least one matching header.
    /// Never returns `Some` with an empty `Vec`.
    pub fn header(&self, name: &str) -> Option<Vec<&Header>> {
        let matching: Vec<&Header> = self
            .headers
            .iter()
            .filter(|h| h.name.0.as_str().eq_ignore_ascii_case(name))
            .collect();
        if matching.is_empty() {
            None
        } else {
            Some(matching)
        }
    }

    /// Shifts the span range by the given offset.
    pub fn offset(&mut self, offset: usize) {
        self.span.offset(offset);
        self.method.offset(offset);
        self.path.offset(offset);
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

/// An HTTP response.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Response {
    pub(crate) span: Span,
    /// The response code.
    pub code: Span<str>,
    /// The reason phrase.
    pub reason: Span<str>,
    /// Response headers.
    pub headers: Vec<Header>,
    /// Response body.
    pub body: Option<Body>,
}

impl Response {
    /// Returns all the response headers with the given name (case-insensitive). Returns `None` if
    /// there are no matching headers. Returns `Some` if there is at least one matching header.
    /// Never returns `Some` with an empty `Vec`.
    pub fn header(&self, name: &str) -> Option<Vec<&Header>> {
        let matching: Vec<&Header> = self
            .headers
            .iter()
            .filter(|h| h.name.0.as_str().eq_ignore_ascii_case(name))
            .collect();
        if matching.is_empty() {
            None
        } else {
            Some(matching)
        }
    }

    /// Shifts the span range by the given offset.
    pub fn offset(&mut self, offset: usize) {
        self.span.offset(offset);
        self.code.offset(offset);
        self.reason.offset(offset);
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
pub struct Body(pub(crate) Span);

impl Body {
    /// Returns the body as a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// Shifts the span range by the given offset.
    pub fn offset(&mut self, offset: usize) {
        self.0.offset(offset);
    }
}

impl Spanned for Body {
    fn span(&self) -> &Span {
        &self.0
    }
}

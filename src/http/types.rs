use crate::{Span, Spanned};

/// An HTTP header name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeaderName(pub(crate) Span<str>);

impl Spanned<str> for HeaderName {
    fn span(&self) -> &Span<str> {
        &self.0
    }
}

/// An HTTP header value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeaderValue(pub(crate) Span);

impl Spanned for HeaderValue {
    fn span(&self) -> &Span {
        &self.0
    }
}

/// An HTTP header.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    pub(crate) span: Span,
    /// The header name.
    pub name: HeaderName,
    /// The header value.
    pub value: HeaderValue,
}

impl Spanned for Header {
    fn span(&self) -> &Span {
        &self.span
    }
}

/// An HTTP request.
#[derive(Debug, Clone, PartialEq, Eq)]
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
    /// Returns the request header with the given name (case-insensitive).
    pub fn header(&self, name: &str) -> Option<&Header> {
        self.headers
            .iter()
            .find(|h| h.name.0.as_str().eq_ignore_ascii_case(name))
    }
}

impl Spanned for Request {
    fn span(&self) -> &Span {
        &self.span
    }
}

/// An HTTP response.
#[derive(Debug, Clone, PartialEq, Eq)]
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
    /// Returns the response header with the given name (case-insensitive).
    pub fn header(&self, name: &str) -> Option<&Header> {
        self.headers
            .iter()
            .find(|h| h.name.0.as_str().eq_ignore_ascii_case(name))
    }
}

impl Spanned for Response {
    fn span(&self) -> &Span {
        &self.span
    }
}

/// An HTTP request or response body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Body(pub(crate) Span);

impl Spanned for Body {
    fn span(&self) -> &Span {
        &self.0
    }
}

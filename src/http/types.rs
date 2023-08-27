use crate::{Span, Spanned};

/// An HTTP header name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeaderName<'a>(pub(crate) Span<'a, str>);

impl Spanned<str> for HeaderName<'_> {
    fn span(&self) -> &Span<'_, str> {
        &self.0
    }
}

/// An HTTP header value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeaderValue<'a>(pub(crate) Span<'a>);

impl Spanned for HeaderValue<'_> {
    fn span(&self) -> &Span<'_> {
        &self.0
    }
}

/// An HTTP header.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header<'a> {
    pub(crate) span: Span<'a>,
    /// The header name.
    pub name: HeaderName<'a>,
    /// The header value.
    pub value: HeaderValue<'a>,
}

impl Spanned for Header<'_> {
    fn span(&self) -> &Span<'_> {
        &self.span
    }
}

/// An HTTP request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request<'a> {
    pub(crate) span: Span<'a>,
    /// The request method.
    pub method: Span<'a, str>,
    /// The request path.
    pub path: Span<'a, str>,
    /// Request headers.
    pub headers: Vec<Header<'a>>,
    /// Request body.
    pub body: Option<Body<'a>>,
}

impl<'a> Request<'a> {
    /// Returns the request header with the given name (case-insensitive).
    pub fn header(&self, name: &str) -> Option<&Header<'a>> {
        self.headers
            .iter()
            .find(|h| h.name.0.span.eq_ignore_ascii_case(name))
    }
}

impl Spanned for Request<'_> {
    fn span(&self) -> &Span<'_> {
        &self.span
    }
}

/// An HTTP response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response<'a> {
    pub(crate) span: Span<'a>,
    /// The response code.
    pub code: Span<'a, str>,
    /// The reason phrase.
    pub reason: Span<'a, str>,
    /// Response headers.
    pub headers: Vec<Header<'a>>,
    /// Response body.
    pub body: Option<Body<'a>>,
}

impl<'a> Response<'a> {
    /// Returns the response header with the given name (case-insensitive).
    pub fn header(&self, name: &str) -> Option<&Header<'a>> {
        self.headers
            .iter()
            .find(|h| h.name.0.span.eq_ignore_ascii_case(name))
    }
}

impl Spanned for Response<'_> {
    fn span(&self) -> &Span<'_> {
        &self.span
    }
}

/// An HTTP request or response body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Body<'a>(pub(crate) Span<'a>);

impl Spanned for Body<'_> {
    fn span(&self) -> &Span<'_> {
        &self.0
    }
}

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

/// An HTTP header.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header<'a> {
    pub(crate) span: Span<'a>,
    pub(crate) name: HeaderName<'a>,
    pub(crate) value: HeaderValue<'a>,
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
    pub(crate) method: &'a str,
    pub(crate) path: &'a str,
    pub(crate) headers: Vec<Header<'a>>,
    pub(crate) body: Option<Span<'a>>,
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
    pub(crate) reason: &'a str,
    pub(crate) headers: Vec<Header<'a>>,
    pub(crate) body: Option<Span<'a>>,
}

impl Spanned for Response<'_> {
    fn span(&self) -> &Span<'_> {
        &self.span
    }
}

/// An HTTP request or response body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Body<'a>(Span<'a>);

impl Spanned for Body<'_> {
    fn span(&self) -> &Span<'_> {
        &self.0
    }
}

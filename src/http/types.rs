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
    pub(crate) name: HeaderName<'a>,
    pub(crate) value: HeaderValue<'a>,
}

impl<'a> Header<'a> {
    /// Returns the header name.
    pub fn name(&self) -> &HeaderName<'a> {
        &self.name
    }

    /// Returns the header value.
    pub fn value(&self) -> &HeaderValue<'a> {
        &self.value
    }
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
    pub(crate) method: Span<'a, str>,
    pub(crate) path: Span<'a, str>,
    pub(crate) headers: Vec<Header<'a>>,
    pub(crate) body: Option<Body<'a>>,
}

impl<'a> Request<'a> {
    /// Returns the request method.
    pub fn method(&self) -> &Span<'a, str> {
        &self.method
    }

    /// Returns the request path.
    pub fn path(&self) -> &Span<'a, str> {
        &self.path
    }

    /// Returns the request header with the given name (case-insensitive).
    pub fn header(&self, name: &str) -> Option<&Header<'a>> {
        self.headers
            .iter()
            .find(|h| h.name.0.span.eq_ignore_ascii_case(name))
    }

    /// Returns the request body
    pub fn body(&self) -> Option<&Body<'a>> {
        self.body.as_ref()
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
    pub(crate) code: Span<'a, str>,
    pub(crate) reason: Span<'a, str>,
    pub(crate) headers: Vec<Header<'a>>,
    pub(crate) body: Option<Body<'a>>,
}

impl<'a> Response<'a> {
    /// Returns the response code.
    pub fn code(&self) -> &Span<'a, str> {
        &self.code
    }

    /// Returns the response reason.
    pub fn reason(&self) -> &Span<'a, str> {
        &self.reason
    }

    /// Returns the response header with the given name (case-insensitive).
    pub fn header(&self, name: &str) -> Option<&Header<'a>> {
        self.headers
            .iter()
            .find(|h| h.name.0.span.eq_ignore_ascii_case(name))
    }

    /// Returns the response body
    pub fn body(&self) -> Option<&Body<'a>> {
        self.body.as_ref()
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

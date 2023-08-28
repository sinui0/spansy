//! HTTP span parsing.

mod span;
mod types;

pub use span::{parse_request, parse_response};
pub use types::{Body, Header, HeaderName, HeaderValue, Request, Response};

use crate::ParseError;
/// An iterator yielding parsed HTTP requests.
#[derive(Debug)]
pub struct Requests<'a> {
    src: &'a [u8],
    /// The current position in the source string.
    pos: usize,
}

impl<'a> Requests<'a> {
    /// Returns a new `Requests` iterator.
    pub fn new(src: &'a [u8]) -> Self {
        Self { src, pos: 0 }
    }
}

impl<'a> Iterator for Requests<'a> {
    type Item = Result<Request<'a>, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.src.len() {
            None
        } else {
            Some(parse_request(&self.src[self.pos..]).map(|req| {
                self.pos += req.span.range.end;
                req
            }))
        }
    }
}

/// An iterator yielding parsed HTTP responses.
#[derive(Debug)]
pub struct Responses<'a> {
    src: &'a [u8],
    /// The current position in the source string.
    pos: usize,
}

impl<'a> Responses<'a> {
    /// Returns a new `Responses` iterator.
    pub fn new(src: &'a [u8]) -> Self {
        Self { src, pos: 0 }
    }
}

impl<'a> Iterator for Responses<'a> {
    type Item = Result<Response<'a>, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.src.len() {
            None
        } else {
            Some(parse_response(&self.src[self.pos..]).map(|resp| {
                self.pos += resp.span.range.end;
                resp
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Spanned;

    use super::*;

    const MULTIPLE_REQUESTS: &[u8] = b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n\
        POST /hello HTTP/1.1\r\nHost: localhost\r\nContent-Length: 14\r\n\r\n\
        Hello, world!\n";

    const MULTIPLE_RESPONSES: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n\
        HTTP/1.1 200 OK\r\nContent-Length: 14\r\n\r\nHello, world!\n\
        HTTP/1.1 204 OK\r\nContent-Length: 0\r\n\r\n";

    #[test]
    fn test_parse_requests() {
        let reqs = Requests::new(MULTIPLE_REQUESTS)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(reqs.len(), 2);

        assert_eq!(reqs[0].method, "GET");
        assert!(reqs[0].body.is_none());
        assert_eq!(reqs[1].method, "POST");
        assert_eq!(
            reqs[1].body.as_ref().unwrap().span(),
            b"Hello, world!\n".as_slice()
        );
    }

    #[test]
    fn test_parse_responses() {
        let resps = Responses::new(MULTIPLE_RESPONSES)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(resps.len(), 3);

        assert_eq!(resps[0].code, "200");
        assert!(resps[0].body.is_none());
        assert_eq!(resps[1].code, "200");
        assert_eq!(
            resps[1].body.as_ref().unwrap().span(),
            b"Hello, world!\n".as_slice()
        );
        assert_eq!(resps[2].code, "204");
        assert!(resps[2].body.is_none());
    }
}

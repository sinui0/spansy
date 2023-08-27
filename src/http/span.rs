use crate::{
    helpers::get_span_range,
    http::types::{Body, Header, HeaderName, HeaderValue, Request, Response},
    ParseError, Span,
};

const MAX_HEADERS: usize = 128;

/// Parses an HTTP request.
pub fn parse_request(src: &[u8]) -> Result<Request<'_>, ParseError> {
    let mut headers = [httparse::EMPTY_HEADER; MAX_HEADERS];

    let (method, path, body_start) = {
        let mut request = httparse::Request::new(&mut headers);

        let body_start = match request.parse(src) {
            Ok(httparse::Status::Complete(body_start)) => body_start,
            Ok(httparse::Status::Partial) => {
                return Err(ParseError(format!("incomplete request: {:?}", src)))
            }
            Err(err) => return Err(ParseError(err.to_string())),
        };

        let method = request
            .method
            .ok_or_else(|| ParseError("method missing from request".to_string()))?;

        let path = request
            .path
            .ok_or_else(|| ParseError("path missing from request".to_string()))?;

        (method, path, body_start)
    };

    let headers = headers
        .iter()
        .take_while(|h| *h != &httparse::EMPTY_HEADER)
        .map(|h| {
            let name = HeaderName(Span {
                src,
                span: h.name,
                range: get_span_range(src, h.name.as_bytes()),
            });

            let value = HeaderValue(Span {
                src,
                span: h.value,
                range: get_span_range(src, h.value),
            });

            Header {
                span: Span {
                    src,
                    span: &src[..body_start],
                    range: 0..body_start,
                },
                name,
                value,
            }
        })
        .collect();

    // httparse allocates a new buffer to store the method for performance reasons,
    // so we have to search for the span in the source. This is quick as the method
    // is at the front.
    let method = src
        .windows(method.len())
        .find(|w| *w == method.as_bytes())
        .expect("method is present");

    let mut request = Request {
        span: Span {
            src,
            span: src,
            range: 0..src.len(),
        },
        method: Span {
            src,
            span: std::str::from_utf8(method).expect("method is valid utf-8"),
            range: get_span_range(src, method),
        },
        path: Span {
            src,
            span: path,
            range: get_span_range(src, path.as_bytes()),
        },
        headers,
        body: None,
    };

    let body_len = if body_start == src.len() {
        0
    } else if let Some(h) = request.header("Content-Length") {
        std::str::from_utf8(h.value.0.span)?
            .parse::<usize>()
            .map_err(|err| ParseError(format!("failed to parse Content-Length value: {err}")))?
    } else if request.header("Transfer-Encoding").is_some() {
        return Err(ParseError(
            "Transfer-Encoding not supported yet".to_string(),
        ));
    } else {
        return Err(ParseError(
            "A request with a body must contain either a Content-Length or Transfer-Encoding header".to_string(),
        ));
    };

    if body_len > 0 {
        let range = body_start..body_start + body_len;

        if range.end > src.len() {
            return Err(ParseError(format!(
                "body range {}..{} exceeds source {}",
                range.start,
                range.end,
                src.len()
            )));
        }

        request.span = Span {
            src,
            span: &src[..range.end],
            range: 0..range.end,
        };

        request.body = Some(Body(Span {
            src,
            span: &src[range.clone()],
            range,
        }));
    }

    Ok(request)
}

/// Parses an HTTP response.
pub fn parse_response(src: &[u8]) -> Result<Response<'_>, ParseError> {
    let mut headers = [httparse::EMPTY_HEADER; MAX_HEADERS];

    let (reason, code, body_start) = {
        let mut response = httparse::Response::new(&mut headers);

        let body_start = match response.parse(src) {
            Ok(httparse::Status::Complete(body_start)) => body_start,
            Ok(httparse::Status::Partial) => {
                return Err(ParseError(format!("incomplete response: {:?}", src)))
            }
            Err(err) => return Err(ParseError(err.to_string())),
        };

        let code = response
            .code
            .ok_or_else(|| ParseError("code missing from response".to_string()))
            .map(|c| c.to_string())?;

        let reason = response
            .reason
            .ok_or_else(|| ParseError("reason missing from response".to_string()))?;

        (reason, code, body_start)
    };

    let headers = headers
        .iter()
        .take_while(|h| *h != &httparse::EMPTY_HEADER)
        .map(|h| {
            let name = HeaderName(Span {
                src,
                span: h.name,
                range: get_span_range(src, h.name.as_bytes()),
            });

            let value = HeaderValue(Span {
                src,
                span: h.value,
                range: get_span_range(src, h.value),
            });

            Header {
                span: Span {
                    src,
                    span: &src[..body_start],
                    range: 0..body_start,
                },
                name,
                value,
            }
        })
        .collect();

    // httparse doesn't preserve the response code span, so we find it.
    let code = src
        .windows(3)
        .find(|w| *w == code.as_bytes())
        .expect("code is present");

    let mut response = Response {
        span: Span {
            src,
            span: src,
            range: 0..src.len(),
        },
        code: Span {
            src,
            span: std::str::from_utf8(code).expect("code is valid utf-8"),
            range: get_span_range(src, code),
        },
        reason: Span {
            src,
            span: reason,
            range: get_span_range(src, reason.as_bytes()),
        },
        headers,
        body: None,
    };

    let body_len = if body_start == src.len() {
        0
    } else if let Some(h) = response.header("Content-Length") {
        std::str::from_utf8(h.value.0.span)?
            .parse::<usize>()
            .map_err(|err| ParseError(format!("failed to parse Content-Length value: {err}")))?
    } else if response.header("Transfer-Encoding").is_some() {
        return Err(ParseError(
            "Transfer-Encoding not supported yet".to_string(),
        ));
    } else {
        return Err(ParseError(
            "A response with a body must contain either a Content-Length or Transfer-Encoding header".to_string(),
        ));
    };

    if body_len > 0 {
        let range = body_start..body_start + body_len;

        if range.end > src.len() {
            return Err(ParseError(format!(
                "body range {}..{} exceeds source {}",
                range.start,
                range.end,
                src.len()
            )));
        }

        response.span = Span {
            src,
            span: &src[..range.end],
            range: 0..range.end,
        };

        response.body = Some(Body(Span {
            src,
            span: &src[range.clone()],
            range,
        }));
    }

    Ok(response)
}

#[cfg(test)]
mod tests {
    use crate::Spanned;

    use super::*;

    const TEST_REQUEST: &[u8] = b"\
                        GET /home.html HTTP/1.1\n\
                        Host: developer.mozilla.org\n\
                        User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.9; rv:50.0) Gecko/20100101 Firefox/50.0\n\
                        Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.\n\
                        Accept-Language: en-US,en;q=0.\n\
                        Accept-Encoding: gzip, deflate, b\n\
                        Referer: https://developer.mozilla.org/testpage.htm\n\
                        Connection: keep-alive\n\
                        Content-Length: 12\n\
                        Cache-Control: max-age=0\n\n\
                        Hello World!";

    const TEST_RESPONSE: &[u8] = b"\
                        HTTP/1.1 200 OK\n\
                        Date: Mon, 27 Jul 2009 12:28:53 GMT\n\
                        Server: Apache/2.2.14 (Win32)\n\
                        Last-Modified: Wed, 22 Jul 2009 19:15:56 GMT\n\
                        Content-Length: 52\n\
                        Content-Type: text/html\n\
                        Connection: Closed\n\n\
                        <html>\n\
                        <body>\n\
                        <h1>Hello, World!</h1>\n\
                        </body>\n\
                        </html>";

    #[test]
    fn test_parse_request() {
        let req = parse_request(TEST_REQUEST).unwrap();

        assert_eq!(req.span(), TEST_REQUEST);
        assert_eq!(req.method, "GET");
        assert_eq!(
            req.header("Host").unwrap().value.span(),
            b"developer.mozilla.org".as_slice()
        );
        assert_eq!(
            req.header("User-Agent").unwrap().value.span(),
            b"Mozilla/5.0 (Macintosh; Intel Mac OS X 10.9; rv:50.0) Gecko/20100101 Firefox/50.0"
                .as_slice()
        );
        assert_eq!(req.body.unwrap().span(), b"Hello World!".as_slice());
    }

    #[test]
    fn test_parse_response() {
        let res = parse_response(TEST_RESPONSE).unwrap();

        assert_eq!(res.span(), TEST_RESPONSE);
        assert_eq!(res.code, "200");
        assert_eq!(res.reason, "OK");
        assert_eq!(
            res.header("Server").unwrap().value.span(),
            b"Apache/2.2.14 (Win32)".as_slice()
        );
        assert_eq!(
            res.header("Connection").unwrap().value.span(),
            b"Closed".as_slice()
        );
        assert_eq!(
            res.body.unwrap().span(),
            b"<html>\n<body>\n<h1>Hello, World!</h1>\n</body>\n</html>".as_slice()
        );
    }
}

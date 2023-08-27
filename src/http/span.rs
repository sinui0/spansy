use crate::{
    helpers::get_span_range,
    http::types::{Header, HeaderName, HeaderValue, Request, Response},
    ParseError, Span,
};

const MAX_HEADERS: usize = 128;

/// Parses an HTTP request.
pub fn parse_request(src: &[u8]) -> Result<Request<'_>, ParseError> {
    let mut headers = vec![httparse::EMPTY_HEADER; MAX_HEADERS];

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

    Ok(Request {
        span: Span {
            src,
            span: &src[..body_start],
            range: 0..body_start,
        },
        method,
        path,
        headers,
        body: None,
    })
}

/// Parses an HTTP response.
pub fn parse_response(src: &[u8]) -> Result<(), String> {
    todo!()
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
                        Cache-Control: max-age=0\n\n\
                        Hello World!";

    const TEST_RESPONSE: &[u8] = b"\
                        HTTP/1.1 200 OK\n\
                        Date: Mon, 27 Jul 2009 12:28:53 GMT\n\
                        Server: Apache/2.2.14 (Win32)\n\
                        Last-Modified: Wed, 22 Jul 2009 19:15:56 GMT\n\
                        Content-Length: 88\n\
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

        println!("{:#?}", std::str::from_utf8(req.span().span()).unwrap());

        println!("{}", TEST_REQUEST.len());
    }
}

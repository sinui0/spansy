//! HTTP span parsing.

mod span;
mod types;

pub use span::{parse_request, parse_response};
pub use types::{Body, Header, HeaderName, HeaderValue, Request, Response};

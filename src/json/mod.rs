//! A JSON span parser.
//!
//! # serde-json
//!
//! Much of the code in this module is based on or copied from the [serde-json](https://github.com/serde-rs/json) crate.

macro_rules! tri {
    ($e:expr $(,)?) => {
        match $e {
            core::result::Result::Ok(val) => val,
            core::result::Result::Err(err) => return core::result::Result::Err(err),
        }
    };
}

mod error;
mod read;
mod span;
mod types;

pub use error::{Error, Result};
pub use span::JsonSpanner;
pub use types::JsonValue;

pub fn parse_json_spans<'a>(src: &'a [u8]) -> Result<JsonValue> {
    let mut span = span::JsonSpanner::new(src);

    span.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let src = b"{ \"foo\": [null,42, {\"test\":\"ok\"},    -16]}";

        let value = parse_json_spans(src).unwrap();

        println!("{:#?}", value);
        println!(
            "value: {:?}",
            String::from_utf8_lossy(&src[value.range().clone()])
        );
    }
}

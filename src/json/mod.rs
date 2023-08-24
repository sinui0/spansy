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
pub mod visit;

pub use error::{Error, Result};
pub use span::JsonSpanner;
pub use types::JsonValue;

#[cfg(test)]
mod tests {
    use super::*;

    use visit::Visit;

    struct NumberPrinter<'a> {
        src: &'a [u8],
    }

    impl<'a> Visit for NumberPrinter<'a> {
        fn visit_number(&mut self, node: &types::Number) {
            println!(
                "number: {:?}",
                String::from_utf8_lossy(&self.src[node.range.clone()])
            );
        }
    }

    #[test]
    fn test() {
        let src = b"{ \"foo\": [null,42, {\"test\":\"ok\"},    -16]}";

        let value = span::JsonSpanner::new(src).parse().unwrap();

        NumberPrinter { src }.visit_value(&value);
    }
}

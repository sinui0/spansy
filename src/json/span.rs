use pest::{iterators::Pair, Parser};

use super::types::{self, JsonValue};

use crate::{ParseError, Span};

#[derive(pest_derive::Parser)]
#[grammar = "json/json.pest"]
struct JsonParser;

/// Parse a JSON value from a source string.
pub fn parse(src: &str) -> Result<JsonValue<'_>, ParseError> {
    Ok(JsonParser::parse(Rule::json, src)?
        .next()
        .ok_or_else(|| ParseError(format!("no JSON value found in source string: {:?}", src)))?
        .into())
}

impl<'a> From<Pair<'a, Rule>> for types::JsonKey<'a> {
    fn from(value: Pair<'a, Rule>) -> Self {
        assert!(matches!(value.as_rule(), Rule::string));

        let span = value.as_span();
        let range = span.start()..span.end();

        Self(Span {
            src: value.get_input().as_bytes(),
            span: value.as_str(),
            range,
        })
    }
}

impl<'a> From<Pair<'a, Rule>> for types::Number<'a> {
    fn from(value: Pair<'a, Rule>) -> Self {
        assert!(matches!(value.as_rule(), Rule::number));

        let span = value.as_span();
        let range = span.start()..span.end();

        Self(Span {
            src: value.get_input().as_bytes(),
            span: value.as_str(),
            range,
        })
    }
}

impl<'a> From<Pair<'a, Rule>> for types::Bool<'a> {
    fn from(value: Pair<'a, Rule>) -> Self {
        assert!(matches!(value.as_rule(), Rule::bool));

        let span = value.as_span();
        let range = span.start()..span.end();

        Self(Span {
            src: value.get_input().as_bytes(),
            span: value.as_str(),
            range,
        })
    }
}

impl<'a> From<Pair<'a, Rule>> for types::Null<'a> {
    fn from(value: Pair<'a, Rule>) -> Self {
        assert!(matches!(value.as_rule(), Rule::null));

        let span = value.as_span();
        let range = span.start()..span.end();

        Self(Span {
            src: value.get_input().as_bytes(),
            span: value.as_str(),
            range,
        })
    }
}

impl<'a> From<Pair<'a, Rule>> for types::String<'a> {
    fn from(value: Pair<'a, Rule>) -> Self {
        assert!(matches!(value.as_rule(), Rule::string));

        let span = value.as_span();
        let range = span.start()..span.end();

        Self(Span {
            src: value.get_input().as_bytes(),
            span: value.as_str(),
            range,
        })
    }
}

impl<'a> From<Pair<'a, Rule>> for types::Object<'a> {
    fn from(value: Pair<'a, Rule>) -> Self {
        assert!(matches!(value.as_rule(), Rule::object));

        let span = value.as_span();
        let range = span.start()..span.end();

        let span = Span {
            src: value.get_input().as_bytes(),
            span: value.as_str(),
            range,
        };

        types::Object {
            span,
            elems: value
                .into_inner()
                .map(|pair| {
                    let types::KeyValue { key, value } = types::KeyValue::try_from(pair).unwrap();
                    (key, value)
                })
                .collect(),
        }
    }
}

impl<'a> From<Pair<'a, Rule>> for types::Array<'a> {
    fn from(value: Pair<'a, Rule>) -> Self {
        assert!(matches!(value.as_rule(), Rule::array));

        let span = value.as_span();
        let range = span.start()..span.end();

        let span = Span {
            src: value.get_input().as_bytes(),
            span: value.as_str(),
            range,
        };

        types::Array {
            span,
            elems: value.into_inner().map(|pair| pair.into()).collect(),
        }
    }
}

impl<'a> From<Pair<'a, Rule>> for types::JsonValue<'a> {
    fn from(value: Pair<'a, Rule>) -> Self {
        match value.as_rule() {
            Rule::object => JsonValue::Object(value.into()),
            Rule::array => JsonValue::Array(value.into()),
            Rule::string => JsonValue::String(value.into()),
            Rule::number => JsonValue::Number(value.into()),
            Rule::bool => JsonValue::Bool(value.into()),
            Rule::null => JsonValue::Null(value.into()),
            rule => unreachable!("unexpected matched rule: {:?}", rule),
        }
    }
}

impl<'a> From<Pair<'a, Rule>> for types::KeyValue<'a> {
    fn from(value: Pair<'a, Rule>) -> Self {
        assert!(matches!(value.as_rule(), Rule::pair));

        let [key, value]: [Pair<'a, Rule>; 2] = value
            .into_inner()
            .collect::<Vec<_>>()
            .try_into()
            .expect("pair has two children");

        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Spanned;

    use super::*;

    #[test]
    fn test_json_spanner() {
        let src = r#"{"foo": "bar", "baz": 123, "quux": { "a": "b", "c": "d" }, "arr": [1, 2, 3]}"#;

        let value = parse(src).unwrap();

        assert_eq!(value.get("foo").unwrap().span(), "bar");
        assert_eq!(value.get("baz").unwrap().span(), "123");
        assert_eq!(value.get("quux.a").unwrap().span(), "b");
        assert_eq!(value.get("arr").unwrap().span(), "[1, 2, 3]");
    }
}

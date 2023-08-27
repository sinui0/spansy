use pest::{iterators::Pair, Parser};

use super::types::{self, JsonValue};

use crate::{ParseError, Span};

#[derive(pest_derive::Parser)]
#[grammar = "json/json.pest"]
struct JsonParser;

/// A JSON span parser.
pub struct JsonSpanner<'a> {
    src: &'a str,
}

impl<'a> JsonSpanner<'a> {
    /// Create a new JSON span parser with the given source string.
    pub fn new(src: &'a str) -> Self {
        Self { src }
    }

    /// Parse the source string spans.
    pub fn parse(&self) -> Result<JsonValue<'a>, ParseError> {
        Ok(JsonParser::parse(Rule::json, self.src)?
            .next()
            .ok_or_else(|| {
                ParseError(format!(
                    "no JSON value found in source string: {:?}",
                    self.src
                ))
            })?
            .into())
    }
}

impl<'a> From<Pair<'a, Rule>> for types::JsonKey<'a> {
    fn from(value: Pair<'a, Rule>) -> Self {
        assert!(matches!(value.as_rule(), Rule::string));

        Self(Span::new(
            value.as_str(),
            value.as_span().start()..value.as_span().end(),
        ))
    }
}

impl<'a> From<Pair<'a, Rule>> for types::Number<'a> {
    fn from(value: Pair<'a, Rule>) -> Self {
        assert!(matches!(value.as_rule(), Rule::number));

        Self(Span::new(
            value.as_str(),
            value.as_span().start()..value.as_span().end(),
        ))
    }
}

impl<'a> From<Pair<'a, Rule>> for types::Bool<'a> {
    fn from(value: Pair<'a, Rule>) -> Self {
        assert!(matches!(value.as_rule(), Rule::bool));

        Self(Span::new(
            value.as_str(),
            value.as_span().start()..value.as_span().end(),
        ))
    }
}

impl<'a> From<Pair<'a, Rule>> for types::Null<'a> {
    fn from(value: Pair<'a, Rule>) -> Self {
        assert!(matches!(value.as_rule(), Rule::null));

        Self(Span::new(
            value.as_str(),
            value.as_span().start()..value.as_span().end(),
        ))
    }
}

impl<'a> From<Pair<'a, Rule>> for types::String<'a> {
    fn from(value: Pair<'a, Rule>) -> Self {
        assert!(matches!(value.as_rule(), Rule::string));

        Self(Span::new(
            value.as_str(),
            value.as_span().start()..value.as_span().end(),
        ))
    }
}

impl<'a> From<Pair<'a, Rule>> for types::Object<'a> {
    fn from(value: Pair<'a, Rule>) -> Self {
        assert!(matches!(value.as_rule(), Rule::object));

        types::Object {
            span: Span::new(
                value.as_str(),
                value.as_span().start()..value.as_span().end(),
            ),
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

        types::Array {
            span: Span::new(
                value.as_str(),
                value.as_span().start()..value.as_span().end(),
            ),
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
    use super::*;
    use pest::Parser;

    #[test]
    fn test_json_parser() {
        let src = r#"{"foo": "bar", "baz": 123, "quux": { "a": "b", "c": "d" }, "arr": [1, 2, 3]}"#;

        let value = JsonParser::parse(Rule::json, src).unwrap();

        println!("{:#?}", value);
    }

    #[test]
    fn test_json_parser_array() {
        let src = r#"[1, 2, 3]"#;

        let value = JsonParser::parse(Rule::json, src).unwrap();

        println!("{:#?}", value);
    }

    #[test]
    fn test_json_spanner() {
        let src = r#"{"foo": "bar", "is": ["he"]}"#;

        let value = JsonSpanner::new(src).parse().unwrap();

        println!("{:#?}", value);
    }
}

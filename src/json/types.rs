use std::ops::Range;

#[derive(Debug)]
pub struct JsonKey {
    pub range: Range<usize>,
}

#[derive(Debug)]
pub enum JsonValue {
    Null(Null),
    Bool(Bool),
    Number(Number),
    String(String),
    Array(Array),
    Object(Object),
}

impl JsonValue {
    pub fn range(&self) -> &Range<usize> {
        match self {
            JsonValue::Null(value) => &value.range,
            JsonValue::Bool(value) => &value.range,
            JsonValue::Number(value) => &value.range,
            JsonValue::String(value) => &value.range,
            JsonValue::Array(value) => &value.range,
            JsonValue::Object(value) => &value.range,
        }
    }
}

#[derive(Debug)]
pub struct Null {
    pub range: Range<usize>,
}

#[derive(Debug)]
pub struct Bool {
    pub range: Range<usize>,
}

#[derive(Debug)]
pub struct Number {
    pub range: Range<usize>,
}

#[derive(Debug)]
pub struct String {
    pub range: Range<usize>,
}

#[derive(Debug)]
pub struct Array {
    pub range: Range<usize>,
    pub elems: Vec<JsonValue>,
}

#[derive(Debug)]
pub struct Object {
    pub range: Range<usize>,
    pub elems: Vec<(JsonKey, JsonValue)>,
}

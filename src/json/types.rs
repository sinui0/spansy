use std::ops::Range;

use crate::{Span, Spanned};

#[derive(Debug, Clone, PartialEq, Eq)]
/// A JSON value.
pub enum JsonValue {
    /// A null value.
    Null(Null),
    /// A boolean value.
    Bool(Bool),
    /// A number value.
    Number(Number),
    /// A string value.
    String(String),
    /// An array value.
    Array(Array),
    /// An object value.
    Object(Object),
}

impl JsonValue {
    /// Returns the span corresponding to the value.
    pub fn into_span(self) -> Span<str> {
        match self {
            JsonValue::Null(v) => v.0,
            JsonValue::Bool(v) => v.0,
            JsonValue::Number(v) => v.0,
            JsonValue::String(v) => v.0,
            JsonValue::Array(v) => v.span,
            JsonValue::Object(v) => v.span,
        }
    }

    /// Shifts the span range by the given offset.
    pub fn offset(&mut self, offset: usize) {
        match self {
            JsonValue::Null(v) => v.0.offset(offset),
            JsonValue::Bool(v) => v.0.offset(offset),
            JsonValue::Number(v) => v.0.offset(offset),
            JsonValue::String(v) => v.0.offset(offset),
            JsonValue::Array(v) => {
                v.span.offset(offset);
                v.elems.iter_mut().for_each(|v| v.offset(offset))
            }
            JsonValue::Object(v) => {
                v.span.offset(offset);
                v.elems.iter_mut().for_each(|(k, v)| {
                    k.offset(offset);
                    v.offset(offset);
                })
            }
        }
    }
}

impl Spanned<str> for JsonValue {
    fn span(&self) -> &Span<str> {
        match self {
            JsonValue::Null(v) => v.span(),
            JsonValue::Bool(v) => v.span(),
            JsonValue::Number(v) => v.span(),
            JsonValue::String(v) => v.span(),
            JsonValue::Array(v) => v.span(),
            JsonValue::Object(v) => v.span(),
        }
    }
}

impl JsonValue {
    /// Get a reference to the value using the given path.
    ///
    /// # Example
    ///
    /// ```
    /// use spansy::json::parse_str;
    /// use spansy::Spanned;
    ///
    /// let src = "{\"foo\": {\"bar\": [42, 14]}}";
    ///
    /// let value = parse_str(src).unwrap();
    ///
    /// assert_eq!(value.get("foo.bar.1").unwrap().span(), "14");
    /// ```
    pub fn get(&self, path: &str) -> Option<&JsonValue> {
        match self {
            JsonValue::Null(_) => None,
            JsonValue::Bool(_) => None,
            JsonValue::Number(_) => None,
            JsonValue::String(_) => None,
            JsonValue::Array(v) => v.get(path),
            JsonValue::Object(v) => v.get(path),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct KeyValue {
    pub(crate) key: JsonKey,
    pub(crate) value: JsonValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A key in a JSON object.
pub struct JsonKey(pub(crate) Span<str>);

#[derive(Debug, Clone, PartialEq, Eq)]
/// A null value.
pub struct Null(pub(crate) Span<str>);

#[derive(Debug, Clone, PartialEq, Eq)]
/// A boolean value.
pub struct Bool(pub(crate) Span<str>);

#[derive(Debug, Clone, PartialEq, Eq)]
/// A number value.
pub struct Number(pub(crate) Span<str>);

#[derive(Debug, Clone, PartialEq, Eq)]
/// A string value.
pub struct String(pub(crate) Span<str>);

#[derive(Debug, Clone, PartialEq, Eq)]
/// An array value.
pub struct Array {
    pub(crate) span: Span<str>,
    /// The elements of the array.
    pub elems: Vec<JsonValue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// An object value.
pub struct Object {
    pub(crate) span: Span<str>,
    /// The key value pairs of the object.
    pub elems: Vec<(JsonKey, JsonValue)>,
}

impl Array {
    /// Get a reference to the value using the given path.
    pub fn get(&self, path: &str) -> Option<&JsonValue> {
        let mut path_iter = path.split('.');

        let key = path_iter.next()?;
        let idx = key.parse::<usize>().ok()?;

        let value = self.elems.get(idx)?;

        if path_iter.next().is_some() {
            value.get(&path[key.len() + 1..])
        } else {
            Some(value)
        }
    }
}

impl Object {
    /// Get a reference to the value using the given path.
    fn get(&self, path: &str) -> Option<&JsonValue> {
        let mut path_iter = path.split('.');

        let key = path_iter.next()?;

        let (_, value) = self.elems.iter().find(|(k, _)| k == key)?;

        if path_iter.next().is_some() {
            value.get(&path[key.len() + 1..])
        } else {
            Some(value)
        }
    }
}

macro_rules! impl_type {
    ($ty:ident, $span:tt) => {
        impl $ty {
            /// Returns the span corresponding to the value.
            pub fn into_span(self) -> Span<str> {
                self.$span
            }

            /// Shifts the span range by the given offset.
            pub fn offset(&mut self, offset: usize) {
                self.$span.offset(offset);
            }
        }

        impl Spanned<str> for $ty {
            fn span(&self) -> &Span<str> {
                &self.$span
            }
        }

        impl PartialEq<str> for $ty {
            fn eq(&self, other: &str) -> bool {
                self.$span == other
            }
        }

        impl PartialEq<$ty> for str {
            fn eq(&self, other: &$ty) -> bool {
                self == &other.$span
            }
        }

        impl PartialEq<&str> for $ty {
            fn eq(&self, other: &&str) -> bool {
                self.$span == *other
            }
        }

        impl PartialEq<$ty> for &str {
            fn eq(&self, other: &$ty) -> bool {
                *self == &other.$span
            }
        }

        impl PartialEq<Range<usize>> for $ty {
            fn eq(&self, other: &Range<usize>) -> bool {
                &self.$span == other
            }
        }

        impl PartialEq<$ty> for Range<usize> {
            fn eq(&self, other: &$ty) -> bool {
                self == &other.$span
            }
        }

        impl PartialEq<Span<str>> for $ty {
            fn eq(&self, other: &Span<str>) -> bool {
                &self.$span == other
            }
        }

        impl PartialEq<$ty> for Span<str> {
            fn eq(&self, other: &$ty) -> bool {
                self == &other.$span
            }
        }
    };
}

impl_type!(JsonKey, 0);
impl_type!(Null, 0);
impl_type!(Bool, 0);
impl_type!(Number, 0);
impl_type!(String, 0);
impl_type!(Array, span);
impl_type!(Object, span);

#[cfg(test)]
mod tests {
    use crate::json::parse_str;

    use super::*;

    #[test]
    fn test_obj_index() {
        let src = "{\"foo\": \"bar\"}";

        let value = parse_str(src).unwrap();

        assert_eq!(value.get("foo").unwrap().span(), "bar");
    }

    #[test]
    fn test_array_index() {
        let src = "{\"foo\": [42, 14]}";

        let value = parse_str(src).unwrap();

        assert_eq!(value.get("foo.1").unwrap().span(), "14");
    }

    #[test]
    fn test_nested_index() {
        let src = "{\"foo\": {\"bar\": [42, 14]}}";

        let value = parse_str(src).unwrap();

        assert_eq!(value.get("foo.bar.1").unwrap().span(), "14");
    }
}

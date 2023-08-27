use std::ops::Range;

use crate::{Span, Spanned};

#[derive(Debug, Clone, PartialEq, Eq)]
/// A JSON value.
pub enum JsonValue<'a> {
    /// A null value.
    Null(Null<'a>),
    /// A boolean value.
    Bool(Bool<'a>),
    /// A number value.
    Number(Number<'a>),
    /// A string value.
    String(String<'a>),
    /// An array value.
    Array(Array<'a>),
    /// An object value.
    Object(Object<'a>),
}

impl<'a> JsonValue<'a> {
    /// Returns the span corresponding to the value.
    pub fn into_span(self) -> Span<'a> {
        match self {
            JsonValue::Null(v) => v.0,
            JsonValue::Bool(v) => v.0,
            JsonValue::Number(v) => v.0,
            JsonValue::String(v) => v.0,
            JsonValue::Array(v) => v.span,
            JsonValue::Object(v) => v.span,
        }
    }
}

impl Spanned for JsonValue<'_> {
    fn span(&self) -> &Span<'_> {
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

impl<'a> JsonValue<'a> {
    /// Get a reference to the value using the given path.
    ///
    /// # Example
    ///
    /// ```
    /// use spansy::json::JsonSpanner;
    /// use spansy::Spanned;
    ///
    /// let src = "{\"foo\": {\"bar\": [42, 14]}}";
    ///
    /// let value = parse(src).unwrap();
    ///
    /// assert_eq!(value.get("foo.bar.1").unwrap().span(), "14");
    /// ```
    pub fn get(&self, path: &str) -> Option<&JsonValue<'a>> {
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
pub(crate) struct KeyValue<'a> {
    pub(crate) key: JsonKey<'a>,
    pub(crate) value: JsonValue<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A key in a JSON object.
pub struct JsonKey<'a>(pub(crate) Span<'a>);

#[derive(Debug, Clone, PartialEq, Eq)]
/// A null value.
pub struct Null<'a>(pub(crate) Span<'a>);

#[derive(Debug, Clone, PartialEq, Eq)]
/// A boolean value.
pub struct Bool<'a>(pub(crate) Span<'a>);

#[derive(Debug, Clone, PartialEq, Eq)]
/// A number value.
pub struct Number<'a>(pub(crate) Span<'a>);

#[derive(Debug, Clone, PartialEq, Eq)]
/// A string value.
pub struct String<'a>(pub(crate) Span<'a>);

#[derive(Debug, Clone, PartialEq, Eq)]
/// An array value.
pub struct Array<'a> {
    pub(crate) span: Span<'a>,
    /// The elements of the array.
    pub elems: Vec<JsonValue<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// An object value.
pub struct Object<'a> {
    pub(crate) span: Span<'a>,
    /// The key value pairs of the object.
    pub elems: Vec<(JsonKey<'a>, JsonValue<'a>)>,
}

impl<'a> Array<'a> {
    /// Get a reference to the value using the given path.
    pub fn get(&self, path: &str) -> Option<&JsonValue<'a>> {
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

impl<'a> Object<'a> {
    /// Get a reference to the value using the given path.
    fn get(&self, path: &str) -> Option<&JsonValue<'a>> {
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
        impl<'a> $ty<'a> {
            /// Returns the span corresponding to the value.
            pub fn into_span(self) -> Span<'a> {
                self.$span
            }
        }

        impl Spanned for $ty<'_> {
            fn span(&self) -> &Span<'_> {
                &self.$span
            }
        }

        impl PartialEq<str> for $ty<'_> {
            fn eq(&self, other: &str) -> bool {
                self.$span == other
            }
        }

        impl PartialEq<$ty<'_>> for str {
            fn eq(&self, other: &$ty<'_>) -> bool {
                self == &other.$span
            }
        }

        impl PartialEq<&str> for $ty<'_> {
            fn eq(&self, other: &&str) -> bool {
                self.$span == *other
            }
        }

        impl PartialEq<$ty<'_>> for &str {
            fn eq(&self, other: &$ty<'_>) -> bool {
                *self == &other.$span
            }
        }

        impl PartialEq<Range<usize>> for $ty<'_> {
            fn eq(&self, other: &Range<usize>) -> bool {
                &self.$span == other
            }
        }

        impl PartialEq<$ty<'_>> for Range<usize> {
            fn eq(&self, other: &$ty<'_>) -> bool {
                self == &other.$span
            }
        }

        impl PartialEq<Span<'_>> for $ty<'_> {
            fn eq(&self, other: &Span<'_>) -> bool {
                &self.$span == other
            }
        }

        impl PartialEq<$ty<'_>> for Span<'_> {
            fn eq(&self, other: &$ty<'_>) -> bool {
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
    use crate::json::parse;

    use super::*;

    #[test]
    fn test_obj_index() {
        let src = "{\"foo\": \"bar\"}";

        let value = parse(src).unwrap();

        assert_eq!(value.get("foo").unwrap().span(), "bar");
    }

    #[test]
    fn test_array_index() {
        let src = "{\"foo\": [42, 14]}";

        let value = parse(src).unwrap();

        assert_eq!(value.get("foo.1").unwrap().span(), "14");
    }

    #[test]
    fn test_nested_index() {
        let src = "{\"foo\": {\"bar\": [42, 14]}}";

        let value = parse(src).unwrap();

        assert_eq!(value.get("foo.bar.1").unwrap().span(), "14");
    }
}

use super::{types, types::JsonValue};

/// A visitor for JSON values.
///
/// # Example
///
/// ```
/// use spansy::json::{Number, JsonSpanner, JsonVisit};
/// use spansy::Spanned;
///
/// struct DigitReplacer<'a, 'b> {
///     src: &'a mut String,
///     digit: &'b str,
/// }
///
/// impl<'a> JsonVisit for DigitReplacer<'a, '_> {
///     fn visit_number(&mut self, node: &Number) {
///         let span = node.span();
///         let replacement = self.digit.repeat(span.len());
///         self.src.replace_range(node.span().range(), &replacement);
///     }
/// }
///
/// let src = "{\"foo\": [42, 69]}";
///
/// let value = JsonSpanner::new(src).parse().unwrap();
///
/// let mut new = src.to_string();
///
/// // Replace the digits of all numbers with 9.
/// DigitReplacer { src: &mut new, digit: "9" }.visit_value(&value);
///
/// assert_eq!(new, "{\"foo\": [99, 99]}");
/// ```
pub trait JsonVisit {
    /// Visit a key in a JSON object.
    fn visit_key(&mut self, _node: &types::JsonKey) {}

    /// Visit a JSON value.
    fn visit_value(&mut self, node: &JsonValue) {
        match node {
            JsonValue::Null(value) => self.visit_null(value),
            JsonValue::Bool(value) => self.visit_bool(value),
            JsonValue::Number(value) => self.visit_number(value),
            JsonValue::String(value) => self.visit_string(value),
            JsonValue::Array(value) => self.visit_array(value),
            JsonValue::Object(value) => self.visit_object(value),
        }
    }

    /// Visit an array value.
    fn visit_array(&mut self, node: &types::Array) {
        for elem in &node.elems {
            self.visit_value(elem);
        }
    }

    /// Visit an object value.
    fn visit_object(&mut self, node: &types::Object) {
        for (key, value) in &node.elems {
            self.visit_key(key);
            self.visit_value(value);
        }
    }

    /// Visit a null value.
    fn visit_null(&mut self, _node: &types::Null) {}

    /// Visit a boolean value.
    fn visit_bool(&mut self, _node: &types::Bool) {}

    /// Visit a number value.
    fn visit_number(&mut self, _node: &types::Number) {}

    /// Visit a string value.
    fn visit_string(&mut self, _node: &types::String) {}
}

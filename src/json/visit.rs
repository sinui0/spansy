use super::{types, types::JsonValue};

pub trait Visit {
    fn visit_key(&mut self, _node: &types::JsonKey) {}

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

    fn visit_array(&mut self, node: &types::Array) {
        for elem in &node.elems {
            self.visit_value(elem);
        }
    }

    fn visit_object(&mut self, node: &types::Object) {
        for (key, value) in &node.elems {
            self.visit_key(key);
            self.visit_value(value);
        }
    }

    fn visit_null(&mut self, _node: &types::Null) {}

    fn visit_bool(&mut self, _node: &types::Bool) {}

    fn visit_number(&mut self, _node: &types::Number) {}

    fn visit_string(&mut self, _node: &types::String) {}
}

pub trait VisitMut {
    fn visit_key_mut(&mut self, _node: &mut types::JsonKey) {}

    fn visit_value_mut(&mut self, node: &mut JsonValue) {
        match node {
            JsonValue::Null(value) => self.visit_null_mut(value),
            JsonValue::Bool(value) => self.visit_bool_mut(value),
            JsonValue::Number(value) => self.visit_number_mut(value),
            JsonValue::String(value) => self.visit_string_mut(value),
            JsonValue::Array(value) => self.visit_array_mut(value),
            JsonValue::Object(value) => self.visit_object_mut(value),
        }
    }

    fn visit_array_mut(&mut self, node: &mut types::Array) {
        for elem in &mut node.elems {
            self.visit_value_mut(elem);
        }
    }

    fn visit_object_mut(&mut self, node: &mut types::Object) {
        for (key, value) in &mut node.elems {
            self.visit_key_mut(key);
            self.visit_value_mut(value);
        }
    }

    fn visit_null_mut(&mut self, _node: &mut types::Null) {}

    fn visit_bool_mut(&mut self, _node: &mut types::Bool) {}

    fn visit_number_mut(&mut self, _node: &mut types::Number) {}

    fn visit_string_mut(&mut self, _node: &mut types::String) {}
}

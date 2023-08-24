//! Deserialize JSON data to a Rust data structure.

use crate::json::{
    error::{Error, ErrorCode, Result},
    types::{Bool, JsonValue, Null, String as JsonString},
};

use super::{
    read::{Read, SliceRead},
    types::{Array, JsonKey, Number, Object},
};

//////////////////////////////////////////////////////////////////////////////

/// A structure that parses JSON spans.
pub struct JsonSpanner<'a> {
    read: SliceRead<'a>,
}

impl<'a> JsonSpanner<'a> {
    /// Create a JSON deserializer from one of the possible serde_json input
    /// sources.
    ///
    /// Typically it is more convenient to use one of these methods instead:
    ///
    ///   - Deserializer::from_str
    ///   - Deserializer::from_slice
    ///   - Deserializer::from_reader
    pub fn new(src: &'a [u8]) -> Self {
        JsonSpanner {
            read: SliceRead::new(src),
        }
    }
}

impl<'a> JsonSpanner<'a> {
    /// The `Deserializer::end` method should be called after a value has been fully deserialized.
    /// This allows the `Deserializer` to validate that the input stream is at the end or that it
    /// only has trailing whitespace.
    pub fn end(&mut self) -> Result<()> {
        match tri!(self.parse_whitespace()) {
            Some(_) => Err(self.peek_error(ErrorCode::TrailingCharacters)),
            None => Ok(()),
        }
    }

    pub(crate) fn peek(&mut self) -> Result<Option<u8>> {
        self.read.peek()
    }

    fn peek_or_null(&mut self) -> Result<u8> {
        Ok(tri!(self.peek()).unwrap_or(b'\x00'))
    }

    fn eat_char(&mut self) {
        self.read.discard();
    }

    fn next_char(&mut self) -> Result<Option<u8>> {
        self.read.next()
    }

    fn next_char_or_null(&mut self) -> Result<u8> {
        Ok(tri!(self.next_char()).unwrap_or(b'\x00'))
    }

    /// Error caused by a byte from next_char().
    #[cold]
    fn error(&self, reason: ErrorCode) -> Error {
        let position = self.read.position();
        Error::syntax(reason, position.line, position.column)
    }

    /// Error caused by a byte from peek().
    #[cold]
    fn peek_error(&self, reason: ErrorCode) -> Error {
        let position = self.read.peek_position();
        Error::syntax(reason, position.line, position.column)
    }

    /// Returns the first non-whitespace byte without consuming it, or `None` if
    /// EOF is encountered.
    fn parse_whitespace(&mut self) -> Result<Option<u8>> {
        loop {
            match tri!(self.peek()) {
                Some(b' ' | b'\n' | b'\t' | b'\r') => {
                    self.eat_char();
                }
                other => {
                    return Ok(other);
                }
            }
        }
    }

    fn parse_ident(&mut self, ident: &[u8]) -> Result<()> {
        for expected in ident {
            match tri!(self.next_char()) {
                None => {
                    return Err(self.error(ErrorCode::EofWhileParsingValue));
                }
                Some(next) => {
                    if next != *expected {
                        return Err(self.error(ErrorCode::ExpectedSomeIdent));
                    }
                }
            }
        }

        Ok(())
    }

    fn parse_object_colon(&mut self) -> Result<()> {
        match tri!(self.parse_whitespace()) {
            Some(b':') => {
                self.eat_char();
                Ok(())
            }
            Some(_) => Err(self.peek_error(ErrorCode::ExpectedColon)),
            None => Err(self.peek_error(ErrorCode::EofWhileParsingObject)),
        }
    }

    fn parse_key(&mut self, start: usize) -> Result<JsonKey> {
        match tri!(self.parse_whitespace()) {
            Some(b'"') => {
                self.eat_char();
                tri!(self.read.ignore_str());
                Ok(JsonKey {
                    range: start..self.read.byte_offset(),
                })
            }
            Some(_) => Err(self.peek_error(ErrorCode::KeyMustBeAString)),
            None => Err(self.peek_error(ErrorCode::EofWhileParsingValue)),
        }
    }

    pub fn parse(&mut self) -> Result<JsonValue> {
        let start = self.read.byte_offset();
        self.parse_value(start)
    }

    fn parse_value(&mut self, start: usize) -> Result<JsonValue> {
        let peek = match tri!(self.parse_whitespace()) {
            Some(b) => b,
            None => {
                return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
            }
        };

        match peek {
            b'n' => {
                self.eat_char();
                tri!(self.parse_ident(b"ull"));
                Ok(JsonValue::Null(Null {
                    range: start..self.read.byte_offset(),
                }))
            }
            b't' => {
                self.eat_char();
                tri!(self.parse_ident(b"rue"));
                Ok(JsonValue::Bool(Bool {
                    range: start..self.read.byte_offset(),
                }))
            }
            b'f' => {
                self.eat_char();
                tri!(self.parse_ident(b"alse"));
                Ok(JsonValue::Bool(Bool {
                    range: start..self.read.byte_offset(),
                }))
            }
            b'-' => {
                self.eat_char();
                self.parse_integer(start)
            }
            b'0'..=b'9' => self.parse_integer(start),
            b'"' => {
                self.eat_char();
                self.parse_string(start)
            }
            b'[' => {
                self.eat_char();
                self.parse_array(start)
            }
            b'{' => {
                // self.scratch.extend(enclosing.take());
                self.eat_char();
                self.parse_object(start)
            }
            _ => Err(self.peek_error(ErrorCode::ExpectedSomeValue)),
        }
    }

    fn parse_object(&mut self, start: usize) -> Result<JsonValue> {
        let mut elems = Vec::new();
        loop {
            match tri!(self.parse_whitespace()) {
                Some(b'}') => {
                    self.eat_char();
                    return Ok(JsonValue::Object(Object {
                        range: start..self.read.byte_offset(),
                        elems,
                    }));
                }
                Some(b',') => {
                    self.eat_char();
                }
                Some(_) => {
                    elems.push(tri!(self.parse_kv(self.read.byte_offset())));
                }
                None => {
                    return Err(self.peek_error(ErrorCode::EofWhileParsingObject));
                }
            }
        }
    }

    fn parse_kv(&mut self, start: usize) -> Result<(JsonKey, JsonValue)> {
        let key = tri!(self.parse_key(start));
        tri!(self.parse_object_colon());
        let value = tri!(self.parse_value(self.read.byte_offset()));
        Ok((key, value))
    }

    fn parse_array(&mut self, start: usize) -> Result<JsonValue> {
        let mut elems = Vec::new();
        loop {
            match tri!(self.parse_whitespace()) {
                Some(b']') => {
                    self.eat_char();
                    return Ok(JsonValue::Array(Array {
                        range: start..self.read.byte_offset(),
                        elems,
                    }));
                }
                Some(b',') => {
                    self.eat_char();
                }
                Some(_) => elems.push(tri!(self.parse_value(self.read.byte_offset()))),
                None => {
                    return Err(self.peek_error(ErrorCode::EofWhileParsingList));
                }
            };
        }
    }

    fn parse_string(&mut self, start: usize) -> Result<JsonValue> {
        tri!(self.read.ignore_str());
        Ok(JsonValue::String(JsonString {
            range: start..self.read.byte_offset(),
        }))
    }

    fn parse_integer(&mut self, start: usize) -> Result<JsonValue> {
        match tri!(self.next_char_or_null()) {
            b'0' => {
                // There can be only one leading '0'.
                if let b'0'..=b'9' = tri!(self.peek_or_null()) {
                    return Err(self.peek_error(ErrorCode::InvalidNumber));
                }
            }
            b'1'..=b'9' => {
                while let b'0'..=b'9' = tri!(self.peek_or_null()) {
                    self.eat_char();
                }
            }
            _ => {
                return Err(self.error(ErrorCode::InvalidNumber));
            }
        }

        match tri!(self.peek_or_null()) {
            b'.' => self.parse_decimal(start),
            b'e' | b'E' => self.parse_exponent(start),
            _ => Ok(JsonValue::Number(Number {
                range: start..self.read.byte_offset(),
            })),
        }
    }

    fn parse_decimal(&mut self, start: usize) -> Result<JsonValue> {
        self.eat_char();

        let mut at_least_one_digit = false;
        while let b'0'..=b'9' = tri!(self.peek_or_null()) {
            self.eat_char();
            at_least_one_digit = true;
        }

        if !at_least_one_digit {
            return Err(self.peek_error(ErrorCode::InvalidNumber));
        }

        match tri!(self.peek_or_null()) {
            b'e' | b'E' => self.parse_exponent(start),
            _ => Ok(JsonValue::Number(Number {
                range: start..self.read.byte_offset(),
            })),
        }
    }

    fn parse_exponent(&mut self, start: usize) -> Result<JsonValue> {
        self.eat_char();

        match tri!(self.peek_or_null()) {
            b'+' | b'-' => self.eat_char(),
            _ => {}
        }

        // Make sure a digit follows the exponent place.
        match tri!(self.next_char_or_null()) {
            b'0'..=b'9' => {}
            _ => {
                return Err(self.error(ErrorCode::InvalidNumber));
            }
        }

        while let b'0'..=b'9' = tri!(self.peek_or_null()) {
            self.eat_char();
        }

        Ok(JsonValue::Number(Number {
            range: start..self.read.byte_offset(),
        }))
    }
}

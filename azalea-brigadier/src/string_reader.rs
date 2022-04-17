use crate::exceptions::{
    builtin_exceptions::BuiltInExceptions, command_syntax_exception::CommandSyntaxException,
};
use std::{rc::Rc, str::FromStr};

#[derive(Clone)]
pub struct StringReader {
    string: String,
    pub cursor: usize,
}

const SYNTAX_ESCAPE: char = '\\';
const SYNTAX_DOUBLE_QUOTE: char = '"';
const SYNTAX_SINGLE_QUOTE: char = '\'';

impl From<String> for StringReader {
    fn from(string: String) -> Self {
        Self { string, cursor: 0 }
    }
}

impl StringReader {
    pub fn string(&self) -> &str {
        &self.string
    }

    pub fn remaining_length(&self) -> usize {
        self.string.len() - self.cursor
    }

    pub fn total_length(&self) -> usize {
        self.string.len()
    }

    pub fn get_read(&self) -> &str {
        &self.string[..self.cursor]
    }

    pub fn remaining(&self) -> &str {
        &self.string[self.cursor..]
    }

    pub fn can_read_length(&self, length: usize) -> bool {
        self.cursor + length <= self.string.len()
    }

    pub fn can_read(&self) -> bool {
        self.can_read_length(1)
    }

    pub fn peek(&self) -> char {
        self.string.chars().nth(self.cursor).unwrap()
    }

    pub fn peek_offset(&self, offset: usize) -> char {
        self.string.chars().nth(self.cursor + offset).unwrap()
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn read(&mut self) -> char {
        let c = self.peek();
        self.cursor += 1;
        c
    }

    pub fn skip(&mut self) {
        self.cursor += 1;
    }

    pub fn is_allowed_number(c: char) -> bool {
        c >= '0' && c <= '9' || c == '.' || c == '-'
    }

    pub fn is_quoted_string_start(c: char) -> bool {
        c == SYNTAX_DOUBLE_QUOTE || c == SYNTAX_SINGLE_QUOTE
    }

    pub fn skip_whitespace(&mut self) {
        while self.can_read() && self.peek().is_whitespace() {
            self.skip();
        }
    }

    pub fn read_int(&mut self) -> Result<i32, CommandSyntaxException> {
        let start = self.cursor;
        while self.can_read() && StringReader::is_allowed_number(self.peek()) {
            self.skip();
        }
        let number = &self.string[start..self.cursor];
        if number.is_empty() {
            return Err(BuiltInExceptions::ReaderExpectedInt.create_with_context(self));
        }
        let result = i32::from_str(number);
        if result.is_err() {
            self.cursor = start;
            return Err(BuiltInExceptions::ReaderInvalidInt {
                value: number.to_string(),
            }
            .create_with_context(self));
        }

        Ok(result.unwrap())
    }

    pub fn read_long(&mut self) -> Result<i64, CommandSyntaxException> {
        let start = self.cursor;
        while self.can_read() && StringReader::is_allowed_number(self.peek()) {
            self.skip();
        }
        let number = &self.string[start..self.cursor];
        if number.is_empty() {
            return Err(BuiltInExceptions::ReaderExpectedLong.create_with_context(self));
        }
        let result = i64::from_str(number);
        if result.is_err() {
            self.cursor = start;
            return Err(BuiltInExceptions::ReaderInvalidLong {
                value: number.to_string(),
            }
            .create_with_context(self));
        }

        Ok(result.unwrap())
    }

    pub fn read_double(&mut self) -> Result<f64, CommandSyntaxException> {
        let start = self.cursor;
        while self.can_read() && StringReader::is_allowed_number(self.peek()) {
            self.skip();
        }
        let number = &self.string[start..self.cursor];
        if number.is_empty() {
            return Err(BuiltInExceptions::ReaderExpectedDouble.create_with_context(self));
        }
        let result = f64::from_str(number);
        if result.is_err() {
            self.cursor = start;
            return Err(BuiltInExceptions::ReaderInvalidDouble {
                value: number.to_string(),
            }
            .create_with_context(self));
        }

        Ok(result.unwrap())
    }

    pub fn read_float(&mut self) -> Result<f32, CommandSyntaxException> {
        let start = self.cursor;
        while self.can_read() && StringReader::is_allowed_number(self.peek()) {
            self.skip();
        }
        let number = &self.string[start..self.cursor];
        if number.is_empty() {
            return Err(BuiltInExceptions::ReaderExpectedFloat.create_with_context(self));
        }
        let result = f32::from_str(number);
        if result.is_err() {
            self.cursor = start;
            return Err(BuiltInExceptions::ReaderInvalidFloat {
                value: number.to_string(),
            }
            .create_with_context(self));
        }

        Ok(result.unwrap())
    }

    pub fn is_allowed_in_unquoted_string(c: char) -> bool {
        c >= '0' && c <= '9'
            || c >= 'A' && c <= 'Z'
            || c >= 'a' && c <= 'z'
            || c == '_'
            || c == '-'
            || c == '.'
            || c == '+'
    }

    pub fn read_unquoted_string(&mut self) -> &str {
        let start = self.cursor;
        while self.can_read() && StringReader::is_allowed_in_unquoted_string(self.peek()) {
            self.skip();
        }
        &self.string[start..self.cursor]
    }

    pub fn read_quoted_string(&mut self) -> Result<String, CommandSyntaxException> {
        if !self.can_read() {
            return Ok(String::new());
        }
        let next = self.peek();
        if !StringReader::is_quoted_string_start(next) {
            return Err(BuiltInExceptions::ReaderExpectedStartOfQuote.create_with_context(self));
        }
        self.skip();
        self.read_string_until(next)
    }

    pub fn read_string_until(
        &mut self,
        terminator: char,
    ) -> Result<String, CommandSyntaxException> {
        let mut result = String::new();
        let mut escaped = false;
        while self.can_read() {
            let c = self.read();
            if escaped {
                if c == terminator || c == SYNTAX_ESCAPE {
                    result.push(c);
                    escaped = false;
                } else {
                    self.cursor -= 1;
                    return Err(BuiltInExceptions::ReaderInvalidEscape { character: c }
                        .create_with_context(self));
                }
            } else if c == SYNTAX_ESCAPE {
                escaped = true;
            } else if c == terminator {
                return Ok(result);
            } else {
                result.push(c);
            }
        }

        return Err(BuiltInExceptions::ReaderExpectedEndOfQuote.create_with_context(self));
    }

    pub fn read_string(&mut self) -> Result<String, CommandSyntaxException> {
        if !self.can_read() {
            return Ok(String::new());
        }
        let next = self.peek();
        if StringReader::is_quoted_string_start(next) {
            self.skip();
            return self.read_string_until(next);
        }
        Ok(self.read_unquoted_string().to_string())
    }

    pub fn read_boolean(&mut self) -> Result<bool, CommandSyntaxException> {
        let start = self.cursor;
        let value = self.read_string()?;
        if value.is_empty() {
            return Err(BuiltInExceptions::ReaderExpectedBool.create_with_context(self));
        }

        if value == "true" {
            return Ok(true);
        } else if value == "false" {
            return Ok(false);
        } else {
            self.cursor = start;
            return Err(BuiltInExceptions::ReaderInvalidBool { value }.create_with_context(self));
        }
    }

    pub fn expect(&mut self, c: char) -> Result<(), CommandSyntaxException> {
        if !self.can_read() || self.peek() != c {
            return Err(
                BuiltInExceptions::ReaderExpectedSymbol { symbol: c }.create_with_context(self)
            );
        }
        self.skip();
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_read() {
        let mut reader = StringReader::from("abc".to_string());
        assert_eq!(reader.can_read(), true);
        reader.skip(); // 'a'
        assert_eq!(reader.can_read(), true);
        reader.skip(); // 'b'
        assert_eq!(reader.can_read(), true);
        reader.skip(); // 'c'
        assert_eq!(reader.can_read(), false);
    }

    #[test]
    fn get_remaining_length() {
        let mut reader = StringReader::from("abc".to_string());
        assert_eq!(reader.remaining_length(), 3);
        reader.cursor = 1;
        assert_eq!(reader.remaining_length(), 2);
        reader.cursor = 2;
        assert_eq!(reader.remaining_length(), 1);
        reader.cursor = 3;
        assert_eq!(reader.remaining_length(), 0);
    }

    #[test]
    fn can_read_length() {
        let reader = StringReader::from("abc".to_string());
        assert_eq!(reader.can_read_length(1), true);
        assert_eq!(reader.can_read_length(2), true);
        assert_eq!(reader.can_read_length(3), true);
        assert_eq!(reader.can_read_length(4), false);
        assert_eq!(reader.can_read_length(5), false);
    }

    #[test]
    fn peek() {
        let mut reader = StringReader::from("abc".to_string());
        assert_eq!(reader.peek(), 'a');
        assert_eq!(reader.cursor(), 0);
        reader.cursor = 2;
        assert_eq!(reader.peek(), 'c');
        assert_eq!(reader.cursor(), 2);
    }

    #[test]
    fn peek_length() {
        let mut reader = StringReader::from("abc".to_string());
        assert_eq!(reader.peek_offset(0), 'a');
        assert_eq!(reader.peek_offset(2), 'c');
        assert_eq!(reader.cursor(), 0);
        reader.cursor = 1;
        assert_eq!(reader.peek_offset(1), 'c');
        assert_eq!(reader.cursor(), 1);
    }

    #[test]
    fn read() {
        let mut reader = StringReader::from("abc".to_string());
        assert_eq!(reader.read(), 'a');
        assert_eq!(reader.read(), 'b');
        assert_eq!(reader.read(), 'c');
        assert_eq!(reader.cursor(), 3);
    }

    #[test]
    fn skip() {
        let mut reader = StringReader::from("abc".to_string());
        reader.skip();
        assert_eq!(reader.cursor(), 1);
    }

    #[test]
    fn get_remaining() {
        let mut reader = StringReader::from("Hello!".to_string());
        assert_eq!(reader.remaining(), "Hello!");
        reader.cursor = 3;
        assert_eq!(reader.remaining(), "lo!");
        reader.cursor = 6;
        assert_eq!(reader.remaining(), "");
    }

    #[test]
    fn get_read() {
        let mut reader = StringReader::from("Hello!".to_string());
        assert_eq!(reader.get_read(), "");
        reader.cursor = 3;
        assert_eq!(reader.get_read(), "Hel");
        reader.cursor = 6;
        assert_eq!(reader.get_read(), "Hello!");
    }

    #[test]
    fn skip_whitespace_none() {
        let mut reader = StringReader::from("Hello!".to_string());
        reader.skip_whitespace();
        assert_eq!(reader.cursor(), 0);
    }

    #[test]
    fn skip_whitespace_mixed() {
        let mut reader = StringReader::from(" \t \t\nHello!".to_string());
        reader.skip_whitespace();
        assert_eq!(reader.cursor(), 5);
    }

    #[test]
    fn skip_whitespace_empty() {
        let mut reader = StringReader::from("".to_string());
        reader.skip_whitespace();
        assert_eq!(reader.cursor(), 0);
    }

    #[test]
    fn read_unquoted_string() {
        let mut reader = StringReader::from("hello world".to_string());
        assert_eq!(reader.read_unquoted_string(), "hello");
        assert_eq!(reader.get_read(), "hello");
        assert_eq!(reader.remaining(), " world");
    }

    #[test]
    fn read_unquoted_string_empty() {
        let mut reader = StringReader::from("".to_string());
        assert_eq!(reader.read_unquoted_string(), "");
        assert_eq!(reader.get_read(), "");
        assert_eq!(reader.remaining(), "");
    }

    #[test]
    fn read_unquoted_string_empty_with_remaining() {
        let mut reader = StringReader::from(" hello world".to_string());
        assert_eq!(reader.read_unquoted_string(), "");
        assert_eq!(reader.get_read(), "");
        assert_eq!(reader.remaining(), " hello world");
    }

    #[test]
    fn read_quoted_string() {
        let mut reader = StringReader::from("\"hello world\"".to_string());
        assert_eq!(reader.read_quoted_string().unwrap(), "hello world");
        assert_eq!(reader.get_read(), "\"hello world\"");
        assert_eq!(reader.remaining(), "");
    }

    #[test]
    fn read_single_quoted_string() {
        let mut reader = StringReader::from("'hello world'".to_string());
        assert_eq!(reader.read_quoted_string().unwrap(), "hello world");
        assert_eq!(reader.get_read(), "'hello world'");
        assert_eq!(reader.remaining(), "");
    }

    #[test]
    fn read_mixed_quoted_string_double_inside_single() {
        let mut reader = StringReader::from("'hello \"world\"'".to_string());
        assert_eq!(reader.read_quoted_string().unwrap(), "hello \"world\"");
        assert_eq!(reader.get_read(), "'hello \"world\"'");
        assert_eq!(reader.remaining(), "");
    }

    #[test]
    fn read_mixed_quoted_string_single_inside_double() {
        let mut reader = StringReader::from("\"hello 'world'\"".to_string());
        assert_eq!(reader.read_quoted_string().unwrap(), "hello 'world'");
        assert_eq!(reader.get_read(), "\"hello 'world'\"");
        assert_eq!(reader.remaining(), "");
    }

    #[test]
    fn read_quoted_string_empty_quoted() {
        let mut reader = StringReader::from("".to_string());
        assert_eq!(reader.read_quoted_string().unwrap(), "");
        assert_eq!(reader.get_read(), "");
        assert_eq!(reader.remaining(), "");
    }

    #[test]
    fn read_quoted_string_empty_quoted_with_remaining() {
        let mut reader = StringReader::from("\"\" hello world".to_string());
        assert_eq!(reader.read_quoted_string().unwrap(), "");
        assert_eq!(reader.get_read(), "\"\"");
        assert_eq!(reader.remaining(), " hello world");
    }

    #[test]
    fn read_quoted_string_with_escaped_quote() {
        let mut reader = StringReader::from("\"hello \\\"world\\\"\"".to_string());
        assert_eq!(reader.read_quoted_string().unwrap(), "hello \"world\"");
        assert_eq!(reader.get_read(), "\"hello \\\"world\\\"\"");
        assert_eq!(reader.remaining(), "");
    }

    #[test]
    fn read_quoted_string_with_escaped_escapes() {
        let mut reader = StringReader::from("\"\\\\o/\"".to_string());
        assert_eq!(reader.read_quoted_string().unwrap(), "\\o/");
        assert_eq!(reader.get_read(), "\"\\\\o/\"");
        assert_eq!(reader.remaining(), "");
    }

    #[test]
    fn read_quoted_string_with_remaining() {
        let mut reader = StringReader::from("\"hello world\" foo bar".to_string());
        assert_eq!(reader.read_quoted_string().unwrap(), "hello world");
        assert_eq!(reader.get_read(), "\"hello world\"");
        assert_eq!(reader.remaining(), " foo bar");
    }

    #[test]
    fn read_quoted_string_with_immediate_remaining() {
        let mut reader = StringReader::from("\"hello world\"foo bar".to_string());
        assert_eq!(reader.read_quoted_string().unwrap(), "hello world");
        assert_eq!(reader.get_read(), "\"hello world\"");
        assert_eq!(reader.remaining(), "foo bar");
    }

    #[test]
    fn read_quoted_string_no_open() {
        let mut reader = StringReader::from("hello world\"".to_string());
        let result = reader.read_quoted_string();
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.get_type(), &BuiltInExceptions::ReaderExpectedStartOfQuote);
            assert_eq!(e.cursor(), Some(0));
        }
    }

    #[test]
    fn read_quoted_string_no_close() {
        let mut reader = StringReader::from("\"hello world".to_string());
        let result = reader.read_quoted_string();
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.get_type(), &BuiltInExceptions::ReaderExpectedEndOfQuote);
            assert_eq!(e.cursor(), Some(12));
        }
    }

    #[test]
    fn read_quoted_string_invalid_escape() {
        let mut reader = StringReader::from("\"hello\\nworld\"".to_string());
        let result = reader.read_quoted_string();
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(
                e.get_type(),
                &BuiltInExceptions::ReaderInvalidEscape { character: 'n' }
            );
            assert_eq!(e.cursor(), Some(7));
        }
    }

    #[test]
    fn read_quoted_string_invalid_quote_escape() {
        let mut reader = StringReader::from("'hello\\\"\'world".to_string());
        let result = reader.read_quoted_string();
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(
                e.get_type(),
                &BuiltInExceptions::ReaderInvalidEscape { character: '"' }
            );
            assert_eq!(e.cursor(), Some(7));
        }
    }

    #[test]
    fn read_string_no_quotes() {
        let mut reader = StringReader::from("hello world".to_string());
        assert_eq!(reader.read_string().unwrap(), "hello");
        assert_eq!(reader.get_read(), "hello");
        assert_eq!(reader.remaining(), " world");
    }

    #[test]
    fn read_string_single_quotes() {
        let mut reader = StringReader::from("'hello world'".to_string());
        assert_eq!(reader.read_string().unwrap(), "hello world");
        assert_eq!(reader.get_read(), "'hello world'");
        assert_eq!(reader.remaining(), "");
    }

    #[test]
    fn read_string_double_quotes() {
        let mut reader = StringReader::from("\"hello world\"".to_string());
        assert_eq!(reader.read_string().unwrap(), "hello world");
        assert_eq!(reader.get_read(), "\"hello world\"");
        assert_eq!(reader.remaining(), "");
    }

    #[test]
    fn read_int() {
        let mut reader = StringReader::from("1234567890".to_string());
        assert_eq!(reader.read_int().unwrap(), 1234567890);
        assert_eq!(reader.get_read(), "1234567890");
        assert_eq!(reader.remaining(), "");
    }

    #[test]
    fn read_int_negative() {
        let mut reader = StringReader::from("-1234567890".to_string());
        assert_eq!(reader.read_int().unwrap(), -1234567890);
        assert_eq!(reader.get_read(), "-1234567890");
        assert_eq!(reader.remaining(), "");
    }

    #[test]
    fn read_int_invalid() {
        let mut reader = StringReader::from("12.34".to_string());
        let result = reader.read_int();
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(
                e.get_type(),
                &BuiltInExceptions::ReaderInvalidInt {
                    value: "12.34".to_string()
                }
            );
            assert_eq!(e.cursor(), Some(0));
        }
    }

    #[test]
    fn read_int_none() {
        let mut reader = StringReader::from("".to_string());
        let result = reader.read_int();
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.get_type(), &BuiltInExceptions::ReaderExpectedInt);
            assert_eq!(e.cursor(), Some(0));
        }
    }

    #[test]
    fn read_int_with_remaining() {
        let mut reader = StringReader::from("1234567890 foo bar".to_string());
        assert_eq!(reader.read_int().unwrap(), 1234567890);
        assert_eq!(reader.get_read(), "1234567890");
        assert_eq!(reader.remaining(), " foo bar");
    }

    #[test]
    fn read_int_with_remaining_immediate() {
        let mut reader = StringReader::from("1234567890foo bar".to_string());
        assert_eq!(reader.read_int().unwrap(), 1234567890);
        assert_eq!(reader.get_read(), "1234567890");
        assert_eq!(reader.remaining(), "foo bar");
    }

    #[test]
    fn read_long() {
        let mut reader = StringReader::from("1234567890".to_string());
        assert_eq!(reader.read_long().unwrap(), 1234567890);
        assert_eq!(reader.get_read(), "1234567890");
        assert_eq!(reader.remaining(), "");
    }

    #[test]
    fn read_long_negative() {
        let mut reader = StringReader::from("-1234567890".to_string());
        assert_eq!(reader.read_long().unwrap(), -1234567890);
        assert_eq!(reader.get_read(), "-1234567890");
        assert_eq!(reader.remaining(), "");
    }

    #[test]
    fn read_long_invalid() {
        let mut reader = StringReader::from("12.34".to_string());
        let result = reader.read_long();
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(
                e.get_type(),
                &BuiltInExceptions::ReaderInvalidLong {
                    value: "12.34".to_string()
                }
            );
            assert_eq!(e.cursor(), Some(0));
        }
    }

    #[test]
    fn read_long_none() {
        let mut reader = StringReader::from("".to_string());
        let result = reader.read_long();
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.get_type(), &BuiltInExceptions::ReaderExpectedLong);
            assert_eq!(e.cursor(), Some(0));
        }
    }

    #[test]
    fn read_long_with_remaining() {
        let mut reader = StringReader::from("1234567890 foo bar".to_string());
        assert_eq!(reader.read_long().unwrap(), 1234567890);
        assert_eq!(reader.get_read(), "1234567890");
        assert_eq!(reader.remaining(), " foo bar");
    }

    #[test]
    fn read_long_with_remaining_immediate() {
        let mut reader = StringReader::from("1234567890foo bar".to_string());
        assert_eq!(reader.read_long().unwrap(), 1234567890);
        assert_eq!(reader.get_read(), "1234567890");
        assert_eq!(reader.remaining(), "foo bar");
    }

    #[test]
    fn read_double() {
        let mut reader = StringReader::from("123".to_string());
        assert_eq!(reader.read_double().unwrap(), 123.0);
        assert_eq!(reader.get_read(), "123");
        assert_eq!(reader.remaining(), "");
    }

    #[test]
    fn read_double_with_decimal() {
        let mut reader = StringReader::from("12.34".to_string());
        assert_eq!(reader.read_double().unwrap(), 12.34);
        assert_eq!(reader.get_read(), "12.34");
        assert_eq!(reader.remaining(), "");
    }

    #[test]
    fn read_double_negative() {
        let mut reader = StringReader::from("-123".to_string());
        assert_eq!(reader.read_double().unwrap(), -123.0);
        assert_eq!(reader.get_read(), "-123");
        assert_eq!(reader.remaining(), "");
    }

    #[test]
    fn read_double_invalid() {
        let mut reader = StringReader::from("12.34.56".to_string());
        let result = reader.read_double();
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(
                e.get_type(),
                &BuiltInExceptions::ReaderInvalidDouble {
                    value: "12.34.56".to_string()
                }
            );
            assert_eq!(e.cursor(), Some(0));
        }
    }

    #[test]
    fn read_double_none() {
        let mut reader = StringReader::from("".to_string());
        let result = reader.read_double();
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.get_type(), &BuiltInExceptions::ReaderExpectedDouble);
            assert_eq!(e.cursor(), Some(0));
        }
    }

    #[test]
    fn read_double_with_remaining() {
        let mut reader = StringReader::from("12.34 foo bar".to_string());
        assert_eq!(reader.read_double().unwrap(), 12.34);
        assert_eq!(reader.get_read(), "12.34");
        assert_eq!(reader.remaining(), " foo bar");
    }

    #[test]
    fn read_double_with_remaining_immediate() {
        let mut reader = StringReader::from("12.34foo bar".to_string());
        assert_eq!(reader.read_double().unwrap(), 12.34);
        assert_eq!(reader.get_read(), "12.34");
        assert_eq!(reader.remaining(), "foo bar");
    }

    #[test]
    fn read_float() {
        let mut reader = StringReader::from("123".to_string());
        assert_eq!(reader.read_float().unwrap(), 123.0f32);
        assert_eq!(reader.get_read(), "123");
        assert_eq!(reader.remaining(), "");
    }

    #[test]
    fn read_float_with_decimal() {
        let mut reader = StringReader::from("12.34".to_string());
        assert_eq!(reader.read_float().unwrap(), 12.34f32);
        assert_eq!(reader.get_read(), "12.34");
        assert_eq!(reader.remaining(), "");
    }

    #[test]
    fn read_float_negative() {
        let mut reader = StringReader::from("-123".to_string());
        assert_eq!(reader.read_float().unwrap(), -123.0f32);
        assert_eq!(reader.get_read(), "-123");
        assert_eq!(reader.remaining(), "");
    }

    #[test]
    fn read_float_invalid() {
        let mut reader = StringReader::from("12.34.56".to_string());
        let result = reader.read_float();
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(
                e.get_type(),
                &BuiltInExceptions::ReaderInvalidFloat {
                    value: "12.34.56".to_string()
                }
            );
            assert_eq!(e.cursor(), Some(0));
        }
    }

    #[test]
    fn read_float_none() {
        let mut reader = StringReader::from("".to_string());
        let result = reader.read_float();
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.get_type(), &BuiltInExceptions::ReaderExpectedFloat);
            assert_eq!(e.cursor(), Some(0));
        }
    }

    #[test]
    fn read_float_with_remaining() {
        let mut reader = StringReader::from("12.34 foo bar".to_string());
        assert_eq!(reader.read_float().unwrap(), 12.34f32);
        assert_eq!(reader.get_read(), "12.34");
        assert_eq!(reader.remaining(), " foo bar");
    }

    #[test]
    fn read_float_with_remaining_immediate() {
        let mut reader = StringReader::from("12.34foo bar".to_string());
        assert_eq!(reader.read_float().unwrap(), 12.34f32);
        assert_eq!(reader.get_read(), "12.34");
        assert_eq!(reader.remaining(), "foo bar");
    }

    #[test]
    fn expect_correct() {
        let mut reader = StringReader::from("abc".to_string());
        reader.expect('a');
        assert_eq!(reader.cursor(), 1);
    }

    #[test]
    fn expect_incorrect() {
        let mut reader = StringReader::from("bcd".to_string());
        let result = reader.expect('a');
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(
                e.get_type(),
                &BuiltInExceptions::ReaderExpectedSymbol { symbol: 'a' }
            );
            assert_eq!(e.cursor(), Some(0));
        }
    }

    #[test]
    fn expect_none() {
        let mut reader = StringReader::from("".to_string());
        let result = reader.expect('a');
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(
                e.get_type(),
                &BuiltInExceptions::ReaderExpectedSymbol { symbol: 'a' }
            );
            assert_eq!(e.cursor(), Some(0));
        }
    }

    #[test]
    fn read_boolean_correct() {
        let mut reader = StringReader::from("true".to_string());
        assert_eq!(reader.read_boolean().unwrap(), true);
        assert_eq!(reader.get_read(), "true");
    }

    #[test]
    fn read_boolean_incorrect() {
        let mut reader = StringReader::from("tuesday".to_string());
        let result = reader.read_boolean();
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(
                e.get_type(),
                &BuiltInExceptions::ReaderInvalidBool {
                    value: "tuesday".to_string()
                }
            );
            assert_eq!(e.cursor(), Some(0));
        }
    }

    #[test]
    fn read_boolean_none() {
        let mut reader = StringReader::from("".to_string());
        let result = reader.read_boolean();
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.get_type(), &BuiltInExceptions::ReaderExpectedBool);
            assert_eq!(e.cursor(), Some(0));
        }
    }
}

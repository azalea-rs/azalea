use crate::exceptions::{BuiltInExceptions, CommandSyntaxException};
use std::str::FromStr;

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
impl From<&str> for StringReader {
    fn from(string: &str) -> Self {
        Self {
            string: string.to_string(),
            cursor: 0,
        }
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
        ('0'..='9').contains(&c) || c == '.' || c == '-'
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
        ('0'..='9').contains(&c)
            || ('A'..='Z').contains(&c)
            || ('a'..='z').contains(&c)
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

        Err(BuiltInExceptions::ReaderExpectedEndOfQuote.create_with_context(self))
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
            Ok(true)
        } else if value == "false" {
            Ok(false)
        } else {
            self.cursor = start;
            Err(BuiltInExceptions::ReaderInvalidBool { value }.create_with_context(self))
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

use crate::immutable_string_reader::ImmutableStringReader;
use std::str::FromStr;

#[derive(Clone)]
struct StringReader<'a> {
    pub string: &'a str,
    pub cursor: usize,
}

const SYNTAX_ESCAPE: char = '\\';
const SYNTAX_DOUBLE_QUOTE: char = '"';
const SYNTAX_SINGLE_QUOTE: char = '\'';

impl<'a> From<&'a str> for &StringReader<'a> {
    fn from(string: &'a str) -> &StringReader<'a> {
        &StringReader { string, cursor: 0 }
    }
}

impl ImmutableStringReader for StringReader<'_> {
    fn remaining_length(&self) -> usize {
        self.string.len() - self.cursor
    }

    fn total_length(&self) -> usize {
        self.string.len()
    }

    fn get_read(&self) -> &str {
        &self.string[self.cursor..]
    }

    fn remaining(&self) -> &str {
        &self.string[self.cursor..]
    }

    fn can_read_length(&self, length: usize) -> bool {
        self.cursor + length <= self.string.len()
    }

    fn can_read(&self) -> bool {
        self.can_read_length(1)
    }

    fn peek(&self) -> char {
        self.string.chars().nth(self.cursor).unwrap()
    }

    fn peek_offset(&self, offset: usize) -> char {
        self.string.chars().nth(self.cursor + offset).unwrap()
    }
}

impl StringReader<'_> {
    fn read(&mut self) -> char {
        let c = self.peek();
        self.cursor += 1;
        c
    }

    fn skip(&mut self) {
        self.cursor += 1;
    }

    fn is_allowed_number(c: char) -> bool {
        c >= '0' && c <= '9' || c == '.' || c == '-'
    }

    fn is_quoted_string_start(c: char) -> bool {
        c == SYNTAX_DOUBLE_QUOTE || c == SYNTAX_SINGLE_QUOTE
    }

    fn skip_whitespace(&mut self) {
        while self.can_read() && self.peek().is_whitespace() {
            self.skip();
        }
    }

    fn read_int(&self) -> Result<(), CommandSyntaxException> {
        let start = self.cursor;
        while self.can_read() && StringReader::<'_>::is_allowed_number(self.peek()) {
            self.skip();
        }
        let number = &self.string[start..self.cursor];
        if number.is_empty() {
            return Err(CommandSyntaxException::BUILT_IN_EXCEPTIONS
                .reader_expected_int()
                .create_with_context(self));
        }
        let result = i32::from_str(number);
        if result.is_err() {
            self.cursor = start;
            return Err(CommandSyntaxException::BUILT_IN_EXCEPTIONS
                .reader_invalid_int()
                .create_with_context(self, number));
        }

        Ok(())
    }

    fn read_long(&self) -> Result<(), CommandSyntaxException> {
        let start = self.cursor;
        while self.can_read() && StringReader::<'_>::is_allowed_number(self.peek()) {
            self.skip();
        }
        let number = &self.string[start..self.cursor];
        if number.is_empty() {
            return Err(CommandSyntaxException::BUILT_IN_EXCEPTIONS
                .reader_expected_long()
                .create_with_context(self));
        }
        let result = i64::from_str(number);
        if result.is_err() {
            self.cursor = start;
            return Err(CommandSyntaxException::BUILT_IN_EXCEPTIONS
                .reader_invalid_long()
                .create_with_context(self, number));
        }

        Ok(())
    }

    fn read_double(&self) -> Result<(), CommandSyntaxException> {
        let start = self.cursor;
        while self.can_read() && StringReader::<'_>::is_allowed_number(self.peek()) {
            self.skip();
        }
        let number = &self.string[start..self.cursor];
        if number.is_empty() {
            return Err(CommandSyntaxException::BUILT_IN_EXCEPTIONS
                .reader_expected_double()
                .create_with_context(self));
        }
        let result = f64::from_str(number);
        if result.is_err() {
            self.cursor = start;
            return Err(CommandSyntaxException::BUILT_IN_EXCEPTIONS
                .reader_invalid_double()
                .create_with_context(self, number));
        }

        Ok(())
    }

    fn read_float(&self) -> Result<(), CommandSyntaxException> {
        let start = self.cursor;
        while self.can_read() && StringReader::<'_>::is_allowed_number(self.peek()) {
            self.skip();
        }
        let number = &self.string[start..self.cursor];
        if number.is_empty() {
            return Err(CommandSyntaxException::BUILT_IN_EXCEPTIONS
                .reader_expected_float()
                .create_with_context(self));
        }
        let result = f32::from_str(number);
        if result.is_err() {
            self.cursor = start;
            return Err(CommandSyntaxException::BUILT_IN_EXCEPTIONS
                .reader_invalid_float()
                .create_with_context(self, number));
        }

        Ok(())
    }

    fn is_allowed_in_unquoted_string(c: char) -> bool {
        c >= '0' && c <= '9'
            || c >= 'A' && c <= 'Z'
            || c >= 'a' && c <= 'z'
            || c == '_'
            || c == '-'
            || c == '.'
            || c == '+'
    }

    fn read_unquoted_string(&self) -> &str {
        let start = self.cursor;
        while self.can_read() && StringReader::<'_>::is_allowed_in_unquoted_string(self.peek()) {
            self.skip();
        }
        &self.string[start..self.cursor]
    }

    fn read_quoted_string(&self) -> Result<&str, CommandSyntaxException> {
        if !self.can_read() {
            return "";
        }
        let next = self.peek();
        if !StringReader::<'_>::is_quoted_string_start(next) {
            return Err(CommandSyntaxException::BUILT_IN_EXCEPTIONS
                .reader_expected_start_of_quote()
                .create_with_context(self));
        }
        self.skip();
        self.read_string_until(next)
    }

    fn read_string_until(&self, terminator: char) -> Result<String, CommandSynatxException> {
        let result = String::new();
        let mut escaped = false;
        while self.can_read() {
            let c = self.read();
            if escaped {
                if c == terminator || c == SYNTAX_ESCAPE {
                    result.push(c);
                    escaped = false;
                } else {
                    self.cursor -= 1;
                    return Err(CommandSyntaxException::BUILT_IN_EXCEPTIONS
                        .reader_invalid_escape()
                        .create_with_context(self, c));
                }
            } else if c == SYNTAX_ESCAPE {
                escaped = true;
            } else if c == terminator {
                return Ok(result);
            } else {
                result.push(c);
            }
        }

        Err(CommandSyntaxException::BUILT_IN_EXCEPTIONS
            .reader_expected_end_of_quote()
            .create_with_context(self))
    }

    fn read_string(&self) -> Result<String, CommandSyntaxException> {
        // if (!canRead()) {
        //     return "";
        // }
        // final char next = peek();
        // if (isQuotedStringStart(next)) {
        //     skip();
        //     return readStringUntil(next);
        // }
        // return readUnquotedString();
        if !self.can_read() {
            return Ok(String::new());
        }
        let next = self.peek();
        if StringReader::<'_>::is_quoted_string_start(next) {
            self.skip();
            return self.read_string_until(next);
        }
        Ok(self.read_unquoted_string().to_string())
    }

	fn read_boolean(&self) -> Result<bool, CommandSyntaxException> {
		let start = self.cursor;
		let value = self.read_string()?;
		if value.is_empty() {
			return Err(CommandSyntaxException::BUILT_IN_EXCEPTIONS
				.reader_expected_bool()
				.create_with_context(self));
		}

		if value == "true" {
			return Ok(true);
		} else if value == "false" {
			return Ok(false);
		} else {
			self.cursor = start;
			return Err(CommandSyntaxException::BUILT_IN_EXCEPTIONS
				.reader_invalid_bool()
				.create_with_context(self, value));
		}
	}

	fn expect(&self, c: char) -> Result<(), CommandSyntaxException> {
		if !self.can_read() || self.peek() != c {
			return Err(CommandSyntaxException::BUILT_IN_EXCEPTIONS
				.reader_expected_symbol()
				.create_with_context(self, c));
		}
		self.skip();
	}

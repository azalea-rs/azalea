use azalea_brigadier::{exceptions::BuiltInExceptions, string_reader::StringReader};

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
    reader.expect('a').unwrap();
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

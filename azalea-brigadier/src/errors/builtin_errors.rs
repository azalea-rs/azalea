use std::fmt;

use super::command_syntax_error::CommandSyntaxError;
use crate::string_reader::StringReader;

#[derive(Clone, PartialEq)]
pub enum BuiltInError {
    DoubleTooSmall { found: f64, min: f64 },
    DoubleTooBig { found: f64, max: f64 },

    FloatTooSmall { found: f32, min: f32 },
    FloatTooBig { found: f32, max: f32 },

    IntegerTooSmall { found: i32, min: i32 },
    IntegerTooBig { found: i32, max: i32 },

    LongTooSmall { found: i64, min: i64 },
    LongTooBig { found: i64, max: i64 },

    LiteralIncorrect { expected: String },

    ReaderExpectedStartOfQuote,
    ReaderExpectedEndOfQuote,
    ReaderInvalidEscape { character: char },
    ReaderInvalidBool { value: String },
    ReaderInvalidInt { value: String },
    ReaderExpectedInt,
    ReaderInvalidLong { value: String },
    ReaderExpectedLong,
    ReaderInvalidDouble { value: String },
    ReaderExpectedDouble,
    ReaderInvalidFloat { value: String },
    ReaderExpectedFloat,
    ReaderExpectedBool,
    ReaderExpectedSymbol { symbol: char },

    DispatcherUnknownCommand,
    DispatcherUnknownArgument,
    DispatcherExpectedArgumentSeparator,
    DispatcherParseException { message: String },
}

impl fmt::Debug for BuiltInError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuiltInError::DoubleTooSmall { found, min } => {
                write!(f, "Double must not be less than {min}, found {found}")
            }
            BuiltInError::DoubleTooBig { found, max } => {
                write!(f, "Double must not be more than {max}, found {found}")
            }

            BuiltInError::FloatTooSmall { found, min } => {
                write!(f, "Float must not be less than {min}, found {found}")
            }
            BuiltInError::FloatTooBig { found, max } => {
                write!(f, "Float must not be more than {max}, found {found}")
            }

            BuiltInError::IntegerTooSmall { found, min } => {
                write!(f, "Integer must not be less than {min}, found {found}")
            }
            BuiltInError::IntegerTooBig { found, max } => {
                write!(f, "Integer must not be more than {max}, found {found}")
            }

            BuiltInError::LongTooSmall { found, min } => {
                write!(f, "Long must not be less than {min}, found {found}")
            }
            BuiltInError::LongTooBig { found, max } => {
                write!(f, "Long must not be more than {max}, found {found}")
            }

            BuiltInError::LiteralIncorrect { expected } => {
                write!(f, "Expected literal {expected}")
            }

            BuiltInError::ReaderExpectedStartOfQuote => {
                write!(f, "Expected quote to start a string")
            }
            BuiltInError::ReaderExpectedEndOfQuote => {
                write!(f, "Unclosed quoted string")
            }
            BuiltInError::ReaderInvalidEscape { character } => {
                write!(f, "Invalid escape sequence '{character}' in quoted string")
            }
            BuiltInError::ReaderInvalidBool { value } => {
                write!(
                    f,
                    "Invalid bool, expected true or false but found '{value}'"
                )
            }
            BuiltInError::ReaderInvalidInt { value } => {
                write!(f, "Invalid Integer '{value}'")
            }
            BuiltInError::ReaderExpectedInt => {
                write!(f, "Expected Integer")
            }
            BuiltInError::ReaderInvalidLong { value } => {
                write!(f, "Invalid long '{value}'")
            }
            BuiltInError::ReaderExpectedLong => {
                write!(f, "Expected long")
            }
            BuiltInError::ReaderInvalidDouble { value } => {
                write!(f, "Invalid double '{value}'")
            }
            BuiltInError::ReaderExpectedDouble => {
                write!(f, "Expected double")
            }
            BuiltInError::ReaderInvalidFloat { value } => {
                write!(f, "Invalid Float '{value}'")
            }
            BuiltInError::ReaderExpectedFloat => {
                write!(f, "Expected Float")
            }
            BuiltInError::ReaderExpectedBool => {
                write!(f, "Expected bool")
            }
            BuiltInError::ReaderExpectedSymbol { symbol } => {
                write!(f, "Expected '{symbol}'")
            }

            BuiltInError::DispatcherUnknownCommand => {
                write!(f, "Unknown command")
            }
            BuiltInError::DispatcherUnknownArgument => {
                write!(f, "Incorrect argument for command")
            }
            BuiltInError::DispatcherExpectedArgumentSeparator => {
                write!(
                    f,
                    "Expected whitespace to end one argument, but found trailing data"
                )
            }
            BuiltInError::DispatcherParseException { message } => {
                write!(f, "Could not parse command: {message}")
            }
        }
    }
}

impl BuiltInError {
    pub fn create(self) -> CommandSyntaxError {
        let message = format!("{self:?}");
        CommandSyntaxError::create(self, message)
    }

    pub fn create_with_context(self, reader: &StringReader) -> CommandSyntaxError {
        let message = format!("{self:?}");
        CommandSyntaxError::new(self, message, reader.string(), reader.cursor())
    }
}

use std::fmt;

use crate::string_reader::StringReader;

use super::command_syntax_exception::CommandSyntaxException;

#[derive(Clone, PartialEq)]
pub enum BuiltInExceptions {
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

impl fmt::Debug for BuiltInExceptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuiltInExceptions::DoubleTooSmall { found, min } => {
                write!(f, "Double must not be less than {min}, found {found}")
            }
            BuiltInExceptions::DoubleTooBig { found, max } => {
                write!(f, "Double must not be more than {max}, found {found}")
            }

            BuiltInExceptions::FloatTooSmall { found, min } => {
                write!(f, "Float must not be less than {min}, found {found}")
            }
            BuiltInExceptions::FloatTooBig { found, max } => {
                write!(f, "Float must not be more than {max}, found {found}")
            }

            BuiltInExceptions::IntegerTooSmall { found, min } => {
                write!(f, "Integer must not be less than {min}, found {found}")
            }
            BuiltInExceptions::IntegerTooBig { found, max } => {
                write!(f, "Integer must not be more than {max}, found {found}")
            }

            BuiltInExceptions::LongTooSmall { found, min } => {
                write!(f, "Long must not be less than {min}, found {found}")
            }
            BuiltInExceptions::LongTooBig { found, max } => {
                write!(f, "Long must not be more than {max}, found {found}")
            }

            BuiltInExceptions::LiteralIncorrect { expected } => {
                write!(f, "Expected literal {expected}")
            }

            BuiltInExceptions::ReaderExpectedStartOfQuote => {
                write!(f, "Expected quote to start a string")
            }
            BuiltInExceptions::ReaderExpectedEndOfQuote => {
                write!(f, "Unclosed quoted string")
            }
            BuiltInExceptions::ReaderInvalidEscape { character } => {
                write!(f, "Invalid escape sequence '{character}' in quoted string")
            }
            BuiltInExceptions::ReaderInvalidBool { value } => {
                write!(
                    f,
                    "Invalid bool, expected true or false but found '{value}'"
                )
            }
            BuiltInExceptions::ReaderInvalidInt { value } => {
                write!(f, "Invalid Integer '{value}'")
            }
            BuiltInExceptions::ReaderExpectedInt => {
                write!(f, "Expected Integer")
            }
            BuiltInExceptions::ReaderInvalidLong { value } => {
                write!(f, "Invalid long '{value}'")
            }
            BuiltInExceptions::ReaderExpectedLong => {
                write!(f, "Expected long")
            }
            BuiltInExceptions::ReaderInvalidDouble { value } => {
                write!(f, "Invalid double '{value}'")
            }
            BuiltInExceptions::ReaderExpectedDouble => {
                write!(f, "Expected double")
            }
            BuiltInExceptions::ReaderInvalidFloat { value } => {
                write!(f, "Invalid Float '{value}'")
            }
            BuiltInExceptions::ReaderExpectedFloat => {
                write!(f, "Expected Float")
            }
            BuiltInExceptions::ReaderExpectedBool => {
                write!(f, "Expected bool")
            }
            BuiltInExceptions::ReaderExpectedSymbol { symbol } => {
                write!(f, "Expected '{symbol}'")
            }

            BuiltInExceptions::DispatcherUnknownCommand => {
                write!(f, "Unknown command")
            }
            BuiltInExceptions::DispatcherUnknownArgument => {
                write!(f, "Incorrect argument for command")
            }
            BuiltInExceptions::DispatcherExpectedArgumentSeparator => {
                write!(
                    f,
                    "Expected whitespace to end one argument, but found trailing data"
                )
            }
            BuiltInExceptions::DispatcherParseException { message } => {
                write!(f, "Could not parse command: {message}")
            }
        }
    }
}

impl BuiltInExceptions {
    pub fn create(self) -> CommandSyntaxException {
        let message = format!("{self:?}");
        CommandSyntaxException::create(self, message)
    }

    pub fn create_with_context(self, reader: &StringReader) -> CommandSyntaxException {
        let message = format!("{self:?}");
        CommandSyntaxException::new(self, message, reader.string(), reader.cursor())
    }
}

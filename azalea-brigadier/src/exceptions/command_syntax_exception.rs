use super::builtin_exceptions::BuiltInExceptions;
use std::{
    cmp,
    fmt::{self, Write},
};

#[derive(Clone, PartialEq)]
pub struct CommandSyntaxException {
    pub type_: BuiltInExceptions,
    message: String,
    input: Option<String>,
    cursor: Option<usize>,
}

const CONTEXT_AMOUNT: usize = 10;

impl CommandSyntaxException {
    pub fn new(type_: BuiltInExceptions, message: String, input: &str, cursor: usize) -> Self {
        Self {
            type_,
            message,
            input: Some(input.to_string()),
            cursor: Some(cursor),
        }
    }

    pub fn create(type_: BuiltInExceptions, message: String) -> Self {
        Self {
            type_,
            message,
            input: None,
            cursor: None,
        }
    }

    pub fn message(&self) -> String {
        let mut message = self.message.clone();
        let context = self.context();
        if let Some(context) = context {
            write!(
                message,
                " at position {}: {}",
                self.cursor.unwrap_or(usize::MAX),
                context
            )
            .unwrap();
        }
        message
    }

    pub fn raw_message(&self) -> &String {
        &self.message
    }

    pub fn context(&self) -> Option<String> {
        if let Some(input) = &self.input {
            if let Some(cursor) = self.cursor {
                let mut builder = String::new();
                let cursor = cmp::min(input.len(), cursor);

                if cursor > CONTEXT_AMOUNT {
                    builder.push_str("...");
                }

                builder.push_str(
                    &input
                        [(cmp::max(0, cursor as isize - CONTEXT_AMOUNT as isize) as usize)..cursor],
                );
                builder.push_str("<--[HERE]");

                return Some(builder);
            }
        }
        None
    }

    pub fn get_type(&self) -> &BuiltInExceptions {
        &self.type_
    }

    pub fn input(&self) -> &Option<String> {
        &self.input
    }

    pub fn cursor(&self) -> Option<usize> {
        self.cursor
    }
}

impl fmt::Debug for CommandSyntaxException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

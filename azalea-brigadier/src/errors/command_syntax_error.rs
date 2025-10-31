use std::{
    cmp,
    fmt::{self, Debug, Write as _},
};

use super::builtin_errors::BuiltInError;

#[derive(Clone, PartialEq)]
pub struct CommandSyntaxError {
    kind: BuiltInError,
    message: String,
    input: Option<String>,
    cursor: Option<usize>,
}

const CONTEXT_AMOUNT: usize = 10;

impl CommandSyntaxError {
    #[must_use]
    pub fn new(kind: BuiltInError, message: String, input: &str, cursor: usize) -> Self {
        Self {
            kind,
            message,
            input: Some(input.to_owned()),
            cursor: Some(cursor),
        }
    }

    #[must_use]
    pub const fn create(kind: BuiltInError, message: String) -> Self {
        Self {
            kind,
            message,
            input: None,
            cursor: None,
        }
    }

    #[must_use]
    pub fn message(&self) -> String {
        let mut message = self.message.clone();
        let context = self.context();
        if let Some(context) = context {
            write!(
                message,
                " at position {}: {context}",
                self.cursor.unwrap_or(usize::MAX)
            )
            .unwrap();
        }
        message
    }

    #[must_use]
    pub const fn raw_message(&self) -> &String {
        &self.message
    }

    #[must_use]
    pub fn context(&self) -> Option<String> {
        if let Some(input) = &self.input
            && let Some(cursor) = self.cursor
        {
            let mut builder = String::new();
            let cursor = cmp::min(input.len(), cursor);

            if cursor > CONTEXT_AMOUNT {
                builder.push_str("...");
            }

            builder.push_str(
                &input[(cmp::max(0, cursor as isize - CONTEXT_AMOUNT as isize) as usize)..cursor],
            );
            builder.push_str("<--[HERE]");

            return Some(builder);
        }
        None
    }

    #[must_use]
    pub const fn kind(&self) -> &BuiltInError {
        &self.kind
    }

    #[must_use]
    pub const fn input(&self) -> &Option<String> {
        &self.input
    }

    #[must_use]
    pub const fn cursor(&self) -> Option<usize> {
        self.cursor
    }
}

impl Debug for CommandSyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

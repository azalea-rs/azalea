use std::{
    cmp,
    fmt::{self, Debug, Write},
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
    pub fn new(kind: BuiltInError, message: String, input: &str, cursor: usize) -> Self {
        Self {
            kind,
            message,
            input: Some(input.to_owned()),
            cursor: Some(cursor),
        }
    }

    pub fn create(kind: BuiltInError, message: String) -> Self {
        Self {
            kind,
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
                " at position {}: {context}",
                self.cursor.unwrap_or(usize::MAX)
            )
            .unwrap();
        }
        message
    }

    pub fn raw_message(&self) -> &String {
        &self.message
    }

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

    pub fn kind(&self) -> &BuiltInError {
        &self.kind
    }

    pub fn input(&self) -> &Option<String> {
        &self.input
    }

    pub fn cursor(&self) -> Option<usize> {
        self.cursor
    }
}

impl Debug for CommandSyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

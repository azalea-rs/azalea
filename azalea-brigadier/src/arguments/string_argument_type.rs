use std::{any::Any, sync::Arc};

use super::ArgumentType;
use crate::{context::CommandContext, errors::CommandSyntaxError, string_reader::StringReader};

pub enum StringArgument {
    /// Match up until the next space.
    SingleWord,
    /// Same as single word unless the argument is wrapped in quotes, in which
    /// case it can contain spaces.
    QuotablePhrase,
    /// Match the rest of the input.
    GreedyPhrase,
}

impl ArgumentType for StringArgument {
    fn parse(&self, reader: &mut StringReader) -> Result<Arc<dyn Any>, CommandSyntaxError> {
        let result = match self {
            StringArgument::SingleWord => reader.read_unquoted_string().to_owned(),
            StringArgument::QuotablePhrase => reader.read_string()?,
            StringArgument::GreedyPhrase => {
                let text = reader.remaining().to_owned();
                reader.cursor = reader.total_length();
                text
            }
        };
        Ok(Arc::new(result))
    }

    fn examples(&self) -> Vec<String> {
        match self {
            StringArgument::SingleWord => vec!["word", "words_with_underscores"],
            StringArgument::QuotablePhrase => vec!["\"quoted phrase\"", "word", "\"\""],
            StringArgument::GreedyPhrase => vec!["word", "words with spaces", "\"and symbols\""],
        }
        .into_iter()
        .map(|s| s.to_owned())
        .collect()
    }
}

/// Match up until the next space.
pub fn word() -> impl ArgumentType {
    StringArgument::SingleWord
}
/// Same as single word unless the argument is wrapped in quotes, in which case
/// it can contain spaces.
pub fn string() -> impl ArgumentType {
    StringArgument::QuotablePhrase
}
/// Match the rest of the input.
pub fn greedy_string() -> impl ArgumentType {
    StringArgument::GreedyPhrase
}
pub fn get_string<S>(context: &CommandContext<S>, name: &str) -> Option<String> {
    context
        .argument(name)
        .unwrap()
        .downcast_ref::<String>()
        .cloned()
}

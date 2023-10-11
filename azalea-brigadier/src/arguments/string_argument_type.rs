use std::{any::Any, sync::Arc};

use crate::{
    context::CommandContext, exceptions::CommandSyntaxException, string_reader::StringReader,
};

use super::ArgumentType;

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
    fn parse(&self, reader: &mut StringReader) -> Result<Arc<dyn Any>, CommandSyntaxException> {
        let result = match self {
            StringArgument::SingleWord => reader.read_unquoted_string().to_string(),
            StringArgument::QuotablePhrase => reader.read_string()?,
            StringArgument::GreedyPhrase => {
                let text = reader.remaining().to_string();
                reader.cursor = reader.total_length();
                text
            }
        };
        Ok(Arc::new(result))
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

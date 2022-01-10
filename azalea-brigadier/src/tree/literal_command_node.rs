use crate::{
    context::{command_context::CommandContext, command_context_builder::CommandContextBuilder},
    exceptions::{
        builtin_exceptions::BuiltInExceptions, command_syntax_exception::CommandSyntaxException,
    },
    string_reader::StringReader,
    suggestion::{suggestions::Suggestions, suggestions_builder::SuggestionsBuilder},
};

use super::command_node::{BaseCommandNode, CommandNode};

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct LiteralCommandNode<S> {
    literal: String,
    literal_lowercase: String,
    // Since Rust doesn't have extending, we put the struct this is extending as the "base" field
    pub base: BaseCommandNode<S>,
}

impl<S> LiteralCommandNode<S> {
    pub fn literal(&self) -> &String {
        &self.literal
    }

    pub fn parse(&self, reader: StringReader) -> i32 {
        let start = reader.get_cursor();
        if reader.can_read(self.literal.len()) {
            let end = start + self.literal.len();
            if reader.get_string()[start..end].eq(&self.literal) {
                reader.set_cursor(end);
                if !reader.can_read() || reader.peek() == ' ' {
                    return end as i32;
                } else {
                    reader.set_cursor(start);
                }
            }
        }
        -1
    }
}

impl<S> CommandNode<S> for LiteralCommandNode<S> {
    fn name(&self) -> &str {
        &self.literal
    }

    fn parse(
        &self,
        reader: StringReader,
        context_builder: CommandContextBuilder<S>,
    ) -> Result<(), CommandSyntaxException> {
        let start = reader.get_cursor();
        let end = self.parse(reader);
        if end > -1 {
            return Ok(());
        }

        Err(BuiltInExceptions::LiteralIncorrect {
            expected: self.literal(),
        }
        .create_with_context(reader))
    }

    fn list_suggestions(
        &self,
        context: CommandContext<S>,
        builder: SuggestionsBuilder,
    ) -> Result<Suggestions, CommandSyntaxException> {
        if self
            .literal_lowercase
            .starts_with(&builder.remaining_lowercase())
        {
            builder.suggest(self.literal())
        } else {
            Suggestions::empty()
        }
    }

    fn is_valid_input(&self, input: &str) -> bool {
        self.parse(StringReader::from(input)) > -1
    }

    fn usage_text(&self) -> &str {
        self.literal
    }

    fn create_builder(&self) -> LiteralArgumentBuilder<S> {
        let builder = LiteralArgumentBuilder::literal(self.literal());
        builder.requires(self.requirement());
        builder.forward(self.redirect(), self.redirect_modifier(), self.is_fork());
        if self.command().is_some() {
            builder.executes(self.command().unwrap());
        }
        builder
    }
}

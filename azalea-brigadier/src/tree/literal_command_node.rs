use crate::{
    arguments::argument_type::ArgumentType,
    builder::{
        argument_builder::ArgumentBuilder, literal_argument_builder::LiteralArgumentBuilder,
    },
    context::{command_context::CommandContext, command_context_builder::CommandContextBuilder},
    exceptions::{
        builtin_exceptions::BuiltInExceptions, command_syntax_exception::CommandSyntaxException,
    },
    immutable_string_reader::ImmutableStringReader,
    string_reader::StringReader,
    suggestion::{suggestions::Suggestions, suggestions_builder::SuggestionsBuilder},
};
use std::fmt::Debug;

use super::command_node::{BaseCommandNode, CommandNodeTrait};

#[derive(Debug, Clone)]
pub struct LiteralCommandNode<'a, S> {
    literal: String,
    literal_lowercase: String,
    // Since Rust doesn't have extending, we put the struct this is extending as the "base" field
    pub base: BaseCommandNode<'a, S>,
}

impl<'a, S> LiteralCommandNode<'a, S> {
    pub fn new(literal: String, base: BaseCommandNode<'a, S>) -> Self {
        let literal_lowercase = literal.to_lowercase();
        Self {
            literal,
            literal_lowercase,
            base,
        }
    }

    pub fn literal(&self) -> &String {
        &self.literal
    }

    pub fn parse(&self, reader: &mut StringReader) -> i32 {
        let start = reader.cursor();
        if reader.can_read_length(self.literal.len()) {
            let end = start + self.literal.len();
            if reader.string()[start..end].eq(&self.literal) {
                reader.cursor = end;
                if !reader.can_read() || reader.peek() == ' ' {
                    return end as i32;
                } else {
                    reader.cursor = start;
                }
            }
        }
        -1
    }
}

impl<S> CommandNodeTrait<S> for LiteralCommandNode<'_, S> {
    fn name(&self) -> &str {
        &self.literal
    }

    fn parse(
        &self,
        reader: &mut StringReader<'_>,
        context_builder: CommandContextBuilder<S>,
    ) -> Result<(), CommandSyntaxException> {
        let start = reader.cursor();
        let end = self.parse(reader);
        if end > -1 {
            return Ok(());
        }

        Err(BuiltInExceptions::LiteralIncorrect {
            expected: self.literal().to_string(),
        }
        .create_with_context(reader))
    }

    fn list_suggestions(
        &self,
        context: CommandContext<S>,
        builder: &SuggestionsBuilder,
    ) -> Result<Suggestions, CommandSyntaxException> {
        if self
            .literal_lowercase
            .starts_with(&builder.remaining_lowercase())
        {
            Ok(builder.suggest(self.literal()).build())
        } else {
            Ok(Suggestions::default())
        }
    }

    fn is_valid_input(&self, input: &str) -> bool {
        self.parse(&mut StringReader::from(input)) > -1
    }

    fn usage_text(&self) -> &str {
        &self.literal
    }

    fn create_builder(&self) -> Box<dyn ArgumentBuilder<S>> {
        let mut builder = LiteralArgumentBuilder::literal(self.literal().to_string());
        builder.base.requires(&self.base().requirement);
        builder.base.forward(
            self.base.redirect(),
            self.base.redirect_modifier(),
            self.base.is_fork(),
        );
        if self.command().is_some() {
            builder.executes(self.command().unwrap());
        }
        builder
    }

    fn get_examples(&self) -> Vec<String> {
        todo!()
    }
}

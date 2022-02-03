use crate::{
    arguments::argument_type::ArgumentType,
    builder::{
        argument_builder::ArgumentBuilder, literal_argument_builder::LiteralArgumentBuilder,
    },
    command::Command,
    context::{command_context::CommandContext, command_context_builder::CommandContextBuilder},
    exceptions::{
        builtin_exceptions::BuiltInExceptions, command_syntax_exception::CommandSyntaxException,
    },
    immutable_string_reader::ImmutableStringReader,
    redirect_modifier::RedirectModifier,
    string_reader::StringReader,
    suggestion::{suggestions::Suggestions, suggestions_builder::SuggestionsBuilder},
};
use std::{collections::HashMap, fmt::Debug};

use super::{
    argument_command_node::ArgumentCommandNode,
    command_node::{BaseCommandNode, CommandNodeTrait},
};

#[derive(Debug, Clone)]
pub struct LiteralCommandNode<S> {
    literal: String,
    literal_lowercase: String,

    children: HashMap<String, Box<dyn CommandNodeTrait<S>>>,
    literals: HashMap<String, LiteralCommandNode<S>>,
    arguments: HashMap<String, ArgumentCommandNode<S>>,
    pub requirement: Box<dyn Fn(&S) -> bool>,
    redirect: Option<Box<dyn CommandNodeTrait<S>>>,
    modifier: Option<Box<dyn RedirectModifier<S>>>,
    forks: bool,
    pub command: Option<Box<dyn Command<S>>>,
}

impl<S> LiteralCommandNode<S> {
    pub fn new(literal: String) -> Self {
        let literal_lowercase = literal.to_lowercase();
        Self {
            literal,
            literal_lowercase,
            ..Default::default()
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

impl<S> CommandNodeTrait<S> for LiteralCommandNode<S> {
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

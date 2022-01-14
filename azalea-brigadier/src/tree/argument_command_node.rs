use std::{
    any::Any,
    fmt::{Display, Formatter},
};

use crate::{
    arguments::argument_type::ArgumentType,
    builder::required_argument_builder::RequiredArgumentBuilder,
    context::{
        command_context::CommandContext, command_context_builder::CommandContextBuilder,
        parsed_argument::ParsedArgument,
    },
    exceptions::command_syntax_exception::CommandSyntaxException,
    immutable_string_reader::ImmutableStringReader,
    string_reader::StringReader,
    suggestion::{
        suggestion_provider::SuggestionProvider, suggestions::Suggestions,
        suggestions_builder::SuggestionsBuilder,
    },
};

use super::command_node::{BaseCommandNode, CommandNode};

const USAGE_ARGUMENT_OPEN: &str = "<";
const USAGE_ARGUMENT_CLOSE: &str = ">";

#[derive(Clone)]
pub struct ArgumentCommandNode<'a, S> {
    name: String,
    type_: Box<dyn ArgumentType<Into = dyn Any>>,
    custom_suggestions: Option<&'a dyn SuggestionProvider<S>>,
    // custom_suggestions: &'a dyn SuggestionProvider<S>,
    // Since Rust doesn't have extending, we put the struct this is extending as the "base" field
    pub base: BaseCommandNode<'a, S>,
}

impl<S> ArgumentCommandNode<'_, S> {
    fn get_type(&self) -> &dyn ArgumentType<Into = dyn Any> {
        self.type_
    }

    fn custom_suggestions(&self) -> Option<&dyn SuggestionProvider<S>> {
        self.custom_suggestions
    }
}

impl<'a, S> CommandNode<S> for ArgumentCommandNode<'a, S>
where
    S: Clone,
{
    fn name(&self) -> &str {
        &self.name
    }

    fn parse(
        &self,
        reader: &mut StringReader,
        context_builder: CommandContextBuilder<S>,
    ) -> Result<(), CommandSyntaxException> {
        // final int start = reader.getCursor();
        // final T result = type.parse(reader);
        // final ParsedArgument<S> parsed = new ParsedArgument<>(start, reader.getCursor(), result);

        // contextBuilder.withArgument(name, parsed);
        // contextBuilder.withNode(this, parsed.getRange());

        let start = reader.cursor();
        let result = self.get_type().parse(reader)?;
        let parsed = ParsedArgument::new(start, reader.get_cursor(), result);

        context_builder.with_argument(&self.name, parsed);
        context_builder.with_node(self, parsed.get_range());

        Ok(())
    }

    fn list_suggestions(
        &self,
        context: CommandContext<S>,
        builder: &SuggestionsBuilder,
    ) -> Result<Suggestions, CommandSyntaxException> {
        if self.custom_suggestions.is_none() {
            self.get_type().list_suggestions(context, builder)
        } else {
            self.custom_suggestions.get_suggestions(context, builder)
        }
    }

    fn is_valid_input(&self, input: &str) -> bool {
        let reader = StringReader::new(input);
        let result = self.get_type().parse(reader);
        if result.is_ok() {
            return !reader.can_read() || reader.peek() == ' ';
        } else {
            return false;
        }
    }

    fn usage_text(&self) -> &str {
        USAGE_ARGUMENT_OPEN + self.name + USAGE_ARGUMENT_CLOSE
    }

    fn create_builder(&self) -> RequiredArgumentBuilder<S> {
        let builder = RequiredArgumentBuilder::argument(&self.name, &self.type_);
        builder.requires(self.base.get_requirement());
        builder.forward(
            self.base.get_redirect(),
            self.base.get_redirect_modifier(),
            self.base.is_fork(),
        );
        builder.suggests(self.custom_suggestions());
        if self.base.get_command() != None {
            builder.executes(self.base.get_command().unwrap());
        }
        builder
    }

    fn get_examples(&self) -> Vec<String> {
        self.type_.get_examples()
    }

    fn base(&self) -> &BaseCommandNode<S> {
        &self.base
    }
}

impl<S> Display for ArgumentCommandNode<'_, S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<argument {}: {}>", self.name, self.type_)
    }
}

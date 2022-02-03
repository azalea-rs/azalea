use std::{
    any::Any,
    collections::HashMap,
    fmt::{Debug, Display, Formatter},
};

use crate::{
    arguments::argument_type::ArgumentType,
    builder::required_argument_builder::RequiredArgumentBuilder,
    command::Command,
    context::{
        command_context::CommandContext, command_context_builder::CommandContextBuilder,
        parsed_argument::ParsedArgument,
    },
    exceptions::command_syntax_exception::CommandSyntaxException,
    immutable_string_reader::ImmutableStringReader,
    redirect_modifier::RedirectModifier,
    string_reader::StringReader,
    suggestion::{
        suggestion_provider::SuggestionProvider, suggestions::Suggestions,
        suggestions_builder::SuggestionsBuilder,
    },
};

use super::{
    command_node::{BaseCommandNode, CommandNodeTrait},
    literal_command_node::LiteralCommandNode,
    root_command_node::RootCommandNode,
};

const USAGE_ARGUMENT_OPEN: &str = "<";
const USAGE_ARGUMENT_CLOSE: &str = ">";

pub struct ArgumentCommandNode<S> {
    name: String,
    type_: Box<dyn ArgumentType<Into = dyn Any>>,
    custom_suggestions: Option<Box<dyn SuggestionProvider<S>>>,

    children: HashMap<String, Box<dyn CommandNodeTrait<S>>>,
    literals: HashMap<String, LiteralCommandNode<S>>,
    arguments: HashMap<String, ArgumentCommandNode<S>>,
    pub requirement: Box<dyn Fn(&S) -> bool>,
    redirect: Option<Box<dyn CommandNodeTrait<S>>>,
    modifier: Option<Box<dyn RedirectModifier<S>>>,
    forks: bool,
    pub command: Option<Box<dyn Command<S>>>,
}

impl<S> ArgumentCommandNode<S> {
    fn get_type(&self) -> &dyn ArgumentType<Into = dyn Any> {
        &*self.type_
    }

    fn custom_suggestions(&self) -> &Option<Box<dyn SuggestionProvider<S>>> {
        &self.custom_suggestions
    }
}

impl<S> CommandNodeTrait<S> for ArgumentCommandNode<S> {
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

    fn redirect_modifier(&self) -> Option<&dyn RedirectModifier<S>> {
        self.modifier.as_ref().map(|modifier| modifier.as_ref())
    }

    fn can_use(&self, source: S) -> bool {
        (self.requirement)(&source)
    }

    fn add_child(&self, node: &Box<dyn CommandNodeTrait<S>>) -> Result<(), String> {
        let dynamic_node = node as &dyn Any;
        if dynamic_node.is::<RootCommandNode<S>>() {
            return Err(String::from(
                "Cannot add a RootCommandNode as a child to any other CommandNode",
            ));
        }

        let mut child = self.children.get(node.name());
        if let Some(child) = child {
            // We've found something to merge onto
            if let Some(command) = node.base().command() {
                child.base_mut().command = Some(*command);
            }
            for grandchild in node.base().children().values() {
                child.base_mut().add_child(&*grandchild)?;
            }
            Ok(())
        } else {
            self.children.insert(node.name().to_string(), *node);

            if let Some(dynamic_node) = dynamic_node.downcast_ref::<LiteralCommandNode<S>>() {
                self.literals.insert(node.name().to_string(), *dynamic_node);
            } else if let Some(dynamic_node) = dynamic_node.downcast_ref::<ArgumentCommandNode<S>>()
            {
                self.arguments
                    .insert(node.name().to_string(), *dynamic_node);
            }
            Ok(())
        }
    }
}

impl<S> Display for ArgumentCommandNode<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<argument {}: {}>", self.name, self.type_)
    }
}

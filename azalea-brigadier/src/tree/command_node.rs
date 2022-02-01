use super::{
    argument_command_node::ArgumentCommandNode, literal_command_node::LiteralCommandNode,
    root_command_node::RootCommandNode,
};
use crate::{
    arguments::argument_type::ArgumentType,
    builder::argument_builder::ArgumentBuilder,
    command::Command,
    context::{command_context::CommandContext, command_context_builder::CommandContextBuilder},
    exceptions::command_syntax_exception::CommandSyntaxException,
    redirect_modifier::RedirectModifier,
    string_reader::StringReader,
    suggestion::{suggestions::Suggestions, suggestions_builder::SuggestionsBuilder},
};
use std::ops::Deref;
use std::{any::Any, collections::HashMap, fmt::Debug};

enum CommandNodeEnum<'a, S> {
    Literal(LiteralCommandNode<'a, S>),
    Argument(ArgumentCommandNode<'a, S>),
    Root(RootCommandNode<'a, S>),
}

impl<S> Deref for CommandNodeEnum<'_, S> {
    type Target = dyn CommandNodeTrait<S>;

    fn deref(&self) -> &Self::Target {
        match self {
            CommandNodeEnum::Literal(node) => node,
            CommandNodeEnum::Argument(node) => node,
            CommandNodeEnum::Root(node) => node,
        }
    }
}

impl<S> From<LiteralCommandNode<'_, S>> for CommandNodeEnum<'_, S> {
    fn from(node: LiteralCommandNode<'_, S>) -> Self {
        CommandNodeEnum::Literal(node)
    }
}

impl<S> From<ArgumentCommandNode<'_, S>> for CommandNodeEnum<'_, S> {
    fn from(node: ArgumentCommandNode<'_, S>) -> Self {
        CommandNodeEnum::Argument(node)
    }
}

impl<S> From<RootCommandNode<'_, S>> for CommandNodeEnum<'_, S> {
    fn from(node: RootCommandNode<'_, S>) -> Self {
        CommandNodeEnum::Root(node)
    }
}

impl<S> CommandNodeEnum<'_, S> {
    fn redirect_modifier(&self) -> Option<&dyn RedirectModifier<S>> {
        (*self).modifier.as_ref().map(|modifier| modifier.as_ref())
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
pub struct BaseCommandNode<'a, S> {
    children: HashMap<String, Box<dyn CommandNodeTrait<S>>>,
    literals: HashMap<String, LiteralCommandNode<'a, S>>,
    arguments: HashMap<String, ArgumentCommandNode<'a, S>>,
    pub requirement: Box<dyn Fn(&S) -> bool>,
    redirect: Option<Box<dyn CommandNodeTrait<S>>>,
    modifier: Option<Box<dyn RedirectModifier<S>>>,
    forks: bool,
    pub command: Option<Box<dyn Command<S>>>,
}

// impl<S> Clone for BaseCommandNode<'_, S> {
//     fn clone(&self) -> Self {
//         Self {
//             children: self.children.clone(),
//             literals: self.literals.clone(),
//             arguments: self.arguments.clone(),
//             requirement: self.requirement.clone(),
//             redirect: self.redirect.clone(),
//             modifier: self.modifier.clone(),
//             forks: self.forks.clone(),
//             command: self.command.clone(),
//         }
//     }
// }

impl<S> Debug for BaseCommandNode<'_, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BaseCommandNode")
            .field("children", &self.children)
            .field("literals", &self.literals)
            .field("arguments", &self.arguments)
            .field("requirement", &self.requirement)
            .field("redirect", &self.redirect)
            .field("modifier", &self.modifier)
            .field("forks", &self.forks)
            .field("command", &self.command)
            .finish()
    }
}

impl<S> Default for BaseCommandNode<'_, S> {
    fn default() -> Self {
        Self {
            children: HashMap::new(),
            literals: HashMap::new(),
            arguments: HashMap::new(),
            requirement: Box::new(|_| true),
            redirect: None,
            modifier: None,
            forks: false,
            command: None,
        }
    }
}

pub trait CommandNodeTrait<S> {
    fn name(&self) -> &str;
    fn usage_text(&self) -> &str;
    fn parse(
        &self,
        reader: &mut StringReader,
        context_builder: CommandContextBuilder<S>,
    ) -> Result<(), CommandSyntaxException>;
    fn list_suggestions(
        &self,
        context: CommandContext<S>,
        builder: &SuggestionsBuilder,
    ) -> Result<Suggestions, CommandSyntaxException>;
    fn is_valid_input(&self, input: &str) -> bool;
    fn create_builder(&self) -> Box<dyn ArgumentBuilder<S>>;
    fn get_examples(&self) -> Vec<String>;
}

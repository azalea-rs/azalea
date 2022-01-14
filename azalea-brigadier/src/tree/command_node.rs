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
use dyn_clonable::*;
use std::{any::Any, collections::HashMap, fmt::Debug};

pub struct BaseCommandNode<'a, S> {
    children: HashMap<String, Box<dyn CommandNode<S>>>,
    literals: HashMap<String, LiteralCommandNode<'a, S>>,
    arguments: HashMap<String, ArgumentCommandNode<'a, S>>,
    requirement: Box<dyn Fn(&S) -> bool>,
    redirect: Option<Box<dyn CommandNode<S>>>,
    modifier: Option<Box<dyn RedirectModifier<S>>>,
    forks: bool,
    command: Option<Box<dyn Command<S>>>,
}

impl<S> BaseCommandNode<'_, S> {
    pub fn command(&self) -> &Option<Box<dyn Command<S>>> {
        &self.command
    }

    pub fn children(&self) -> &HashMap<String, Box<dyn CommandNode<S>>> {
        &self.children
    }

    pub fn child(&self, name: &str) -> Option<&dyn CommandNode<S>> {
        self.children.get(name).map(|child| child.as_ref())
    }

    pub fn redirect(&self) -> Option<&dyn CommandNode<S>> {
        self.redirect.as_ref().map(|redirect| redirect.as_ref())
    }

    pub fn redirect_modifier(&self) -> Option<&dyn RedirectModifier<S>> {
        self.modifier.as_ref().map(|modifier| modifier.as_ref())
    }

    pub fn can_use(&self, source: S) -> bool {
        (self.requirement)(&source)
    }

    // public void addChild(final CommandNode<S> node) {
    //     if (node instanceof RootCommandNode) {
    //         throw new UnsupportedOperationException("Cannot add a RootCommandNode as a child to any other CommandNode");
    //     }

    //     final CommandNode<S> child = children.get(node.getName());
    //     if (child != null) {
    //         // We've found something to merge onto
    //         if (node.getCommand() != null) {
    //             child.command = node.getCommand();
    //         }
    //         for (final CommandNode<S> grandchild : node.getChildren()) {
    //             child.addChild(grandchild);
    //         }
    //     } else {
    //         children.put(node.getName(), node);
    //         if (node instanceof LiteralCommandNode) {
    //             literals.put(node.getName(), (LiteralCommandNode<S>) node);
    //         } else if (node instanceof ArgumentCommandNode) {
    //             arguments.put(node.getName(), (ArgumentCommandNode<S, ?>) node);
    //         }
    //     }
    // }

    pub fn add_child(&self, node: &dyn CommandNode<S>) -> Result<(), String> {
        if (&node as &dyn Any).is::<RootCommandNode<S>>() {
            return Err(String::from(
                "Cannot add a RootCommandNode as a child to any other CommandNode",
            ));
        }

        let child = self.children.get(node.name());
        if let Some(child) = child {
            // We've found something to merge onto
            if let Some(command) = node.base.command() {
                child.command = Some(command);
            }
            for grandchild in node.children() {
                child.add_child(grandchild)?;
            }
        } else {
            self.children.insert(node.name().to_string(), node);
            if let Some(literal) =
                &node.clone_boxed() as &dyn Any as &dyn Any as &LiteralCommandNode<S>
            {
                self.literals
                    .insert(node.name().to_string(), literal.clone_boxed());
            } else if let Some(argument) =
                &node.clone_boxed() as &dyn Any as &dyn Any as &ArgumentCommandNode<S>
            {
                self.arguments
                    .insert(node.name().to_string(), argument.clone_boxed());
            }
        }
    }
}

impl<S> Clone for BaseCommandNode<'_, S> {
    fn clone(&self) -> Self {
        Self {
            children: self.children.clone(),
            literals: self.literals.clone(),
            arguments: self.arguments.clone(),
            requirement: self.requirement.clone(),
            redirect: self.redirect.clone(),
            modifier: self.modifier.clone(),
            forks: self.forks.clone(),
            command: self.command.clone(),
        }
    }
}

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

pub trait CommandNode<S> {
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
    fn create_builder(&self) -> dyn ArgumentBuilder<S, dyn Any>;
    fn get_examples(&self) -> Vec<String>;
    fn base(&self) -> &BaseCommandNode<S>;
}

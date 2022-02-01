use super::argument_builder::{ArgumentBuilder, BaseArgumentBuilder};
use crate::{
    arguments::argument_type::ArgumentType,
    command::Command,
    redirect_modifier::RedirectModifier,
    tree::{
        command_node::CommandNodeTrait, literal_command_node::LiteralCommandNode,
        root_command_node::RootCommandNode,
    },
};
use std::fmt::Debug;

pub struct LiteralArgumentBuilder<'a, S>
where
    ,
{
    arguments: RootCommandNode<'a, S>,
    command: Option<Box<dyn Command<S>>>,
    requirement: Box<dyn Fn(&S) -> bool>,
    target: Option<Box<dyn CommandNodeTrait<S>>>,
    modifier: Option<Box<dyn RedirectModifier<S>>>,
    forks: bool,
    literal: String,
}

impl<'a, S> LiteralArgumentBuilder<'a, S>
where
    ,
{
    pub fn new(literal: String) -> Self {
        Self {
            literal,
            arguments: RootCommandNode::new(),
            command: None,
            requirement: Box::new(|_| true),
            target: None,
            modifier: None,
            forks: false,
        }
    }

    pub fn literal(name: String) -> Self {
        Self::new(name)
    }
}

impl<'a, S> ArgumentBuilder<S> for LiteralArgumentBuilder<'a, S>
where
    ,
{
    fn build(self) -> Box<dyn CommandNodeTrait<S>> {
        let result = LiteralCommandNode::new(self.literal, self.base.build());

        for argument in self.base.arguments() {
            result.add_child(argument);
        }

        Box::new(result)
    }
}

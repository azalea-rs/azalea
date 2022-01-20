use super::argument_builder::{ArgumentBuilder, BaseArgumentBuilder};
use crate::{
    arguments::argument_type::ArgumentType,
    command::Command,
    redirect_modifier::RedirectModifier,
    tree::{
        command_node::CommandNode, literal_command_node::LiteralCommandNode,
        root_command_node::RootCommandNode,
    },
};

pub struct LiteralArgumentBuilder<'a, S> {
    arguments: RootCommandNode<'a, S>,
    command: Option<Box<dyn Command<S>>>,
    requirement: Box<dyn Fn(&S) -> bool>,
    target: Option<Box<dyn CommandNode<S>>>,
    modifier: Option<Box<dyn RedirectModifier<S>>>,
    forks: bool,

    literal: String,
}

impl<'a, S> LiteralArgumentBuilder<'a, S> {
    pub fn new(literal: String) -> Self {
        Self {
            literal,
            base: BaseArgumentBuilder::default(),
        }
    }

    pub fn literal(name: String) -> Self {
        Self::new(name)
    }
}

impl<'a, S, T> ArgumentBuilder<S, T> for LiteralArgumentBuilder<'a, S>
where
    T: ArgumentBuilder<S, T>,
{
    fn build(self) -> Box<dyn CommandNode<S>> {
        let result = LiteralCommandNode::new(self.literal, self.base.build());

        for argument in self.base.arguments() {
            result.add_child(argument);
        }

        Box::new(result)
    }
}

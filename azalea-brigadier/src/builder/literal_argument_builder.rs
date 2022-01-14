use crate::{
    arguments::argument_type::ArgumentType,
    tree::{command_node::CommandNode, literal_command_node::LiteralCommandNode},
};

use super::argument_builder::{ArgumentBuilder, BaseArgumentBuilder};

pub struct LiteralArgumentBuilder<'a, S> {
    literal: String,

    pub base: BaseArgumentBuilder<'a, S>,
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

        for argument in self.base.arguments {
            result.add_child(argument);
        }

        result
    }
}

use crate::{
    arguments::argument_type::ArgumentType, tree::literal_command_node::LiteralCommandNode,
};

use super::argument_builder::BaseArgumentBuilder;

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

    pub fn build(self) -> LiteralCommandNode<'a, S> {
        let result = LiteralCommandNode::new(self.literal, self.base);

        for argument in self.base.arguments {
            result.add_child(argument);
        }

        result
    }
}

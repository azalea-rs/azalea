use crate::{
    arguments::argument_type::{ArgumentType, Types},
    tree::literal_command_node::LiteralCommandNode,
};

use super::argument_builder::BaseArgumentBuilder;

pub struct LiteralArgumentBuilder<'a, S, T>
where
    T: ArgumentType<dyn Types>,
{
    literal: String,

    pub base: BaseArgumentBuilder<'a, S, T>,
}

impl<'a, S, T> LiteralArgumentBuilder<'a, S, T>
where
    T: ArgumentType<dyn Types>,
{
    pub fn new(literal: String) -> Self {
        Self {
            literal,
            base: BaseArgumentBuilder::default(),
        }
    }

    pub fn literal(name: String) -> Self {
        Self::new(name)
    }

    pub fn build(self) -> LiteralCommandNode<'a, S, T> {
        let result = LiteralCommandNode::new(self.literal, self.base);

        for argument in self.base.arguments {
            result.add_child(argument);
        }

        result
    }
}

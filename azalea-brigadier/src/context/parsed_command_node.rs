use super::string_range::StringRange;
use crate::tree::command_node::CommandNode;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct ParsedCommandNode<'a, S, T> {
    node: &'a dyn CommandNode<S, T>,
    range: StringRange,
}

impl<S, T> ParsedCommandNode<'_, S, T> {
    fn new(node: &dyn CommandNode<S, T>, range: StringRange) -> Self {
        Self { node, range }
    }

    fn node(&self) -> &dyn CommandNode<S, T> {
        &self.node
    }

    fn range(&self) -> &StringRange {
        &self.range
    }
}

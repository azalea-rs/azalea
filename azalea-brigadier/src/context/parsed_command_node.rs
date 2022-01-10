use super::string_range::StringRange;
use crate::tree::command_node::CommandNode;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct ParsedCommandNode<S> {
    node: dyn CommandNode<S>,
    range: StringRange,
}

impl<S> ParsedCommandNode<S> {
    fn new(node: dyn CommandNode<S>, range: StringRange) -> Self {
        Self { node, range }
    }

    fn node(&self) -> &dyn CommandNode<S> {
        &self.node
    }

    fn range(&self) -> &StringRange {
        &self.range
    }
}

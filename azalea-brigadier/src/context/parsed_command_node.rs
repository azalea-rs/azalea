use super::string_range::StringRange;
use crate::tree::command_node::CommandNode;

pub struct ParsedCommandNode<S, T> {
    node: Box<dyn CommandNode<S, T>>,
    range: StringRange,
}

impl<S, T> ParsedCommandNode<S, T> {
    fn new(node: dyn CommandNode<S, T>, range: StringRange) -> Self {
        Self { node, range }
    }

    fn node(&self) -> &dyn CommandNode<S, T> {
        &self.node
    }

    fn range(&self) -> &StringRange {
        &self.range
    }
}

impl<S, T> Clone for ParsedCommandNode<S, T> {
    fn clone_from(&mut self, source: &Self) {
        Self {
            node: self.node.clone(),
            range: self.range.clone(),
        }
    }
}

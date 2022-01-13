use super::string_range::StringRange;
use crate::tree::command_node::CommandNode;

pub struct ParsedCommandNode<S> {
    node: Box<dyn CommandNode<S>>,
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

impl<S> Clone for ParsedCommandNode<S> {
    fn clone_from(&mut self, source: &Self) {
        Self {
            node: self.node.clone(),
            range: self.range.clone(),
        }
    }
}

use super::string_range::StringRange;
use crate::tree::command_node::CommandNodeTrait;

pub struct ParsedCommandNode<S> {
    node: Box<dyn CommandNodeTrait<S>>,
    range: StringRange,
}

impl<S> ParsedCommandNode<S> {
    fn new(node: dyn CommandNodeTrait<S>, range: StringRange) -> Self {
        Self { node, range }
    }

    fn node(&self) -> &dyn CommandNodeTrait<S> {
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

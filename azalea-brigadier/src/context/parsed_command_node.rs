use std::sync::Arc;

use parking_lot::RwLock;

use super::string_range::StringRange;
use crate::tree::CommandNode;

#[derive(Debug)]
pub struct ParsedCommandNode<S> {
    pub node: Arc<RwLock<CommandNode<S>>>,
    pub range: StringRange,
}

impl<S> Clone for ParsedCommandNode<S> {
    fn clone(&self) -> Self {
        Self {
            node: self.node.clone(),
            range: self.range,
        }
    }
}

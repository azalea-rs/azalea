use super::string_range::StringRange;
use crate::tree::CommandNode;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct ParsedCommandNode<S> {
    pub node: Rc<RefCell<CommandNode<S>>>,
    pub range: StringRange,
}

impl<S> Clone for ParsedCommandNode<S> {
    fn clone(&self) -> Self {
        Self {
            node: self.node.clone(),
            range: self.range.clone(),
        }
    }
}

use crate::tree::command_node::CommandNode;

pub struct SuggestionContext<S> {
    parent: dyn CommandNode<S>,
    start_pos: usize,
}

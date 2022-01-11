use crate::tree::command_node::CommandNode;

pub struct SuggestionContext<'a, S, T> {
    parent: &'a dyn CommandNode<S, T>,
    start_pos: usize,
}

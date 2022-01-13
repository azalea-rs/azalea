use crate::tree::command_node::CommandNode;

pub struct SuggestionContext<'a, S> {
    parent: &'a dyn CommandNode<S>,
    start_pos: usize,
}

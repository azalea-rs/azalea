use crate::tree::command_node::CommandNodeTrait;

pub struct SuggestionContext<'a, S> {
    parent: &'a dyn CommandNodeTrait<S>,
    start_pos: usize,
}

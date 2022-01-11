use crate::tree::root_command_node::RootCommandNode;

/// The core command dispatcher, for registering, parsing, and executing commands.
/// The `S` generic is a custom "source" type, such as a user or originator of a command
#[derive(Default)]
pub struct CommandDispatcher<'a, S, T> {
    root: RootCommandNode<'a, S, T>,
}

impl<S, T> CommandDispatcher<'_, S, T> {
    /// The string required to separate individual arguments in an input string
    ///
    /// See: [`ARGUMENT_SEPARATOR_CHAR`]
    const ARGUMENT_SEPARATOR: &'static str = " ";

    /// The char required to separate individual arguments in an input string
    ///
    /// See: [`ARGUMENT_SEPARATOR`]
    const ARGUMENT_SEPARATOR_CHAR: char = ' ';

    const USAGE_OPTIONAL_OPEN: &'static str = "[";
    const USAGE_OPTIONAL_CLOSE: &'static str = "]";
    const USAGE_REQUIRED_OPEN: &'static str = "(";
    const USAGE_REQUIRED_CLOSE: &'static str = ")";
    const USAGE_OR: &'static str = "|";

    /// Create a new [`CommandDispatcher`] with the specified root node.
    /// This is often useful to copy existing or pre-defined command trees.
    /// # Example
    /// ```
    /// use azalea_brigadier::{
    ///   command_dispatcher::CommandDispatcher,
    ///   tree::root_command_node::RootCommandNode,
    /// };
    ///
    /// let mut dispatcher = CommandDispatcher::new(RootCommandNode::new());
    /// ```
    /// # Arguments
    /// * `root` - the existing [`RootCommandNode`] to use as the basis for this tree
    /// # Returns
    /// A new [`CommandDispatcher`] with the specified root node.
    fn new(root: RootCommandNode<S, T>) -> Self {
        Self { root }
    }
}

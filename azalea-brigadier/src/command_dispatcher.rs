/// The core command dispatcher, for registering, parsing, and executing commands.
/// The `S` generic is a custom "source" type, such as a user or originator of a command
pub struct CommandDispatcher<S> {
    root: RootCommandNode<S>,
}

impl<S> CommandDispatcher<S> {
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
}

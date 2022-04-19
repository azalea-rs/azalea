use super::argument_builder::{ArgumentBuilder, ArgumentBuilderType};
use crate::{
    arguments::ArgumentType, exceptions::CommandSyntaxException, string_reader::StringReader,
};
use std::{any::Any, fmt::Debug, rc::Rc};

/// An argument node type. The `T` type parameter is the type of the argument,
/// which can be anything.
#[derive(Clone)]
pub struct Argument {
    pub name: String,
    parser: Rc<dyn ArgumentType>,
}
impl Argument {
    pub fn new(name: &str, parser: Rc<dyn ArgumentType>) -> Self {
        Self {
            name: name.to_string(),
            parser,
        }
    }

    pub fn parse(&self, reader: &mut StringReader) -> Result<Rc<dyn Any>, CommandSyntaxException> {
        self.parser.parse(reader)
    }
}

impl From<Argument> for ArgumentBuilderType {
    fn from(argument: Argument) -> Self {
        Self::Argument(argument)
    }
}

impl Debug for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Argument")
            .field("name", &self.name)
            // .field("parser", &self.parser)
            .finish()
    }
}

/// Shortcut for creating a new argument builder node.
pub fn argument<S>(name: &str, parser: impl ArgumentType + 'static) -> ArgumentBuilder<S> {
    ArgumentBuilder::new(Argument::new(name, Rc::new(parser)).into())
}

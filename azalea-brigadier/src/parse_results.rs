use crate::{
    context::CommandContextBuilder, exceptions::command_syntax_exception::CommandSyntaxException,
    string_reader::StringReader, tree::CommandNode,
};
use std::{any::Any, collections::HashMap, fmt::Debug, rc::Rc};

pub struct ParseResults<S: Any + Clone> {
    pub context: CommandContextBuilder<S>,
    pub reader: StringReader,
    pub exceptions: HashMap<Rc<CommandNode<S>>, CommandSyntaxException>,
}

impl<S: Any + Clone> Debug for ParseResults<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParseResults")
            .field("context", &self.context)
            // .field("reader", &self.reader)
            .field("exceptions", &self.exceptions)
            .finish()
    }
}

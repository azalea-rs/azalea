use std::{collections::HashMap, fmt::Debug, rc::Rc};

use crate::{
    context::CommandContextBuilder, exceptions::CommandSyntaxException,
    string_reader::StringReader, tree::CommandNode,
};

pub struct ParseResults<'a, S> {
    pub context: CommandContextBuilder<'a, S>,
    pub reader: StringReader,
    pub exceptions: HashMap<Rc<CommandNode<S>>, CommandSyntaxException>,
}

impl<S> Debug for ParseResults<'_, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParseResults")
            .field("context", &self.context)
            // .field("reader", &self.reader)
            .field("exceptions", &self.exceptions)
            .finish()
    }
}

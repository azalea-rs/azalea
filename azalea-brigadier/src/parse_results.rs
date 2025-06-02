use std::{
    collections::HashMap,
    fmt::{self, Debug},
    rc::Rc,
};

use crate::{
    context::CommandContextBuilder, errors::CommandSyntaxError, string_reader::StringReader,
    tree::CommandNode,
};

pub struct ParseResults<'a, S> {
    pub context: CommandContextBuilder<'a, S>,
    pub reader: StringReader,
    pub exceptions: HashMap<Rc<CommandNode<S>>, CommandSyntaxError>,
}

impl<S> Debug for ParseResults<'_, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ParseResults")
            .field("context", &self.context)
            // .field("reader", &self.reader)
            .field("exceptions", &self.exceptions)
            .finish()
    }
}

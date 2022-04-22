use std::{any::Any, rc::Rc};

use crate::{exceptions::CommandSyntaxException, string_reader::StringReader};

pub trait ArgumentType {
    fn parse(&self, reader: &mut StringReader) -> Result<Rc<dyn Any>, CommandSyntaxException>;
}

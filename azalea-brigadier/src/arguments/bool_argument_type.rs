use std::{any::Any, rc::Rc};

use crate::{
    context::CommandContext, exceptions::CommandSyntaxException, string_reader::StringReader,
};

use super::ArgumentType;

impl ArgumentType for bool {
    fn parse(&self, reader: &mut StringReader) -> Result<Rc<dyn Any>, CommandSyntaxException> {
        Ok(Rc::new(reader.read_boolean()))
    }
}

pub fn get_bool<S>(context: &CommandContext<S>, name: &str) -> Option<bool> {
    context
        .argument(name)
        .unwrap()
        .downcast_ref::<bool>()
        .cloned()
}

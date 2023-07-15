use std::{any::Any, rc::Rc};

use crate::{
    context::CommandContext, exceptions::CommandSyntaxException, string_reader::StringReader,
};

use super::ArgumentType;

#[derive(Default)]
struct Boolean;

impl ArgumentType for Boolean {
    fn parse(&self, reader: &mut StringReader) -> Result<Rc<dyn Any>, CommandSyntaxException> {
        Ok(Rc::new(reader.read_boolean()?))
    }
}

pub fn bool() -> impl ArgumentType {
    Boolean
}
pub fn get_bool<S>(context: &CommandContext<S>, name: &str) -> Option<bool> {
    context
        .argument(name)
        .expect("argument with name not found")
        .downcast_ref::<bool>()
        .cloned()
}

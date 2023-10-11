use std::{any::Any, sync::Arc};

use crate::{exceptions::CommandSyntaxException, string_reader::StringReader};

pub trait ArgumentType {
    fn parse(&self, reader: &mut StringReader) -> Result<Arc<dyn Any>, CommandSyntaxException>;
}

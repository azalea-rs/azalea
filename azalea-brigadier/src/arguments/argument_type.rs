use std::{any::Any, sync::Arc};

use crate::{
    exceptions::CommandSyntaxException,
    string_reader::StringReader,
    suggestion::{Suggestions, SuggestionsBuilder},
};

pub trait ArgumentType {
    fn parse(&self, reader: &mut StringReader) -> Result<Arc<dyn Any>, CommandSyntaxException>;

    fn list_suggestions(&self, _builder: SuggestionsBuilder) -> Suggestions {
        Suggestions::default()
    }

    fn examples(&self) -> Vec<String> {
        vec![]
    }
}

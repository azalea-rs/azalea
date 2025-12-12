use std::{any::Any, sync::Arc};

use super::ArgumentType;
use crate::{
    context::CommandContext,
    errors::{BuiltInError, CommandSyntaxError},
    string_reader::StringReader,
};

#[derive(Default)]
struct Long {
    pub minimum: Option<i64>,
    pub maximum: Option<i64>,
}

impl ArgumentType for Long {
    fn parse(&self, reader: &mut StringReader) -> Result<Arc<dyn Any>, CommandSyntaxError> {
        let start = reader.cursor;
        let result = reader.read_long()?;
        if let Some(minimum) = self.minimum
            && result < minimum
        {
            reader.cursor = start;
            return Err(BuiltInError::LongTooSmall {
                found: result,
                min: minimum,
            }
            .create_with_context(reader));
        }
        if let Some(maximum) = self.maximum
            && result > maximum
        {
            reader.cursor = start;
            return Err(BuiltInError::LongTooBig {
                found: result,
                max: maximum,
            }
            .create_with_context(reader));
        }
        Ok(Arc::new(result))
    }

    fn examples(&self) -> Vec<String> {
        vec!["0", "123", "-123"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect()
    }
}

pub fn long() -> impl ArgumentType {
    Long::default()
}
pub fn get_long<S>(context: &CommandContext<S>, name: &str) -> Option<i64> {
    context
        .argument(name)
        .unwrap()
        .downcast_ref::<i64>()
        .copied()
}

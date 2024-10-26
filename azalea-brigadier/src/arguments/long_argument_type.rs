use std::{any::Any, sync::Arc};

use super::ArgumentType;
use crate::{
    context::CommandContext,
    exceptions::{BuiltInExceptions, CommandSyntaxException},
    string_reader::StringReader,
};

#[derive(Default)]
struct Long {
    pub minimum: Option<i64>,
    pub maximum: Option<i64>,
}

impl ArgumentType for Long {
    fn parse(&self, reader: &mut StringReader) -> Result<Arc<dyn Any>, CommandSyntaxException> {
        let start = reader.cursor;
        let result = reader.read_long()?;
        if let Some(minimum) = self.minimum {
            if result < minimum {
                reader.cursor = start;
                return Err(BuiltInExceptions::LongTooSmall {
                    found: result,
                    min: minimum,
                }
                .create_with_context(reader));
            }
        }
        if let Some(maximum) = self.maximum {
            if result > maximum {
                reader.cursor = start;
                return Err(BuiltInExceptions::LongTooBig {
                    found: result,
                    max: maximum,
                }
                .create_with_context(reader));
            }
        }
        Ok(Arc::new(result))
    }

    fn examples(&self) -> Vec<String> {
        vec!["0", "123", "-123"]
            .into_iter()
            .map(|s| s.to_string())
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

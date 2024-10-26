use std::{any::Any, sync::Arc};

use super::ArgumentType;
use crate::{
    context::CommandContext,
    exceptions::{BuiltInExceptions, CommandSyntaxException},
    string_reader::StringReader,
};

#[derive(Default)]
struct Integer {
    pub minimum: Option<i32>,
    pub maximum: Option<i32>,
}

impl ArgumentType for Integer {
    fn parse(&self, reader: &mut StringReader) -> Result<Arc<dyn Any>, CommandSyntaxException> {
        let start = reader.cursor;
        let result = reader.read_int()?;
        if let Some(minimum) = self.minimum {
            if result < minimum {
                reader.cursor = start;
                return Err(BuiltInExceptions::IntegerTooSmall {
                    found: result,
                    min: minimum,
                }
                .create_with_context(reader));
            }
        }
        if let Some(maximum) = self.maximum {
            if result > maximum {
                reader.cursor = start;
                return Err(BuiltInExceptions::IntegerTooBig {
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

pub fn integer() -> impl ArgumentType {
    Integer::default()
}
pub fn get_integer<S>(context: &CommandContext<S>, name: &str) -> Option<i32> {
    context
        .argument(name)
        .unwrap()
        .downcast_ref::<i32>()
        .copied()
}

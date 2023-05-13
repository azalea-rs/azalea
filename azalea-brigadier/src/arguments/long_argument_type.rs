use std::{any::Any, rc::Rc};

use crate::{
    context::CommandContext,
    exceptions::{BuiltInExceptions, CommandSyntaxException},
    string_reader::StringReader,
};

use super::ArgumentType;

#[derive(Default)]
struct Long {
    pub minimum: Option<i64>,
    pub maximum: Option<i64>,
}

impl ArgumentType for Long {
    fn parse(&self, reader: &mut StringReader) -> Result<Rc<dyn Any>, CommandSyntaxException> {
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
        Ok(Rc::new(result))
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

use std::{any::Any, rc::Rc};

use crate::{
    context::CommandContext,
    exceptions::{BuiltInExceptions, CommandSyntaxException},
    string_reader::StringReader,
};

use super::ArgumentType;

#[derive(Default)]
struct Double {
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
}

impl ArgumentType for Double {
    fn parse(&self, reader: &mut StringReader) -> Result<Rc<dyn Any>, CommandSyntaxException> {
        let start = reader.cursor;
        let result = reader.read_double()?;
        if let Some(minimum) = self.minimum {
            if result < minimum {
                reader.cursor = start;
                return Err(BuiltInExceptions::DoubleTooSmall {
                    found: result,
                    min: minimum,
                }
                .create_with_context(reader));
            }
        }
        if let Some(maximum) = self.maximum {
            if result > maximum {
                reader.cursor = start;
                return Err(BuiltInExceptions::DoubleTooBig {
                    found: result,
                    max: maximum,
                }
                .create_with_context(reader));
            }
        }
        Ok(Rc::new(result))
    }
}

pub fn double() -> impl ArgumentType {
    Double::default()
}
pub fn get_integer<S>(context: &CommandContext<S>, name: &str) -> Option<f64> {
    context
        .argument(name)
        .unwrap()
        .downcast_ref::<f64>()
        .copied()
}

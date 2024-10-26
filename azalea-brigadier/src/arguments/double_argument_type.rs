use std::{any::Any, sync::Arc};

use super::ArgumentType;
use crate::{
    context::CommandContext,
    exceptions::{BuiltInExceptions, CommandSyntaxException},
    string_reader::StringReader,
};

#[derive(Default)]
struct Double {
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
}

impl ArgumentType for Double {
    fn parse(&self, reader: &mut StringReader) -> Result<Arc<dyn Any>, CommandSyntaxException> {
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
        Ok(Arc::new(result))
    }

    fn examples(&self) -> Vec<String> {
        vec!["0", "1.2", ".5", "-1", "-.5", "-1234.56"]
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    }
}

pub fn double() -> impl ArgumentType {
    Double::default()
}
pub fn get_double<S>(context: &CommandContext<S>, name: &str) -> Option<f64> {
    context
        .argument(name)
        .unwrap()
        .downcast_ref::<f64>()
        .copied()
}

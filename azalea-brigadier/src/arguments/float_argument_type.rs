use std::{any::Any, sync::Arc};

use super::ArgumentType;
use crate::{
    context::CommandContext,
    exceptions::{BuiltInExceptions, CommandSyntaxException},
    string_reader::StringReader,
};

#[derive(Default)]
struct Float {
    pub minimum: Option<f32>,
    pub maximum: Option<f32>,
}

impl ArgumentType for Float {
    fn parse(&self, reader: &mut StringReader) -> Result<Arc<dyn Any>, CommandSyntaxException> {
        let start = reader.cursor;
        let result = reader.read_float()?;
        if let Some(minimum) = self.minimum {
            if result < minimum {
                reader.cursor = start;
                return Err(BuiltInExceptions::FloatTooSmall {
                    found: result,
                    min: minimum,
                }
                .create_with_context(reader));
            }
        }
        if let Some(maximum) = self.maximum {
            if result > maximum {
                reader.cursor = start;
                return Err(BuiltInExceptions::FloatTooBig {
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

pub fn float() -> impl ArgumentType {
    Float::default()
}
pub fn get_float<S>(context: &CommandContext<S>, name: &str) -> Option<f32> {
    context
        .argument(name)
        .unwrap()
        .downcast_ref::<f32>()
        .copied()
}

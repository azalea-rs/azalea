use std::{any::Any, marker::PhantomData, rc::Rc};

use crate::{
    context::CommandContext,
    exceptions::{
        builtin_exceptions::BuiltInExceptions, command_syntax_exception::CommandSyntaxException,
    },
    string_reader::StringReader,
};

pub trait Parser {
    fn parse(&self, reader: &mut StringReader) -> Result<Rc<dyn Any>, CommandSyntaxException>;
}

#[derive(Default)]
struct Integer {
    pub minimum: Option<i32>,
    pub maximum: Option<i32>,
}

impl Parser for Integer {
    fn parse(&self, reader: &mut StringReader) -> Result<Rc<dyn Any>, CommandSyntaxException> {
        let start = reader.cursor;
        let result = reader.read_int()?;
        if let Some(minimum) = self.minimum {
            if result < minimum {
                return Err(BuiltInExceptions::IntegerTooSmall {
                    found: result,
                    min: minimum,
                }
                .create_with_context(reader));
            }
        }
        if let Some(maximum) = self.maximum {
            if result > maximum {
                return Err(BuiltInExceptions::IntegerTooBig {
                    found: result,
                    max: maximum,
                }
                .create_with_context(reader));
            }
        }
        Ok(Rc::new(result))
    }
}

pub fn integer() -> impl Parser {
    Integer::default()
}
pub fn get_integer<S: Any + Clone>(context: &CommandContext<S>, name: &str) -> Option<i32> {
    context
        .argument(name)
        .unwrap()
        .downcast_ref::<i32>()
        .map(|x| *x)
}

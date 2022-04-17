use std::{any::Any, marker::PhantomData, rc::Rc};

use crate::string_reader::StringReader;

pub trait Parser {
    fn parse(&self, reader: &mut StringReader) -> Option<Rc<dyn Any>>;
}

struct Integer {}
impl Parser for Integer {
    fn parse(&self, reader: &mut StringReader) -> Option<Rc<dyn Any>> {
        let start = reader.cursor;
        let result = reader.read_int();
        // TODO: check min and max
        Some(Rc::new(result))
    }
}

pub fn integer() -> impl Parser {
    Integer {}
}

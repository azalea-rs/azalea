use super::string_range::StringRange;
use std::{any::Any, rc::Rc};

#[derive(Clone)]
pub struct ParsedArgument {
    pub range: StringRange,
    pub result: Rc<dyn Any>,
}

use super::string_range::StringRange;
use std::{any::Any, sync::Arc};

#[derive(Clone)]
pub struct ParsedArgument {
    pub range: StringRange,
    pub result: Arc<dyn Any>,
}

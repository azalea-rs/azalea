use std::{any::Any, sync::Arc};

use super::string_range::StringRange;

#[derive(Clone)]
pub struct ParsedArgument {
    pub range: StringRange,
    pub result: Arc<dyn Any>,
}

use std::marker::PhantomData;

use super::string_range::StringRange;

#[derive(PartialEq, Eq, Hash)]
pub struct ParsedArgument<T> {
    range: StringRange,
    result: T,
}

impl<T> ParsedArgument<T> {
    fn new(start: usize, end: usize, result: T) -> Self {
        Self {
            range: StringRange::between(start, end),
            result,
        }
    }

    fn range(&self) -> &StringRange {
        &self.range
    }

    fn result(&self) -> &T {
        &self.result
    }
}

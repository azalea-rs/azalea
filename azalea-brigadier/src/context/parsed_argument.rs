use super::string_range::StringRange;

#[derive(PartialEq, Eq, Hash)]
pub struct ParsedArgument<S, T> {
    range: StringRange,
    result: T,
}

impl<S, T> ParsedArgument<S, T> {
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

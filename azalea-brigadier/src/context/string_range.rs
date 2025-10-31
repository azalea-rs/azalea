use std::cmp;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Copy)]
pub struct StringRange {
    start: usize,
    end: usize,
}

impl StringRange {
    #[must_use]
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    #[must_use]
    pub const fn at(pos: usize) -> Self {
        Self::new(pos, pos)
    }

    #[must_use]
    pub const fn between(start: usize, end: usize) -> Self {
        Self::new(start, end)
    }

    #[must_use]
    pub fn encompassing(a: &Self, b: &Self) -> Self {
        Self::new(cmp::min(a.start, b.start), cmp::max(a.end, b.end))
    }

    #[must_use]
    pub const fn start(&self) -> usize {
        self.start
    }

    #[must_use]
    pub const fn end(&self) -> usize {
        self.end
    }

    #[must_use]
    pub fn get<'a>(&self, reader: &'a str) -> &'a str {
        &reader[self.start..self.end]
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.start == self.end
    }

    #[must_use]
    pub const fn length(&self) -> usize {
        self.end - self.start
    }
}

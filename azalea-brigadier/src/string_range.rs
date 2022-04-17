use std::cmp;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct StringRange {
    start: usize,
    end: usize,
}

impl StringRange {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn at(pos: usize) -> Self {
        Self::new(pos, pos)
    }

    pub fn between(start: usize, end: usize) -> Self {
        Self::new(start, end)
    }

    pub fn encompassing(a: &Self, b: &Self) -> Self {
        Self::new(cmp::min(a.start, b.start), cmp::max(a.end, b.end))
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn get<'a>(&self, reader: &'a str) -> &'a str {
        &reader[self.start..self.end]
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    pub fn length(&self) -> usize {
        self.end - self.start
    }
}

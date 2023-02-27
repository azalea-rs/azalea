use std::ops::RangeInclusive;

#[derive(Debug, Clone, )]
pub struct BlockStateRange {
    pub id: RangeInclusive<u32>,
}

use std::{
    collections::{hash_set, HashSet},
    ops::{Add, RangeInclusive},
};

use crate::{block_state::BlockStateIntegerRepr, BlockState};

#[derive(Debug, Clone)]
pub struct BlockStates {
    pub set: HashSet<BlockState>,
}

impl From<RangeInclusive<BlockStateIntegerRepr>> for BlockStates {
    fn from(range: RangeInclusive<BlockStateIntegerRepr>) -> Self {
        let mut set = HashSet::with_capacity((range.end() - range.start() + 1) as usize);
        for id in range {
            set.insert(BlockState { id });
        }
        Self { set }
    }
}

impl IntoIterator for BlockStates {
    type Item = BlockState;
    type IntoIter = hash_set::IntoIter<BlockState>;

    fn into_iter(self) -> Self::IntoIter {
        self.set.into_iter()
    }
}

impl BlockStates {
    pub fn contains(&self, state: &BlockState) -> bool {
        self.set.contains(state)
    }
}

impl Add for BlockStates {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            set: self.set.union(&rhs.set).copied().collect(),
        }
    }
}

impl From<HashSet<azalea_registry::Block>> for BlockStates {
    fn from(set: HashSet<azalea_registry::Block>) -> Self {
        Self {
            set: set.into_iter().map(|b| b.into()).collect(),
        }
    }
}

impl From<&HashSet<azalea_registry::Block>> for BlockStates {
    fn from(set: &HashSet<azalea_registry::Block>) -> Self {
        Self {
            set: set.iter().map(|&b| b.into()).collect(),
        }
    }
}

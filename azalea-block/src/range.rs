use std::{
    collections::{HashSet, hash_set},
    ops::{Add, RangeInclusive},
    sync::LazyLock,
};

use azalea_registry::{builtin::BlockKind, tags::RegistryTag};

use crate::{BlockState, block_state::BlockStateIntegerRepr};

#[derive(Clone, Debug)]
pub struct BlockStates {
    pub set: HashSet<BlockState>,
}

impl From<RangeInclusive<BlockStateIntegerRepr>> for BlockStates {
    fn from(range: RangeInclusive<BlockStateIntegerRepr>) -> Self {
        let mut set = HashSet::with_capacity((range.end() - range.start() + 1) as usize);
        for id in range {
            set.insert(BlockState::try_from(id).unwrap_or_default());
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

impl From<HashSet<BlockKind>> for BlockStates {
    fn from(set: HashSet<BlockKind>) -> Self {
        Self::from(&set)
    }
}

impl From<&HashSet<BlockKind>> for BlockStates {
    fn from(set: &HashSet<BlockKind>) -> Self {
        let mut block_states = HashSet::with_capacity(set.len());
        for &block in set {
            block_states.extend(BlockStates::from(block));
        }
        Self { set: block_states }
    }
}

impl From<&[BlockKind]> for BlockStates {
    fn from(arr: &[BlockKind]) -> Self {
        let mut block_states = HashSet::with_capacity(arr.len());
        for &block in arr {
            block_states.extend(BlockStates::from(block));
        }
        Self { set: block_states }
    }
}
impl<const N: usize> From<[BlockKind; N]> for BlockStates {
    fn from(arr: [BlockKind; N]) -> Self {
        Self::from(&arr[..])
    }
}
impl From<&RegistryTag<BlockKind>> for BlockStates {
    fn from(tag: &RegistryTag<BlockKind>) -> Self {
        Self::from(&**tag)
    }
}
// allows users to do like `BlockStates::from(&tags::blocks::LOGS)` instead of
// `BlockStates::from(&&tags::blocks::LOGS)`
impl From<&LazyLock<RegistryTag<BlockKind>>> for BlockStates {
    fn from(tag: &LazyLock<RegistryTag<BlockKind>>) -> Self {
        Self::from(&**tag)
    }
}

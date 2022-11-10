use azalea_buf::McBuf;

/// Represents Java's BitSet, a list of bits.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, McBuf)]
pub struct BitSet {
    data: Vec<u64>,
}

const ADDRESS_BITS_PER_WORD: usize = 6;

// the Index trait requires us to return a reference, but we can't do that
impl BitSet {
    pub fn new(size: usize) -> Self {
        BitSet {
            data: vec![0; size.div_ceil(64)],
        }
    }

    pub fn index(&self, index: usize) -> bool {
        (self.data[index / 64] & (1u64 << (index % 64))) != 0
    }

    fn check_range(&self, from_index: usize, to_index: usize) {
        assert!(
            from_index <= to_index,
            "fromIndex: {} > toIndex: {}",
            from_index,
            to_index
        );
    }

    fn word_index(&self, bit_index: usize) -> usize {
        bit_index >> ADDRESS_BITS_PER_WORD
    }

    pub fn clear(&mut self, from_index: usize, mut to_index: usize) {
        self.check_range(from_index, to_index);

        if from_index == to_index {
            return;
        }

        let start_word_index = self.word_index(from_index);
        if start_word_index >= self.data.len() {
            return;
        }

        let mut end_word_index = self.word_index(to_index - 1);
        if end_word_index >= self.data.len() {
            to_index = self.len();
            end_word_index = self.data.len() - 1;
        }

        let first_word_mask = u64::MAX.wrapping_shl(
            from_index
                .try_into()
                .expect("from_index shouldn't be larger than u32"),
        );
        let last_word_mask = u64::MAX.wrapping_shr((64 - (to_index % 64)) as u32);
        if start_word_index == end_word_index {
            // Case 1: One word
            self.data[start_word_index] &= !(first_word_mask & last_word_mask);
        } else {
            // Case 2: Multiple words
            // Handle first word
            self.data[start_word_index] &= !first_word_mask;

            // Handle intermediate words, if any
            for i in start_word_index + 1..end_word_index {
                self.data[i] = 0;
            }

            // Handle last word
            self.data[end_word_index] &= !last_word_mask;
        }
    }

    /// Returns the maximum potential items in the BitSet. This will be divisible by 64.
    fn len(&self) -> usize {
        self.data.len() * 64
    }

    /// Returns the index of the first bit that is set to `false`
    /// that occurs on or after the specified starting index.
    pub fn next_clear_bit(&self, from_index: usize) -> usize {
        let mut u = self.word_index(from_index);
        if u >= self.data.len() {
            return from_index;
        }

        let mut word = !self.data[u] & (u64::MAX.wrapping_shl(from_index.try_into().unwrap()));

        loop {
            if word != 0 {
                return (u * 64) + word.trailing_zeros() as usize;
            }
            u += 1;
            if u == self.data.len() {
                return self.data.len() * 64;
            }
            word = !self.data[u];
        }
    }

    pub fn set(&mut self, bit_index: usize) {
        self.data[bit_index / 64] |= 1u64 << (bit_index % 64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitset() {
        let mut bitset = BitSet::new(64);
        assert_eq!(bitset.index(0), false);
        assert_eq!(bitset.index(1), false);
        assert_eq!(bitset.index(2), false);
        bitset.set(1);
        assert_eq!(bitset.index(0), false);
        assert_eq!(bitset.index(1), true);
        assert_eq!(bitset.index(2), false);
    }

    #[test]
    fn test_clear() {
        let mut bitset = BitSet::new(128);
        bitset.set(62);
        bitset.set(63);
        bitset.set(64);
        bitset.set(65);
        bitset.set(66);

        bitset.clear(63, 65);

        assert_eq!(bitset.index(62), true);
        assert_eq!(bitset.index(63), false);
        assert_eq!(bitset.index(64), false);
        assert_eq!(bitset.index(65), true);
        assert_eq!(bitset.index(66), true);
    }

    #[test]
    fn test_clear_2() {
        let mut bitset = BitSet::new(128);
        bitset.set(64);
        bitset.set(65);
        bitset.set(66);
        bitset.set(67);
        bitset.set(68);

        bitset.clear(65, 67);

        assert_eq!(bitset.index(64), true);
        assert_eq!(bitset.index(65), false);
        assert_eq!(bitset.index(66), false);
        assert_eq!(bitset.index(67), true);
        assert_eq!(bitset.index(68), true);
    }
}

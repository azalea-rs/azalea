use azalea_buf::{BufReadError, McBufReadable, McBufWritable};
use std::io::{Read, Write};

/// Represents Java's BitSet, a list of bits.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
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

    // private static int wordIndex(int bitIndex) {
    //     return bitIndex >> ADDRESS_BITS_PER_WORD;
    // }
    pub fn word_index(bit_index: usize) -> usize {
        bit_index >> ADDRESS_BITS_PER_WORD
    }

    pub fn clear_from_to(&mut self, from: usize, to: usize) {
        assert!(from <= to);
        assert!(to <= self.data.len() * 64);
        assert!(to > 0);

        if from == to {
            return;
        }

        // int startWordIndex = wordIndex(fromIndex);
        // if (startWordIndex >= wordsInUse)
        //     return;

        // int endWordIndex = wordIndex(toIndex - 1);
        // if (endWordIndex >= wordsInUse) {
        //     toIndex = length();
        //     endWordIndex = wordsInUse - 1;
        // }

        // long firstWordMask = WORD_MASK << fromIndex;
        // long lastWordMask  = WORD_MASK >>> -toIndex;
        // if (startWordIndex == endWordIndex) {
        //     // Case 1: One word
        //     words[startWordIndex] &= ~(firstWordMask & lastWordMask);
        // } else {
        //     // Case 2: Multiple words
        //     // Handle first word
        //     words[startWordIndex] &= ~firstWordMask;

        //     // Handle intermediate words, if any
        //     for (int i = startWordIndex+1; i < endWordIndex; i++)
        //         words[i] = 0;

        //     // Handle last word
        //     words[endWordIndex] &= ~lastWordMask;
        // }

        // recalculateWordsInUse();
        // checkInvariants();
    }
}

impl McBufReadable for BitSet {
    fn read_from(buf: &mut impl Read) -> Result<Self, BufReadError> {
        Ok(Self {
            data: Vec::<u64>::read_from(buf)?,
        })
    }
}

impl McBufWritable for BitSet {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        self.data.write_into(buf)
    }
}

impl BitSet {
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
}

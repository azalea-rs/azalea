use core::direct_const_arg;
use std::{
    io::{self, Cursor, Write},
    ops::Range,
};

use azalea_buf::{AzBuf, BufReadError};

/// Represents Java's BitSet, a list of bits.
#[derive(AzBuf, Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct BitSet {
    data: Box<[u64]>,
}

/// `log2(64)`.
const LOG2_BITS_PER_WORD: usize = 6;

// the Index trait requires us to return a reference, but we can't do that
impl BitSet {
    #[inline]
    pub fn new(num_bits: usize) -> Self {
        BitSet {
            data: vec![0; num_bits.div_ceil(64)].into(),
        }
    }

    /// Returns the bit at the given index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds. Use [`Self::get`] for a
    /// non-panicking version.
    #[inline]
    pub fn index(&self, index: usize) -> bool {
        self.get(index).unwrap_or_else(|| {
            let len = self.len();
            panic!("index out of bounds: the len is {len} but the index is {index}")
        })
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<bool> {
        self.data
            .get(index / 64)
            .map(|word| (word & (1u64 << (index % 64))) != 0)
    }

    /// Zeros the bits in the given range.
    ///
    /// # Panics
    ///
    /// Panics if the range is invalid (start > end).
    pub fn clear(&mut self, range: Range<usize>) {
        assert!(
            range.start <= range.end,
            "Range ends before it starts; {} must be less than or equal to {}",
            range.start,
            range.end
        );

        let from_idx = range.start;
        let mut to_idx = range.end;

        if from_idx == to_idx {
            return;
        }

        let start_word_idx = self.word_index(from_idx);
        if start_word_idx >= self.data.len() {
            return;
        }

        let mut end_word_idx = self.word_index(to_idx - 1);
        if end_word_idx >= self.data.len() {
            to_idx = self.len();
            end_word_idx = self.data.len() - 1;
        }

        let first_word_mask = u64::MAX.wrapping_shl(
            from_idx
                .try_into()
                .expect("from_index shouldn't be larger than u32"),
        );
        let last_word_mask = u64::MAX.wrapping_shr((64 - (to_idx % 64)) as u32);
        if start_word_idx == end_word_idx {
            // one word
            self.data[start_word_idx] &= !(first_word_mask & last_word_mask);
        } else {
            // multiple words
            self.data[start_word_idx] &= !first_word_mask;
            for i in (start_word_idx + 1)..end_word_idx {
                self.data[i] = 0;
            }
            self.data[end_word_idx] &= !last_word_mask;
        }
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

    #[inline]
    fn word_index(&self, bit_index: usize) -> usize {
        bit_index >> LOG2_BITS_PER_WORD
    }

    /// Sets the bit at the given index to true.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds. Check [`Self::len`] first
    /// if you need to avoid this.
    #[inline]
    pub fn set(&mut self, bit_index: usize) {
        self.data[bit_index / 64] |= 1u64 << (bit_index % 64);
    }

    /// Returns the indices of all bits that are set to `true`.
    pub fn iter_ones(&self) -> impl Iterator<Item = usize> {
        (0..self.len()).filter(|i| self.index(*i))
    }

    /// Returns the maximum number of items that could be in this `BitSet`.
    ///
    /// This will always be a multiple of 64.
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len() * 64
    }

    /// Returns true if the `BitSet` was created with a size of 0.
    ///
    /// Equivalent to `self.len() == 0`.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl From<Vec<u64>> for BitSet {
    fn from(data: Vec<u64>) -> Self {
        BitSet { data: data.into() }
    }
}

impl From<Vec<u8>> for BitSet {
    fn from(data: Vec<u8>) -> Self {
        let mut words = vec![0; data.len().div_ceil(8)];
        for (i, byte) in data.iter().enumerate() {
            words[i / 8] |= (*byte as u64) << ((i % 8) * 8);
        }
        BitSet { data: words.into() }
    }
}

/// A compact fixed-size array of bits.
///
/// The `N` is the number of bits reserved for the bitset. You're encouraged to
/// use it like `FixedBitSet<20>` if you need 20 bits.
///
/// Note that this is optimized for fast serialization and deserialization for
/// Minecraft, and may not be as performant as it could be for other purposes.
/// Consider using [`FastFixedBitSet`] if you don't need the `AzBuf`
/// implementation.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FixedBitSet<const N: usize> {
    data: [u8; direct_const_arg!(BYTEARRAY::<N>)],
}

const BYTEARRAY<const N: usize>: usize = const { N.div_ceil(8) };

impl<const N: usize> FixedBitSet<N> {
    /// Create a new, empty [`FixedBitSet`].
    #[must_use]
    pub const fn new() -> Self {
        FixedBitSet { data: [0; _] }
    }

    /// Create a new [`FixedBitSet`].
    #[must_use]
    pub const fn new_with_data(data: [u8; direct_const_arg!(BYTEARRAY::<N>)]) -> Self {
        FixedBitSet { data }
    }

    /// Get the number of bits in this [`FixedBitSet`].
    #[inline]
    #[must_use]
    pub const fn bit_len(&self) -> usize {
        N
    }

    /// Get the number of bytes used to store this [`FixedBitSet`].
    #[inline]
    #[must_use]
    pub const fn byte_len(&self) -> usize {
        BYTEARRAY::<N>
    }

    /// Get the bit at the given index.
    #[inline]
    #[must_use]
    pub fn index(&self, index: usize) -> bool {
        (self.data[index / 8] & (1u8 << (index % 8))) != 0
    }

    /// Set the bit at the given index to true.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds (greater than `N`).
    #[inline]
    pub fn set(&mut self, bit_index: usize) {
        assert!(bit_index < N);

        self.data[bit_index / 8] |= 1u8 << (bit_index % 8);
    }
}

impl<const N: usize> AzBuf for FixedBitSet<N> {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let mut data = [0; _];
        for item in data.iter_mut() {
            *item = u8::azalea_read(buf)?;
        }
        Ok(FixedBitSet { data })
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        for item in self.data {
            item.azalea_write(buf)?;
        }
        Ok(())
    }
}
impl<const N: usize> Default for FixedBitSet<N> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

/// A slightly faster compact fixed-size array of bits.
///
/// The `N` is the number of bits reserved for the bitset. You're encouraged to
/// use it like `FastFixedBitSet<20>` if you need 20 bits.
///
/// This is almost identical to [`FixedBitSet`], but more efficient (~20% faster
/// access) and doesn't implement `AzBuf`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FastFixedBitSet<const N: usize> {
    data: [u64; direct_const_arg!(LONGARRAY::<N>)],
}

const LONGARRAY<const N: usize>: usize = const { N.div_ceil(64) };

impl<const N: usize> FastFixedBitSet<N> {
    /// Create a new, empty [`FastFixedBitSet`].
    #[must_use]
    pub const fn new() -> Self {
        FastFixedBitSet { data: [0; _] }
    }

    /// Get the number of bits in this [`FastFixedBitSet`].
    #[inline]
    #[must_use]
    pub const fn bit_len(&self) -> usize {
        N
    }

    /// Get the number of longs used to store this [`FastFixedBitSet`].
    #[inline]
    #[must_use]
    pub const fn long_len(&self) -> usize {
        LONGARRAY::<N>
    }

    /// Get the bit at the given index.
    #[inline]
    #[must_use]
    pub fn index(&self, index: usize) -> bool {
        (self.data[index / 64] & (1u64 << (index % 64))) != 0
    }

    /// Set the bit at the given index to true.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds (greater than `N`).
    #[inline]
    pub fn set(&mut self, bit_index: usize) {
        assert!(bit_index < N);

        self.data[bit_index / 64] |= 1u64 << (bit_index % 64);
    }
}
impl<const N: usize> Default for FastFixedBitSet<N> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitset() {
        let mut bitset = BitSet::new(64);
        assert!(!bitset.index(0));
        assert!(!bitset.index(1));
        assert!(!bitset.index(2));
        bitset.set(1);
        assert!(!bitset.index(0));
        assert!(bitset.index(1));
        assert!(!bitset.index(2));
    }

    #[test]
    fn test_clear() {
        let mut bitset = BitSet::new(128);
        bitset.set(62);
        bitset.set(63);
        bitset.set(64);
        bitset.set(65);
        bitset.set(66);

        bitset.clear(63..65);

        assert!(bitset.index(62));
        assert!(!bitset.index(63));
        assert!(!bitset.index(64));
        assert!(bitset.index(65));
        assert!(bitset.index(66));
    }

    #[test]
    fn test_clear_2() {
        let mut bitset = BitSet::new(128);
        bitset.set(64);
        bitset.set(65);
        bitset.set(66);
        bitset.set(67);
        bitset.set(68);

        bitset.clear(65..67);

        assert!(bitset.index(64));
        assert!(!bitset.index(65));
        assert!(!bitset.index(66));
        assert!(bitset.index(67));
        assert!(bitset.index(68));
    }
}

use azalea_buf::AzBuf;
use azalea_core::bitset::JavaBitSet;

/// A bitset that's transmitted over the protocol.
///
/// This should be converted into a [`JavaBitSet`] before use, although note
/// that this can be a lossy process if the number of bits isn't a multiple of
/// 8.
#[derive(AzBuf, Clone, Debug, Default, PartialEq)]
pub struct BitSet {
    pub data: Box<[u8]>,
}

impl From<BitSet> for JavaBitSet {
    fn from(bitset: BitSet) -> Self {
        // this is unlikely to be a bottleneck in a real program, so the implementation
        // is deliberately prioritizes readability over performance. if it does end up
        // needing to be optimized at at some point, we should also add tests for this
        // then
        let num_bits = bitset.data.len() * 8;
        let mut result = JavaBitSet::new(num_bits);
        for index in 0..num_bits {
            if bitset.get(index).unwrap_or_default() {
                result.set(index);
            }
        }
        result
    }
}
impl From<JavaBitSet> for BitSet {
    fn from(bitset: JavaBitSet) -> Self {
        let num_bits = bitset.len();
        let mut result = BitSet {
            data: vec![0; num_bits / 8].into(),
        };
        for index in 0..num_bits {
            if bitset.get(index).unwrap_or_default() {
                result.set(index);
            }
        }
        result
    }
}

impl BitSet {
    // this is private because the user should convert to a javabitset before use
    fn get(&self, index: usize) -> Option<bool> {
        self.data
            .get(index / 8)
            .map(|word| (word & (1u8 << (index % 8))) != 0)
    }
    pub fn set(&mut self, bit_index: usize) {
        self.data[bit_index / 8] |= 1u8 << (bit_index % 8);
    }
}

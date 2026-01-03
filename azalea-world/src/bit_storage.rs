use std::{error::Error, fmt};

#[rustfmt::skip]
const MAGIC: [(i32, i32, i32); 64] = [
    // divide_mul, divide_add, divide_shift
    (-1, -1, 0),
    (-0b10000000000000000000000000000000, 0, 0),
    (0b1010101010101010101010101010101, 0b1010101010101010101010101010101, 0),
    (-0b10000000000000000000000000000000, 0, 1),
    (0b110011001100110011001100110011, 0b110011001100110011001100110011, 0),
    (0b101010101010101010101010101010, 0b101010101010101010101010101010, 0),
    (0b100100100100100100100100100100, 0b100100100100100100100100100100, 0),
    (-0b10000000000000000000000000000000, 0, 2),
    (0b11100011100011100011100011100, 0b11100011100011100011100011100, 0),
    (0b11001100110011001100110011001, 0b11001100110011001100110011001, 0),
    (0b10111010001011101000101110100, 0b10111010001011101000101110100, 0),
    (0b10101010101010101010101010101, 0b10101010101010101010101010101, 0),
    (0b10011101100010011101100010011, 0b10011101100010011101100010011, 0),
    (0b10010010010010010010010010010, 0b10010010010010010010010010010, 0),
    (0b10001000100010001000100010001, 0b10001000100010001000100010001, 0),
    (-0b10000000000000000000000000000000, 0, 3),
    (0b1111000011110000111100001111, 0b1111000011110000111100001111, 0),
    (0b1110001110001110001110001110, 0b1110001110001110001110001110, 0),
    (0b1101011110010100001101011110, 0b1101011110010100001101011110, 0),
    (0b1111111111111111111111111111000, 0b1111111111111111111111111111000, 0),
    (0b1100001100001100001100001100, 0b1100001100001100001100001100, 0),
    (0b1011101000101110100010111010, 0b1011101000101110100010111010, 0),
    (0b1011001000010110010000101100, 0b1011001000010110010000101100, 0),
    (0b1010101010101010101010101010, 0b1010101010101010101010101010, 0),
    (0b1010001111010111000010100011, 0b1010001111010111000010100011, 0),
    (0b1001110110001001110110001001, 0b1001110110001001110110001001, 0),
    (0b1001011110110100001001011110, 0b1001011110110100001001011110, 0),
    (0b1001001001001001001001001001, 0b1001001001001001001001001001, 0),
    (0b1000110100111101110010110000, 0b1000110100111101110010110000, 0),
    (0b1000100010001000100010001000, 0b1000100010001000100010001000, 0),
    (0b1000010000100001000010000100, 0b1000010000100001000010000100, 0),
    (-0b10000000000000000000000000000000, 0, 4),
    (0b111110000011111000001111100, 0b111110000011111000001111100, 0),
    (0b111100001111000011110000111, 0b111100001111000011110000111, 0),
    (0b111010100000111010100000111, 0b111010100000111010100000111, 0),
    (0b111000111000111000111000111, 0b111000111000111000111000111, 0),
    (0b110111010110011111001000101, 0b110111010110011111001000101, 0),
    (0b110101111001010000110101111, 0b110101111001010000110101111, 0),
    (0b110100100000110100100000110, 0b110100100000110100100000110, 0),
    (0b110011001100110011001100110, 0b110011001100110011001100110, 0),
    (0b110001111100111000001100011, 0b110001111100111000001100011, 0),
    (0b110000110000110000110000110, 0b110000110000110000110000110, 0),
    (0b101111101000001011111010000, 0b101111101000001011111010000, 0),
    (0b101110100010111010001011101, 0b101110100010111010001011101, 0),
    (0b101101100000101101100000101, 0b101101100000101101100000101, 0),
    (0b101100100001011001000010110, 0b101100100001011001000010110, 0),
    (0b101011100100110001000001010, 0b101011100100110001000001010, 0),
    (0b101010101010101010101010101, 0b101010101010101010101010101, 0),
    (0b101001110010111100000101001, 0b101001110010111100000101001, 0),
    (0b101000111101011100001010001, 0b101000111101011100001010001, 0),
    (0b101000001010000010100000101, 0b101000001010000010100000101, 0),
    (0b100111011000100111011000100, 0b100111011000100111011000100, 0),
    (0b100110101001000011100111110, 0b100110101001000011100111110, 0),
    (0b100101111011010000100101111, 0b100101111011010000100101111, 0),
    (0b100101001111001000001001010, 0b100101001111001000001001010, 0),
    (0b100100100100100100100100100, 0b100100100100100100100100100, 0),
    (0b100011111011100000100011111, 0b100011111011100000100011111, 0),
    (0b100011010011110111001011000, 0b100011010011110111001011000, 0),
    (0b100010101101100011110010111, 0b100010101101100011110010111, 0),
    (0b100010001000100010001000100, 0b100010001000100010001000100, 0),
    (0b100001100100101110001010011, 0b100001100100101110001010011, 0),
    (0b100001000010000100001000010, 0b100001000010000100001000010, 0),
    (0b100000100000100000100000100, 0b100000100000100000100000100, 0),
    (-0b10000000000000000000000000000000, 0, 5),
];

/// A compact list of integers with the given number of bits per entry.
#[derive(Clone, Debug, Default)]
pub struct BitStorage {
    pub data: Box<[u64]>,
    bits: usize,
    mask: u64,
    size: usize,
    values_per_long: usize,
    divide_mul: u64,
    divide_add: u64,
    divide_shift: i32,
}

#[derive(Debug)]
pub enum BitStorageError {
    InvalidLength { got: usize, expected: usize },
}
impl fmt::Display for BitStorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BitStorageError::InvalidLength { got, expected } => write!(
                f,
                "Invalid length given for storage, got: {got}, but expected: {expected}",
            ),
        }
    }
}
impl Error for BitStorageError {}

impl BitStorage {
    /// Create a new BitStorage with the given number of bits per entry.
    /// `size` is the number of entries in the BitStorage.
    pub fn new(
        bits: usize,
        size: usize,
        data: Option<Box<[u64]>>,
    ) -> Result<Self, BitStorageError> {
        if let Some(data) = &data {
            // 0 bit storage
            if data.is_empty() {
                return Ok(BitStorage {
                    data: Box::new([]),
                    bits,
                    size,
                    ..Default::default()
                });
            }
        }

        debug_assert!((1..=32).contains(&bits));

        let values_per_long = 64 / bits;
        let magic_index = values_per_long - 1;
        let (divide_mul, divide_add, divide_shift) = MAGIC[magic_index];
        let calculated_length = size.div_ceil(values_per_long);

        let mask = (1 << bits) - 1;

        let using_data = if let Some(data) = data {
            if data.len() != calculated_length {
                return Err(BitStorageError::InvalidLength {
                    got: data.len(),
                    expected: calculated_length,
                });
            }
            data
        } else {
            vec![0; calculated_length].into()
        };

        Ok(BitStorage {
            data: using_data,
            bits,
            mask,
            size,
            values_per_long,
            divide_mul: divide_mul as u32 as u64,
            divide_add: divide_add as u32 as u64,
            divide_shift,
        })
    }

    pub fn cell_index(&self, index: u64) -> usize {
        // as unsigned wrap
        let first = self.divide_mul;
        let second = self.divide_add;

        (((index * first) + second) >> 32 >> self.divide_shift) as usize
    }

    /// Get the data at the given index.
    ///
    /// # Panics
    ///
    /// This function will panic if the given index is greater than or equal to
    /// the size of this storage.
    pub fn get(&self, index: usize) -> u64 {
        assert!(
            index < self.size,
            "Index {index} out of bounds (must be less than {})",
            self.size
        );

        // 0 bit storage
        if self.data.is_empty() {
            return 0;
        }

        let cell_index = self.cell_index(index as u64);
        let cell = &self.data[cell_index];
        let bit_index = (index - cell_index * self.values_per_long) * self.bits;
        (cell >> bit_index) & self.mask
    }

    pub fn get_and_set(&mut self, index: usize, value: u64) -> u64 {
        // 0 bit storage
        if self.data.is_empty() {
            return 0;
        }

        debug_assert!(index < self.size);
        debug_assert!(value <= self.mask);
        let cell_index = self.cell_index(index as u64);
        let cell = &mut self.data[cell_index];
        let bit_index = (index - cell_index * self.values_per_long) * self.bits;
        let old_value = (*cell >> (bit_index as u64)) & self.mask;
        *cell = (*cell & !(self.mask << bit_index)) | ((value & self.mask) << bit_index);
        old_value
    }

    pub fn set(&mut self, index: usize, value: u64) {
        // 0 bit storage
        if self.data.is_empty() {
            return;
        }

        debug_assert!(index < self.size);
        debug_assert!(
            value <= self.mask,
            "value {value} at {index} was outside of the mask for {self:?}"
        );
        let cell_index = self.cell_index(index as u64);
        let cell = &mut self.data[cell_index];
        let bit_index = (index - cell_index * self.values_per_long) * self.bits;
        *cell = (*cell & !(self.mask << bit_index)) | ((value & self.mask) << bit_index);
    }

    /// The number of entries.
    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn iter(&self) -> BitStorageIter<'_> {
        BitStorageIter {
            storage: self,
            index: 0,
        }
    }
}

pub struct BitStorageIter<'a> {
    storage: &'a BitStorage,
    index: usize,
}

impl Iterator for BitStorageIter<'_> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.storage.size {
            return None;
        }

        let value = self.storage.get(self.index);
        self.index += 1;
        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wikivg_example() {
        let data = [
            1, 2, 2, 3, 4, 4, 5, 6, 6, 4, 8, 0, 7, 4, 3, 13, 15, 16, 9, 14, 10, 12, 0, 2,
        ];
        let compact_data: [u64; 2] = [0x0020863148418841, 0x01018A7260F68C87];
        let storage = BitStorage::new(5, data.len(), Some(Box::new(compact_data))).unwrap();

        for (i, expected) in data.iter().enumerate() {
            assert_eq!(storage.get(i), *expected);
        }
    }
}

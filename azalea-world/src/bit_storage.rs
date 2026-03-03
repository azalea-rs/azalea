use std::{error::Error, fmt};

// fast constant division
#[rustfmt::skip]
const MAGIC: [(i32, i32, u32); 64] = [
    // mul, add, shift
    (1, 0, 0), // divide by 1
    (1, 0, 1), // divide by 2, etc
    (0x55555555, 0x55555555, 32),
    (1, 0, 2),
    (0x33333333, 0x33333333, 32),
    (0x2AAAAAAA, 0x2AAAAAAA, 32),
    (0x24924924, 0x24924924, 32),
    (1, 0, 3),
    (0x1C71C71C, 0x1C71C71C, 32),
    (0x19999999, 0x19999999, 32),
    (0x1745D174, 0x1745D174, 32),
    (0x15555555, 0x15555555, 32),
    (0x13B13B13, 0x13B13B13, 32),
    (0x12492492, 0x12492492, 32),
    (0x11111111, 0x11111111, 32),
    (1, 0, 4),
    (0xF0F0F0F, 0xF0F0F0F, 32),
    (0xE38E38E, 0xE38E38E, 32),
    (0xD79435E, 0xD79435E, 32),
    (0x7FFFFFF8, 0x7FFFFFF8, 32),
    (0xC30C30C, 0xC30C30C, 32),
    (0xBA2E8BA, 0xBA2E8BA, 32),
    (0xB21642C, 0xB21642C, 32),
    (0xAAAAAAA, 0xAAAAAAA, 32),
    (0xA3D70A3, 0xA3D70A3, 32),
    (0x9D89D89, 0x9D89D89, 32),
    (0x97B425E, 0x97B425E, 32),
    (0x9249249, 0x9249249, 32),
    (0x8D3DCB0, 0x8D3DCB0, 32),
    (0x8888888, 0x8888888, 32),
    (0x8421084, 0x8421084, 32),
    (1, 0, 5),
    (0x7C1F07C, 0x7C1F07C, 32),
    (0x7878787, 0x7878787, 32),
    (0x7507507, 0x7507507, 32),
    (0x71C71C7, 0x71C71C7, 32),
    (0x6EB3E45, 0x6EB3E45, 32),
    (0x6BCA1AF, 0x6BCA1AF, 32),
    (0x6906906, 0x6906906, 32),
    (0x6666666, 0x6666666, 32),
    (0x63E7063, 0x63E7063, 32),
    (0x6186186, 0x6186186, 32),
    (0x5F417D0, 0x5F417D0, 32),
    (0x5D1745D, 0x5D1745D, 32),
    (0x5B05B05, 0x5B05B05, 32),
    (0x590B216, 0x590B216, 32),
    (0x572620A, 0x572620A, 32),
    (0x5555555, 0x5555555, 32),
    (0x5397829, 0x5397829, 32),
    (0x51EB851, 0x51EB851, 32),
    (0x5050505, 0x5050505, 32),
    (0x4EC4EC4, 0x4EC4EC4, 32),
    (0x4D4873E, 0x4D4873E, 32),
    (0x4BDA12F, 0x4BDA12F, 32),
    (0x4A7904A, 0x4A7904A, 32),
    (0x4924924, 0x4924924, 32),
    (0x47DC11F, 0x47DC11F, 32),
    (0x469EE58, 0x469EE58, 32),
    (0x456C797, 0x456C797, 32),
    (0x4444444, 0x4444444, 32),
    (0x4325C53, 0x4325C53, 32),
    (0x4210842, 0x4210842, 32),
    (0x4104104, 0x4104104, 32),
    (1, 0, 6),
];

/// A compact list of integers with the given number of bits per entry.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BitStorage {
    pub data: Box<[u64]>,
    bits: usize,
    mask: u64,
    size: usize,
    values_per_long: usize,
    divide_mul: i32,
    divide_add: i32,
    divide_shift: u32,
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
    /// Create a new BitStorage with the given number of bits per entry. `size`
    /// is the number of entries in the BitStorage.
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
            divide_mul,
            divide_add,
            divide_shift,
        })
    }

    #[inline]
    fn cell_index(&self, index: u64) -> usize {
        let mul = self.divide_mul as u32 as u64;
        let add = self.divide_add as u32 as u64;
        let shift = self.divide_shift;

        (((index * mul) + add) >> shift) as usize
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
    fn test_protocol_wiki_example() {
        // https://minecraft.wiki/w/Java_Edition_protocol/Chunk_format#Visual_example

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

use std::{error::Error, fmt};

// this is from minecraft's code
// yeah idk either
const MAGIC: [(i32, i32, i32); 64] = [
    (-1, -1, 0),
    (-2147483648, 0, 0),
    (1431655765, 1431655765, 0),
    (-2147483648, 0, 1),
    (858993459, 858993459, 0),
    (715827882, 715827882, 0),
    (613566756, 613566756, 0),
    (-2147483648, 0, 2),
    (477218588, 477218588, 0),
    (429496729, 429496729, 0),
    (390451572, 390451572, 0),
    (357913941, 357913941, 0),
    (330382099, 330382099, 0),
    (306783378, 306783378, 0),
    (286331153, 286331153, 0),
    (-2147483648, 0, 3),
    (252645135, 252645135, 0),
    (238609294, 238609294, 0),
    (226050910, 226050910, 0),
    (214748364, 214748364, 0),
    (204522252, 204522252, 0),
    (195225786, 195225786, 0),
    (186737708, 186737708, 0),
    (178956970, 178956970, 0),
    (171798691, 171798691, 0),
    (165191049, 165191049, 0),
    (159072862, 159072862, 0),
    (153391689, 153391689, 0),
    (148102320, 148102320, 0),
    (143165576, 143165576, 0),
    (138547332, 138547332, 0),
    (-2147483648, 0, 4),
    (130150524, 130150524, 0),
    (126322567, 126322567, 0),
    (122713351, 122713351, 0),
    (119304647, 119304647, 0),
    (116080197, 116080197, 0),
    (113025455, 113025455, 0),
    (110127366, 110127366, 0),
    (107374182, 107374182, 0),
    (104755299, 104755299, 0),
    (102261126, 102261126, 0),
    (99882960, 99882960, 0),
    (97612893, 97612893, 0),
    (95443717, 95443717, 0),
    (93368854, 93368854, 0),
    (91382282, 91382282, 0),
    (89478485, 89478485, 0),
    (87652393, 87652393, 0),
    (85899345, 85899345, 0),
    (84215045, 84215045, 0),
    (82595524, 82595524, 0),
    (81037118, 81037118, 0),
    (79536431, 79536431, 0),
    (78090314, 78090314, 0),
    (76695844, 76695844, 0),
    (75350303, 75350303, 0),
    (74051160, 74051160, 0),
    (72796055, 72796055, 0),
    (71582788, 71582788, 0),
    (70409299, 70409299, 0),
    (69273666, 69273666, 0),
    (68174084, 68174084, 0),
    (-2147483648, 0, 5),
];

/// A compact list of integers with the given number of bits per entry.
#[derive(Clone, Debug, Default)]
pub struct BitStorage {
    pub data: Vec<u64>,
    bits: usize,
    mask: u64,
    size: usize,
    values_per_long: u8,
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
    pub fn new(bits: usize, size: usize, data: Option<Vec<u64>>) -> Result<Self, BitStorageError> {
        if let Some(data) = &data {
            // 0 bit storage
            if data.is_empty() {
                return Ok(BitStorage {
                    data: Vec::with_capacity(0),
                    bits,
                    size,
                    ..Default::default()
                });
            }
        }

        // vanilla has this assert but it's not always true for some reason??
        // assert!(bits >= 1 && bits <= 32);

        let values_per_long = 64 / bits;
        let magic_index = values_per_long - 1;
        let (divide_mul, divide_add, divide_shift) = MAGIC[magic_index];
        let calculated_length = (size + values_per_long - 1) / values_per_long;

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
            vec![0; calculated_length]
        };

        Ok(BitStorage {
            data: using_data,
            bits,
            mask,
            size,
            values_per_long: values_per_long as u8,
            divide_mul: divide_mul as u32 as u64,
            divide_add: divide_add as u32 as u64,
            divide_shift,
        })
    }

    pub fn cell_index(&self, index: u64) -> usize {
        // as unsigned wrap
        let first = self.divide_mul;
        let second = self.divide_add;

        (((index * first) + second) >> 32 >> self.divide_shift)
            .try_into()
            .unwrap()
    }

    pub fn get(&self, index: usize) -> u64 {
        // Validate.inclusiveBetween(0L, (long)(this.size - 1), (long)var1);
        // int var2 = this.cellIndex(var1);
        // long var3 = this.data[var2];
        // int var5 = (var1 - var2 * this.valuesPerLong) * this.bits;
        // return (int)(var3 >> var5 & this.mask);

        assert!(
            index < self.size,
            "Index {} out of bounds (must be less than {})",
            index,
            self.size
        );

        // 0 bit storage
        if self.data.is_empty() {
            return 0;
        }

        let cell_index = self.cell_index(index as u64);
        let cell = &self.data[cell_index];
        let bit_index = (index - cell_index * self.values_per_long as usize) * self.bits;
        cell >> bit_index & self.mask
    }

    pub fn get_and_set(&mut self, index: usize, value: u64) -> u64 {
        // 0 bit storage
        if self.data.is_empty() {
            return 0;
        }

        assert!(index < self.size);
        assert!(value <= self.mask);
        let cell_index = self.cell_index(index as u64);
        let cell = &mut self.data[cell_index];
        let bit_index = (index - cell_index * self.values_per_long as usize) * self.bits;
        let old_value = *cell >> (bit_index as u64) & self.mask;
        *cell = *cell & !(self.mask << bit_index) | (value & self.mask) << bit_index;
        old_value
    }

    pub fn set(&mut self, index: usize, value: u64) {
        // 0 bit storage
        if self.data.is_empty() {
            return;
        }

        assert!(index < self.size);
        assert!(value <= self.mask);
        let cell_index = self.cell_index(index as u64);
        let cell = &mut self.data[cell_index];
        let bit_index = (index - cell_index * self.values_per_long as usize) * self.bits;
        *cell = *cell & !(self.mask << bit_index) | (value & self.mask) << bit_index;
    }

    /// The number of entries.
    #[inline]
    pub fn size(&self) -> usize {
        self.size
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
        let storage = BitStorage::new(5, data.len(), Some(compact_data.to_vec())).unwrap();

        for (i, expected) in data.iter().enumerate() {
            assert_eq!(storage.get(i), *expected);
        }
    }
}

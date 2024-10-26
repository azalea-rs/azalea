use std::cmp::{self, Ordering};

use azalea_core::math::{gcd, lcm, EPSILON};

use super::CubePointRange;

#[derive(Debug)]
pub enum IndexMerger {
    Identical {
        coords: Vec<f64>,
    },
    DiscreteCube {
        result: CubePointRange,
        first_div: u32,
        second_div: u32,
    },
    NonOverlapping {
        lower: Vec<f64>,
        upper: Vec<f64>,
        swap: bool,
    },
    Indirect {
        result: Vec<f64>,
        first_indices: Vec<isize>,
        second_indices: Vec<isize>,
        result_length: usize,
    },
}

impl IndexMerger {
    pub fn get_list(&self) -> Vec<f64> {
        match self {
            IndexMerger::Identical { coords } => coords.clone(),
            IndexMerger::DiscreteCube { result, .. } => result.iter(),
            IndexMerger::NonOverlapping { lower, upper, .. } => (0..self.size())
                .map(|i| {
                    if i < lower.len() {
                        lower[i]
                    } else {
                        upper[i - lower.len()]
                    }
                })
                .collect(),
            IndexMerger::Indirect {
                result,
                result_length,
                ..
            } => {
                if *result_length <= 1 {
                    vec![]
                } else {
                    result[..*result_length].to_vec()
                }
            }
        }
    }
    pub fn for_merged_indexes(&self, mut consumer: impl IndexConsumer) -> bool {
        match self {
            IndexMerger::Identical { coords } => {
                for coord in 0..(coords.len() - 1) {
                    if !consumer(coord as i32, coord as i32, coord as i32) {
                        return false;
                    }
                }
                true
            }
            IndexMerger::DiscreteCube {
                result,
                first_div,
                second_div,
            } => {
                for var3 in 0..(result.size() - 1) {
                    if !consumer(
                        (var3 / second_div).try_into().unwrap(),
                        (var3 / first_div).try_into().unwrap(),
                        var3.try_into().unwrap(),
                    ) {
                        return false;
                    }
                }
                true
            }
            IndexMerger::NonOverlapping { lower, upper, swap } => {
                if *swap {
                    for_non_swapped_indexes(lower, upper, move |var1x, var2, var3| {
                        consumer(var2, var1x, var3)
                    })
                } else {
                    for_non_swapped_indexes(lower, upper, consumer)
                }
            }
            IndexMerger::Indirect {
                first_indices,
                second_indices,
                result_length,
                ..
            } => {
                let var2 = result_length - 1;

                for var3 in 0..var2 {
                    if !consumer(
                        first_indices[var3].try_into().unwrap(),
                        second_indices[var3].try_into().unwrap(),
                        var3.try_into().unwrap(),
                    ) {
                        return false;
                    }
                }

                true
            }
        }
    }
    pub fn size(&self) -> usize {
        match self {
            IndexMerger::Identical { coords } => coords.len(),
            IndexMerger::DiscreteCube { result, .. } => result.size().try_into().unwrap(),
            IndexMerger::NonOverlapping { lower, upper, .. } => lower.len() + upper.len(),
            IndexMerger::Indirect { result_length, .. } => *result_length,
        }
    }

    pub fn new_discrete_cube(a: u32, b: u32) -> Self {
        let result = CubePointRange {
            parts: (u32::try_from(lcm(a, b)).expect("lcm should be able to fit in a u32"))
                .try_into()
                .expect("lcm should not be 0"),
        };
        let gcd = gcd(a, b);
        let first_div = a / gcd;
        let second_div = b / gcd;
        Self::DiscreteCube {
            result,
            first_div,
            second_div,
        }
    }

    pub fn new_indirect(coords1: &[f64], coords2: &[f64], var3: bool, var4: bool) -> Self {
        let mut var5 = f64::NAN;
        let coords1_len = coords1.len();
        let coords2_len = coords2.len();
        let number_of_indices = coords1_len + coords2_len;
        let mut result = vec![0.0; number_of_indices];
        let mut first_indices: Vec<isize> = vec![0; number_of_indices];
        let mut second_indices: Vec<isize> = vec![0; number_of_indices];
        let var10 = !var3;
        let var11 = !var4;
        let mut var12 = 0;
        let mut coords1_index = 0;
        let mut coords2_index = 0;

        loop {
            let mut iterating_coords1: bool;
            loop {
                let at_end_of_coords1 = coords1_index >= coords1_len;
                let at_end_of_coords2 = coords2_index >= coords2_len;
                if at_end_of_coords1 && at_end_of_coords2 {
                    let result_length = cmp::max(1, var12);
                    return Self::Indirect {
                        result,
                        first_indices,
                        second_indices,
                        result_length,
                    };
                }

                iterating_coords1 = !at_end_of_coords1
                    && (at_end_of_coords2
                        || coords1[coords1_index] < coords2[coords2_index] + EPSILON);
                if iterating_coords1 {
                    coords1_index += 1;
                    if !var10 || coords2_index != 0 && !at_end_of_coords2 {
                        break;
                    }
                } else {
                    coords2_index += 1;
                    if !var11 || coords1_index != 0 && !at_end_of_coords1 {
                        break;
                    }
                }
            }

            let var18: isize = (coords1_index as isize) - 1;
            let var19: isize = (coords2_index as isize) - 1;
            let var20 = if iterating_coords1 {
                coords1[usize::try_from(var18).unwrap()]
            } else {
                coords2[usize::try_from(var19).unwrap()]
            };
            match var5.partial_cmp(&(var20 - EPSILON)) {
                None | Some(Ordering::Less) => {
                    result[var12] = var20;
                    first_indices[var12] = var18;
                    second_indices[var12] = var19;
                    var12 += 1;
                    var5 = var20;
                }
                _ => {
                    first_indices[var12 - 1] = var18;
                    second_indices[var12 - 1] = var19;
                }
            }
        }
    }
}

pub trait IndexConsumer = FnMut(i32, i32, i32) -> bool;

fn for_non_swapped_indexes(lower: &[f64], upper: &[f64], mut consumer: impl IndexConsumer) -> bool {
    let var2 = lower.len();
    for var3 in 0..var2 {
        if !consumer(var3.try_into().unwrap(), -1, var3.try_into().unwrap()) {
            return false;
        }
    }
    let var3 = upper.len() - 1;
    for var4 in 0..var3 {
        if !consumer(
            (var2 - 1).try_into().unwrap(),
            var4.try_into().unwrap(),
            (var2 + var4).try_into().unwrap(),
        ) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indirect_index_merger() {
        IndexMerger::new_indirect(&[0.0, 1.0], &[0.0, 0.5, 1.0], true, true);
    }
}

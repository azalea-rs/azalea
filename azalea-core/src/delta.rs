use std::io::{self, Cursor, Write};

pub use azalea_buf::AzBuf;
use azalea_buf::{AzaleaRead, AzaleaReadVar, AzaleaWrite, AzaleaWriteVar, BufReadError};

use crate::{math, position::Vec3};

pub trait PositionDeltaTrait {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
}

/// Only works for up to 8 blocks
#[derive(Clone, Debug, AzBuf, Default, PartialEq)]
pub struct PositionDelta8 {
    pub xa: i16,
    pub ya: i16,
    pub za: i16,
}

impl PositionDeltaTrait for PositionDelta8 {
    fn x(&self) -> f64 {
        (self.xa as f64) / 4096.0
    }
    fn y(&self) -> f64 {
        (self.ya as f64) / 4096.0
    }
    fn z(&self) -> f64 {
        (self.za as f64) / 4096.0
    }
}
impl<T: PositionDeltaTrait> From<T> for Vec3 {
    fn from(value: T) -> Self {
        Vec3::new(value.x(), value.y(), value.z())
    }
}

impl Vec3 {
    #[must_use]
    pub fn with_delta(&self, delta: &impl PositionDeltaTrait) -> Vec3 {
        Vec3 {
            x: self.x + delta.x(),
            y: self.y + delta.y(),
            z: self.z + delta.z(),
        }
    }

    pub fn normalize(&self) -> Vec3 {
        let length = f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
        if length < 1e-5 {
            return Vec3::ZERO;
        }
        Vec3 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    pub fn multiply(&self, x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x: self.x * x,
            y: self.y * y,
            z: self.z * z,
        }
    }
    pub fn scale(&self, amount: f64) -> Vec3 {
        self.multiply(amount, amount, amount)
    }
}

/// A variable-length representation of a position delta.
///
/// Can be freely converted to and from a [`Vec3`], but some precision will be
/// lost.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum LpVec3 {
    #[default]
    Zero,
    Normal {
        a: u8,
        b: u8,
        c: u32,
    },
    Extended {
        a: u8,
        b: u8,
        c: u32,
        d: u32,
    },
}

impl AzaleaRead for LpVec3 {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let a = u8::azalea_read(buf)?;
        if a == 0 {
            return Ok(LpVec3::Zero);
        }
        let b = u8::azalea_read(buf)?;
        let c = u32::azalea_read(buf)?;
        if a & 4 == 4 {
            let d = u32::azalea_read_var(buf)?;
            Ok(LpVec3::Extended { a, b, c, d })
        } else {
            Ok(LpVec3::Normal { a, b, c })
        }
    }
}
impl AzaleaWrite for LpVec3 {
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        match self {
            LpVec3::Zero => {
                0u8.azalea_write(buf)?;
            }
            LpVec3::Normal { a, b, c } => {
                a.azalea_write(buf)?;
                b.azalea_write(buf)?;
                c.azalea_write(buf)?;
            }
            LpVec3::Extended { a, b, c, d } => {
                a.azalea_write(buf)?;
                b.azalea_write(buf)?;
                c.azalea_write(buf)?;
                d.azalea_write_var(buf)?;
            }
        }
        Ok(())
    }
}
impl LpVec3 {
    pub fn from_vec3(vec3: Vec3) -> Self {
        let x = Self::sanitize(vec3.x);
        let y = Self::sanitize(vec3.y);
        let z = Self::sanitize(vec3.z);
        let max = x.abs().max(y.abs()).max(z.abs());
        if max < 3.051944088384301E-5 {
            return LpVec3::Zero;
        }

        let divisor = math::ceil_long(max);
        let is_extended = divisor & 3 != divisor;
        let packed_divisor = if is_extended {
            (divisor as u64 & 3) | 4
        } else {
            divisor as u64
        };
        let packed_x = Self::pack(x / (divisor as f64)) << 3;
        let packed_y = Self::pack(y / (divisor as f64)) << 18;
        let packed_z = Self::pack(z / (divisor as f64)) << 33;
        let packed = packed_divisor | packed_x | packed_y | packed_z;

        let a = packed as u8;
        let b = (packed >> 8) as u8;
        let c = (packed >> 16) as u32;

        if is_extended {
            let d = ((divisor as u64) >> 2) as u32;
            Self::Extended { a, b, c, d }
        } else {
            Self::Normal { a, b, c }
        }
    }

    pub fn to_vec3(self) -> Vec3 {
        match self {
            LpVec3::Zero => Vec3::ZERO,
            LpVec3::Normal { a, b, c } => {
                let packed: u64 = (c as u64) << 16 | (b as u64) << 8 | (a as u64);
                let multiplier = (a & 3) as u64 as f64;

                Vec3 {
                    x: Self::unpack(packed >> 3) * multiplier,
                    y: Self::unpack(packed >> 18) * multiplier,
                    z: Self::unpack(packed >> 33) * multiplier,
                }
            }
            LpVec3::Extended { a, b, c, d } => {
                let packed: u64 = (c as u64) << 16 | (b as u64) << 8 | (a as u64);
                let multiplier = (a & 3) as u64;
                let multiplier = multiplier | ((d as u64) << 2);
                let multiplier = multiplier as f64;

                Vec3 {
                    x: Self::unpack(packed >> 3) * multiplier,
                    y: Self::unpack(packed >> 18) * multiplier,
                    z: Self::unpack(packed >> 33) * multiplier,
                }
            }
        }
    }

    fn unpack(value: u64) -> f64 {
        f64::min((value & 32767) as f64, 32766.) * 2. / 32766. - 1.
    }

    fn pack(value: f64) -> u64 {
        f64::round((value * 0.5 + 0.5) * 32766.) as u64
    }

    fn sanitize(value: f64) -> f64 {
        if value.is_nan() {
            0.
        } else {
            f64::clamp(value, -1.7179869183E10, 1.7179869183E10)
        }
    }
}
impl From<LpVec3> for Vec3 {
    fn from(value: LpVec3) -> Self {
        value.to_vec3()
    }
}
impl From<Vec3> for LpVec3 {
    fn from(value: Vec3) -> Self {
        LpVec3::from_vec3(value)
    }
}
#[cfg(test)]
mod tests {
    use azalea_buf::AzaleaWrite;

    use super::*;

    static TEST_VALUES: [Vec3; 3] = [
        Vec3::ZERO,
        Vec3 {
            x: 1.234,
            y: -5.678,
            z: 9.876,
        },
        Vec3 {
            x: 10000000.,
            y: -5000000.,
            z: 9876543.,
        },
    ];

    #[test]
    fn test_lpvec3_roundtrip() {
        fn close_enough(a: f64, b: f64) -> bool {
            a == b || (a / b - 1.).abs() < 0.01
        }

        for v in TEST_VALUES {
            let lp = LpVec3::from_vec3(v);
            let v2 = lp.to_vec3();
            assert!(
                close_enough(v.x, v2.x) && close_enough(v.y, v2.y) && close_enough(v.z, v2.z),
                "Original: {:?}, Roundtrip: {:?}",
                v,
                v2
            );
        }
    }

    #[test]
    fn test_encode_decode_lpvec3() {
        for v in TEST_VALUES {
            let v: LpVec3 = LpVec3::from(v);
            let mut first_buf = Vec::new();
            v.azalea_write(&mut first_buf).unwrap();
            let decoded = LpVec3::azalea_read(&mut Cursor::new(&first_buf)).unwrap();
            assert_eq!(v, decoded);

            let mut second_buf = Vec::new();
            LpVec3::from(Vec3::from(decoded))
                .azalea_write(&mut second_buf)
                .unwrap();

            assert_eq!(first_buf, second_buf);
        }
    }
}

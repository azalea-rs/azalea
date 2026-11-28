use azalea_buf::AzBuf;
use azalea_codec::Codec;

use crate::position::Vec3f32;

#[derive(AzBuf, Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RgbColor {
    value: u32,
}
azalea_codec::impl_codec_from!(RgbColor, SerializableRgbColor, struct IncompleteRgbColor);

impl RgbColor {
    pub const BLACK: Self = Self { value: 0 };
    // this is max because that's how SpellParticleOption encodes the default
    pub const WHITE: Self = Self { value: u32::MAX };

    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            value: ((r as u32) << 16) | ((g as u32) << 8) | (b as u32),
        }
    }
    pub fn new_from_floats(r: f32, g: f32, b: f32) -> Self {
        Self::new((r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8)
    }

    pub fn red(&self) -> u8 {
        (self.value >> 16) as u8
    }
    pub fn green(&self) -> u8 {
        (self.value >> 8) as u8
    }
    pub fn blue(&self) -> u8 {
        self.value as u8
    }
}

#[derive(AzBuf, Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ArgbColor {
    value: u32,
}
azalea_codec::impl_codec_from!(ArgbColor, SerializableRgbColor, struct IncompleteArgbColor);

impl ArgbColor {
    pub fn new(a: u8, r: u8, g: u8, b: u8) -> Self {
        Self {
            value: ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | b as u32,
        }
    }
    pub fn new_from_floats(a: f32, r: f32, g: f32, b: f32) -> Self {
        Self::new(
            (a * 255.) as u8,
            (r * 255.) as u8,
            (g * 255.) as u8,
            (b * 255.) as u8,
        )
    }

    pub fn alpha(&self) -> u8 {
        (self.value >> 24) as u8
    }
    pub fn red(&self) -> u8 {
        (self.value >> 16) as u8
    }
    pub fn green(&self) -> u8 {
        (self.value >> 8) as u8
    }
    pub fn blue(&self) -> u8 {
        self.value as u8
    }
}

#[derive(Codec)]
enum SerializableRgbColor {
    Int(i32),
    Vec3f32(Vec3f32),
}
impl From<SerializableRgbColor> for RgbColor {
    fn from(r: SerializableRgbColor) -> Self {
        match r {
            SerializableRgbColor::Int(i) => Self { value: i as u32 },
            SerializableRgbColor::Vec3f32(Vec3f32 { x, y, z }) => Self::new_from_floats(x, y, z),
        }
    }
}
impl From<RgbColor> for SerializableRgbColor {
    fn from(r: RgbColor) -> Self {
        Self::Int(r.value as i32)
    }
}
impl From<SerializableRgbColor> for ArgbColor {
    fn from(r: SerializableRgbColor) -> Self {
        match r {
            SerializableRgbColor::Int(i) => Self { value: i as u32 },
            SerializableRgbColor::Vec3f32(Vec3f32 { x, y, z }) => {
                Self::new_from_floats(1., x, y, z)
            }
        }
    }
}
impl From<ArgbColor> for SerializableRgbColor {
    fn from(r: ArgbColor) -> Self {
        Self::Int(r.value as i32)
    }
}

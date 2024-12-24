use azalea_buf::AzBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, AzBuf)]
pub struct RgbColor {
    value: u32,
}

impl RgbColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            value: (r as u32) << 16 | (g as u32) << 8 | b as u32,
        }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, AzBuf)]
pub struct ArgbColor {
    value: u32,
}

impl ArgbColor {
    pub fn new(a: u8, r: u8, g: u8, b: u8) -> Self {
        Self {
            value: (a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32,
        }
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

use crate::packets::{McBufReadable, McBufWritable};
use azalea_chat::component::Component;
use azalea_core::{resource_location::ResourceLocation, Slot};
use packet_macros::{GamePacket, McBuf};
use std::{
    collections::HashMap,
    io::{Read, Write},
};

#[derive(Clone, Debug, McBuf, GamePacket)]
pub struct ClientboundUpdateAdvancementsPacket {
    pub reset: bool,
    pub added: HashMap<ResourceLocation, Advancement>,
    pub removed: Vec<ResourceLocation>,
    pub progress: HashMap<ResourceLocation, AdvancementProgress>,
}

#[derive(Clone, Debug, McBuf)]
pub struct Advancement {
    parent_id: Option<ResourceLocation>,
    display: Option<DisplayInfo>,
    // rewards: AdvancementRewards.EMPTY,
    criteria: HashMap<ResourceLocation, Criterion>,
    requirements: Vec<Vec<String>>,
    // requirements_strategy: RequirementsStrategy.AND
}

#[derive(Clone, Debug, McBuf)]
pub struct DisplayInfo {
    pub title: Component,
    pub description: Component,
    pub icon: Slot,
    pub frame: FrameType,
    pub flags: DisplayFlags,
    pub background: Option<ResourceLocation>,
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug)]
pub struct DisplayFlags {
    pub background: bool,
    pub show_toast: bool,
    pub hidden: bool,
}

impl McBufReadable for DisplayFlags {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        let data = u32::read_into(buf)?;
        Ok(DisplayFlags {
            background: (data & 0b1) != 0,
            show_toast: (data & 0b10) != 0,
            hidden: (data & 0b100) != 0,
        })
    }
}

impl McBufWritable for DisplayFlags {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let mut data = 0;
        if self.background {
            data |= 0b1;
        }
        if self.show_toast {
            data |= 0b10;
        }
        if self.hidden {
            data |= 0b100;
        }
        u32::write_into(&data, buf)
    }
}

#[derive(Clone, Debug, Copy, McBuf)]
pub enum FrameType {
    Task = 0,
    Challenge = 1,
    Goal = 2,
}

// nothing is written here
#[derive(Clone, Debug, McBuf)]
pub struct Criterion {}

pub type AdvancementProgress = HashMap<ResourceLocation, CriterionProgress>;

#[derive(Clone, Debug, McBuf)]
pub struct CriterionProgress {
    date: Option<u64>,
}

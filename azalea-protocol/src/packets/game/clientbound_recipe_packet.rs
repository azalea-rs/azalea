use crate::mc_buf::{McBufReadable, McBufWritable, Readable, Writable};
use azalea_core::{resource_location::ResourceLocation, Slot};
use packet_macros::{GamePacket, McBufReadable, McBufWritable};
use std::io::{Read, Write};

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundRecipePacket {
    pub action: State,
    pub settings: RecipeBookSettings,
    pub recipes: Vec<ResourceLocation>,
    pub to_highlight: Vec<ResourceLocation>,
}

#[derive(Clone, Debug, McBufReadable, McBufWritable)]
pub struct RecipeBookSettings {
    pub gui_open: bool,
    pub filtering_craftable: bool,

    pub furnace_gui_open: bool,
    pub furnace_filtering_craftable: bool,

    pub blast_furnace_gui_open: bool,
    pub blast_furnace_filtering_craftable: bool,

    pub smoker_gui_open: bool,
    pub smoker_filtering_craftable: bool,
}

#[derive(Clone, Debug, Copy)]
pub enum State {
    Init = 0,
    Add = 1,
    Remove = 2,
}

impl McBufWritable for State {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        buf.write_varint(*self as i32)?;
        Ok(())
    }
}
impl McBufReadable for State {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        let state = buf.read_varint()?.try_into().unwrap();
        Ok(match state {
            0 => State::Init,
            1 => State::Add,
            2 => State::Remove,
            _ => panic!("Invalid state: {}", state),
        })
    }
}

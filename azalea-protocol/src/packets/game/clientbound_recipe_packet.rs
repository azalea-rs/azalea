use async_trait::async_trait;
use azalea_chat::component::Component;
use azalea_core::{resource_location::ResourceLocation, Slot};
use packet_macros::{GamePacket, McBufReadable, McBufWritable};
use tokio::io::AsyncRead;

use crate::mc_buf::{McBufReadable, McBufWritable, Readable, Writable};

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
    fn write_into(&self, buf: &mut Vec<u8>) -> Result<(), std::io::Error> {
        buf.write_varint(*self as i32)?;
        Ok(())
    }
}
#[async_trait]
impl McBufReadable for State {
    async fn read_into<R>(buf: &mut R) -> Result<Self, String>
    where
        R: AsyncRead + std::marker::Unpin + std::marker::Send,
    {
        let state = buf.read_varint().await?.try_into().unwrap();
        Ok(match state {
            0 => State::Init,
            1 => State::Add,
            2 => State::Remove,
            _ => panic!("Invalid state: {}", state),
        })
    }
}

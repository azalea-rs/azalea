use azalea_buf::{
    BufReadError, McBuf, McBufReadable, McBufVarReadable, McBufVarWritable, McBufWritable,
};
use azalea_core::ResourceLocation;
use azalea_protocol_macros::ClientboundGamePacket;
use std::io::{Cursor, Write};

#[derive(Clone, Debug, ClientboundGamePacket)]
pub struct ClientboundRecipePacket {
    pub action: State,
    pub settings: RecipeBookSettings,
    pub recipes: Vec<ResourceLocation>,
}

impl McBufWritable for ClientboundRecipePacket {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match self.action {
            State::Init { .. } => 0,
            State::Add => 1,
            State::Remove => 2,
        }
        .var_write_into(buf)?;
        self.settings.write_into(buf)?;
        self.recipes.write_into(buf)?;
        if let State::Init { to_highlight } = &self.action {
            to_highlight.write_into(buf)?;
        }
        Ok(())
    }
}
impl McBufReadable for ClientboundRecipePacket {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, azalea_buf::BufReadError> {
        let action_id = u32::var_read_from(buf)?;
        let settings = RecipeBookSettings::read_from(buf)?;
        let recipes = Vec::<ResourceLocation>::read_from(buf)?;
        let action = match action_id {
            0 => State::Init {
                to_highlight: Vec::<ResourceLocation>::read_from(buf)?,
            },
            1 => State::Add,
            2 => State::Remove,
            _ => {
                return Err(BufReadError::UnexpectedEnumVariant {
                    id: action_id as i32,
                })
            }
        };

        Ok(ClientboundRecipePacket {
            action,
            settings,
            recipes,
        })
    }
}

#[derive(Clone, Debug, McBuf)]
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

#[derive(Clone, Debug)]
pub enum State {
    Init { to_highlight: Vec<ResourceLocation> },
    Add,
    Remove,
}

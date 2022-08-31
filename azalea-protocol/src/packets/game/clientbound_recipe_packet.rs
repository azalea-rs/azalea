use azalea_buf::McBuf;
use azalea_core::ResourceLocation;
use packet_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundRecipePacket {
    pub action: State,
    pub settings: RecipeBookSettings,
    pub recipes: Vec<ResourceLocation>,
    pub to_highlight: Vec<ResourceLocation>,
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

#[derive(Clone, Debug, Copy, McBuf)]
pub enum State {
    Init = 0,
    Add = 1,
    Remove = 2,
}

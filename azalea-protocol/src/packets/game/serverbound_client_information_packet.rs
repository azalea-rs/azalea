use azalea_buf::McBuf;
use packet_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundClientInformationPacket {
    pub language: String,
    pub view_distance: u8,
    pub chat_visibility: ChatVisiblity,
    pub chat_colors: bool,
    pub model_customisation: u8,
    pub main_hand: HumanoidArm,
    pub text_filtering_enabled: bool,
    pub allows_listing: bool,
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum ChatVisiblity {
    Full = 0,
    System = 1,
    Hidden = 2,
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum HumanoidArm {
    Left = 0,
    Right = 1,
}

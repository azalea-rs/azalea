use azalea_buf::McBuf;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundClientInformationPacket {
    /// The locale of the client.
    pub language: String,
    /// The view distance of the client in chunks, same as the render distance
    /// in-game.
    pub view_distance: u8,
    /// The types of chat messages the client wants to receive. Note that many
    /// servers ignore this.
    pub chat_visibility: ChatVisibility,
    /// Whether the messages sent from the server should have colors. Note that
    /// many servers ignore this and always send colored messages.
    pub chat_colors: bool,
    pub model_customisation: u8,
    pub main_hand: HumanoidArm,
    pub text_filtering_enabled: bool,
    /// Whether the client should show up as "Anonymous Player" in the server
    /// list.
    pub allows_listing: bool,
}

impl Default for ServerboundClientInformationPacket {
    fn default() -> Self {
        Self {
            language: "en_us".to_string(),
            view_distance: 8,
            chat_visibility: ChatVisibility::Full,
            chat_colors: true,
            model_customisation: 0,
            main_hand: HumanoidArm::Right,
            text_filtering_enabled: false,
            allows_listing: false,
        }
    }
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum ChatVisibility {
    /// All chat messages should be sent to the client.
    Full = 0,
    /// Chat messages from other players should be not sent to the client, only
    /// messages from the server like "Player joined the game" should be sent.
    System = 1,
    /// No chat messages should be sent to the client.
    Hidden = 2,
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum HumanoidArm {
    Left = 0,
    Right = 1,
}

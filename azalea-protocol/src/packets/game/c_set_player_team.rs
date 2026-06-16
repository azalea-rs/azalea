use azalea_buf::AzBuf;
use azalea_chat::{FormattedText, style::ChatFormatting};
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundSetPlayerTeam {
    pub name: String,
    pub method: Method,
}

#[derive(AzBuf, Clone, Debug, PartialEq)]
pub enum Method {
    Add((Parameters, PlayerList)),
    Remove,
    Change(Parameters),
    Join(PlayerList),
    Leave(PlayerList),
}

#[derive(AzBuf, Clone, Debug, PartialEq)]
pub struct Parameters {
    pub display_name: FormattedText,
    pub player_prefix: FormattedText,
    pub player_suffix: FormattedText,
    pub nametag_visibility: NameTagVisibility,
    pub collision_rule: CollisionRule,
    pub color: ChatFormatting,
    pub options: u8,
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq)]
pub enum CollisionRule {
    Always,
    Never,
    PushOtherTeams,
    PushOwnTeam,
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq)]
pub enum NameTagVisibility {
    Always,
    Never,
    HideForOtherTeams,
    HideForOwnTeam,
}

type PlayerList = Vec<String>;

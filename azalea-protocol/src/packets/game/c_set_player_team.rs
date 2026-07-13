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
    pub color: Option<ChatFormatting>,
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

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use azalea_buf::AzBuf;

    use crate::packets::game::ClientboundSetPlayerTeam;

    #[test]
    fn read_set_player_team() {
        // from hypixel
        #[rustfmt::skip]
        let contents = [12, 97, 56, 49, 56, 53, 51, 54, 100, 50, 98, 52, 52, 0, 10, 8, 0, 5, 99, 111, 108, 111, 114, 0, 5, 119, 104, 105, 116, 101, 8, 0, 4, 116, 101, 120, 116, 0, 12, 97, 56, 49, 56, 53, 51, 54, 100, 50, 98, 52, 52, 0, 10, 8, 0, 4, 116, 101, 120, 116, 0, 16, 194, 167, 97, 91, 86, 73, 80, 194, 167, 54, 43, 194, 167, 97, 93, 32, 0, 10, 8, 0, 4, 116, 101, 120, 116, 0, 0, 0, 0, 1, 1, 10, 3, 3, 10, 72, 97, 109, 97, 75, 105, 108, 108, 101, 114, 6, 90, 120, 121, 49, 111, 78, 10, 116, 97, 109, 97, 108, 101, 115, 102, 97, 109];
        let mut buf = Cursor::new(contents.as_slice());
        let _packet = ClientboundSetPlayerTeam::azalea_read(&mut buf).unwrap();

        assert_eq!(buf.position(), contents.len() as u64);
    }
}

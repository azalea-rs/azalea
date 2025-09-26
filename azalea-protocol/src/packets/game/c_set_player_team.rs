use azalea_buf::AzBuf;
use azalea_chat::{FormattedText, style::ChatFormatting};
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundSetPlayerTeam {
    pub name: String,
    pub method: Method,
}

#[derive(Clone, Debug, AzBuf, PartialEq)]
pub enum Method {
    Add((Parameters, PlayerList)),
    Remove,
    Change(Parameters),
    Join(PlayerList),
    Leave(PlayerList),
}

#[derive(Clone, Debug, AzBuf, PartialEq)]
pub struct Parameters {
    pub display_name: FormattedText,
    pub options: u8,
    pub nametag_visibility: NameTagVisibility,
    pub collision_rule: CollisionRule,
    pub color: ChatFormatting,
    pub player_prefix: FormattedText,
    pub player_suffix: FormattedText,
}

#[derive(Clone, Copy, Debug, AzBuf, PartialEq)]
pub enum CollisionRule {
    Always,
    Never,
    PushOtherTeams,
    PushOwnTeam,
}

#[derive(Clone, Copy, Debug, AzBuf, PartialEq)]
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

    use azalea_buf::AzaleaRead;

    use crate::packets::game::ClientboundSetPlayerTeam;

    #[test]
    fn test_read_set_player_team() {
        let contents = [
            16, 99, 111, 108, 108, 105, 100, 101, 82, 117, 108, 101, 95, 57, 52, 53, 54, 0, 8, 0,
            16, 99, 111, 108, 108, 105, 100, 101, 82, 117, 108, 101, 95, 57, 52, 53, 54, 1, 0, 1,
            21, 8, 0, 0, 8, 0, 0, 0,
        ];
        let mut buf = Cursor::new(contents.as_slice());
        let packet = ClientboundSetPlayerTeam::azalea_read(&mut buf).unwrap();
        println!("{packet:?}");

        assert_eq!(buf.position(), contents.len() as u64);
    }
}

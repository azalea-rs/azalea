use std::io::{self, Cursor, Write};

use azalea_buf::AzBuf;
use azalea_chat::FormattedText;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundMapItemData {
    #[var]
    pub map_id: u32,
    pub scale: u8,
    pub locked: bool,
    pub decorations: Option<Vec<MapDecoration>>,
    pub color_patch: OptionalMapPatch,
}

#[derive(AzBuf, Clone, Debug, PartialEq)]
pub struct MapDecoration {
    pub decoration_type: DecorationType,
    pub x: i8,
    pub y: i8,
    /// Minecraft does `& 15` on this value and azalea-protocol doesn't. I don't
    /// think it matters.
    pub rot: i8,
    pub name: Option<FormattedText>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OptionalMapPatch(pub Option<MapPatch>);

impl AzBuf for OptionalMapPatch {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, azalea_buf::BufReadError> {
        let pos = buf.position();
        Ok(Self(if u8::azalea_read(buf)? == 0 {
            None
        } else {
            buf.set_position(pos);
            Some(MapPatch::azalea_read(buf)?)
        }))
    }
    fn azalea_write(&self, buf: &mut impl Write) -> io::Result<()> {
        match &self.0 {
            None => 0u8.azalea_write(buf),
            Some(m) => m.azalea_write(buf),
        }
    }
}

#[derive(AzBuf, Clone, Debug, PartialEq)]
pub struct MapPatch {
    pub width: u8,
    pub height: u8,
    pub start_x: u8,
    pub start_y: u8,
    pub map_colors: Vec<u8>,
}

#[derive(AzBuf, Clone, Copy, Debug, PartialEq)]
pub enum DecorationType {
    Player,
    Frame,
    RedMarker,
    BlueMarker,
    TargetX,
    TargetPoint,
    PlayerOffMap,
    PlayerOffLimits,
    Mansion,
    Monument,
    BannerWhite,
    BannerOrange,
    BannerMagenta,
    BannerLightBlue,
    BannerYellow,
    BannerLime,
    BannerPink,
    BannerGray,
    BannerLightGray,
    BannerCyan,
    BannerPurple,
    BannerBlue,
    BannerBrown,
    BannerGreen,
    BannerRed,
    BannerBlack,
    RedX,
    VillageDesert,
    VillagePlains,
    VillageSavanna,
    VillageSnowy,
    VillageTaiga,
    JungleTemple,
    SwampHut,
    TrialChambers,
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use azalea_buf::AzBuf;

    use super::*;

    #[test]
    fn test_read_trial_chambers_decoration() {
        // map 0, scale 0, unlocked, 1 decoration, no color patch
        let bytes = [0, 0, 0, 1, 1, 34, 5, 6, 8, 0, 0];
        let mut buf = Cursor::new(bytes.as_slice());
        let packet = ClientboundMapItemData::azalea_read(&mut buf).unwrap();
        let decoration = &packet.decorations.unwrap()[0];
        assert_eq!(decoration.decoration_type, DecorationType::TrialChambers);
        assert_eq!((decoration.x, decoration.y, decoration.rot), (5, 6, 8));
        assert_eq!(buf.position() as usize, bytes.len());
    }
}

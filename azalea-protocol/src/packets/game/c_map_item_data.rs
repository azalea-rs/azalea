use std::io::{self, Cursor, Write};

use azalea_buf::{AzBuf, AzaleaRead, AzaleaWrite};
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

impl AzaleaRead for OptionalMapPatch {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, azalea_buf::BufReadError> {
        let pos = buf.position();
        Ok(Self(if u8::azalea_read(buf)? == 0 {
            None
        } else {
            buf.set_position(pos);
            Some(MapPatch::azalea_read(buf)?)
        }))
    }
}

impl AzaleaWrite for OptionalMapPatch {
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
}

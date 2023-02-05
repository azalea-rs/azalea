use azalea_buf::{McBuf, McBufReadable, McBufWritable};
use azalea_chat::FormattedText;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, ClientboundGamePacket, McBuf)]
pub struct ClientboundMapItemDataPacket {
    #[var]
    pub map_id: u32,
    pub scale: u8,
    pub locked: bool,
    pub decorations: Option<Vec<MapDecoration>>,
    pub color_patch: OptionalMapPatch,
}

#[derive(Clone, Debug, McBuf)]
pub struct MapDecoration {
    pub decoration_type: DecorationType,
    pub x: i8,
    pub y: i8,
    /// Minecraft does & 15 on this value, azalea-protocol doesn't. I don't
    /// think it matters.
    pub rot: i8,
    pub name: Option<FormattedText>,
}

#[derive(Debug, Clone)]
pub struct OptionalMapPatch(pub Option<MapPatch>);

impl McBufReadable for OptionalMapPatch {
    fn read_from(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, azalea_buf::BufReadError> {
        let pos = buf.position();
        Ok(Self(if u8::read_from(buf)? == 0 {
            None
        } else {
            buf.set_position(pos);
            Some(MapPatch::read_from(buf)?)
        }))
    }
}

impl McBufWritable for OptionalMapPatch {
    fn write_into(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        match &self.0 {
            None => 0u8.write_into(buf),
            Some(m) => m.write_into(buf),
        }
    }
}

#[derive(Debug, Clone, McBuf)]
pub struct MapPatch {
    pub width: u8,
    pub height: u8,
    pub start_x: u8,
    pub start_y: u8,
    pub map_colors: Vec<u8>,
}

#[derive(Clone, Copy, Debug, McBuf)]
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

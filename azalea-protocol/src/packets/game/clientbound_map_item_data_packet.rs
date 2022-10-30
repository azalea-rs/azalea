use azalea_buf::{BufReadError, McBuf};
use azalea_buf::{McBufReadable, McBufVarReadable, McBufVarWritable, McBufWritable};
use azalea_chat::Component;
use azalea_protocol_macros::ClientboundGamePacket;
use std::io::{Cursor, Write};

#[derive(Clone, Debug, ClientboundGamePacket)]
pub struct ClientboundMapItemDataPacket {
    // #[var]
    pub map_id: u32,
    pub scale: u8,
    pub locked: bool,
    pub decorations: Vec<MapDecoration>,
    pub color_patch: Option<MapPatch>,
}

impl McBufReadable for ClientboundMapItemDataPacket {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let map_id = u32::var_read_from(buf)?;
        let scale = u8::read_from(buf)?;
        let locked = bool::read_from(buf)?;
        let decorations = Option::<Vec<MapDecoration>>::read_from(buf)?.unwrap_or_default();

        let width = u8::read_from(buf)?;
        let color_patch = if width == 0 {
            None
        } else {
            let height = u8::read_from(buf)?;
            let start_x = u8::read_from(buf)?;
            let start_y = u8::read_from(buf)?;
            let map_colors = Vec::<u8>::read_from(buf)?;
            Some(MapPatch {
                width,
                height,
                start_x,
                start_y,
                map_colors,
            })
        };

        Ok(Self {
            map_id,
            scale,
            locked,
            decorations,
            color_patch,
        })
    }
}

impl McBufWritable for ClientboundMapItemDataPacket {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        self.map_id.var_write_into(buf)?;
        self.scale.write_into(buf)?;
        self.locked.write_into(buf)?;
        (!self.decorations.is_empty()).write_into(buf)?;
        self.decorations.write_into(buf)?;
        if let Some(color_patch) = &self.color_patch {
            color_patch.width.write_into(buf)?;
            color_patch.height.write_into(buf)?;
            color_patch.start_x.write_into(buf)?;
            color_patch.start_y.write_into(buf)?;
            color_patch.map_colors.write_into(buf)?;
        } else {
            0u8.write_into(buf)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, McBuf)]
pub struct MapDecoration {
    pub decoration_type: DecorationType,
    pub x: i8,
    pub y: i8,
    /// Minecraft does & 15 on this value, azalea-protocol doesn't. I don't think it matters.
    pub rot: i8,
    pub name: Option<Component>,
}

#[derive(Debug, Clone)]
pub struct MapPatch {
    pub start_x: u8,
    pub start_y: u8,
    pub width: u8,
    pub height: u8,
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

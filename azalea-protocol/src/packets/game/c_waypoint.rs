use std::io::{self, Cursor, Write};

use azalea_buf::{AzBuf, AzaleaRead, AzaleaWrite, BufReadError};
use azalea_core::{color::RgbColor, position::Vec3i, resource_location::ResourceLocation};
use azalea_protocol_macros::ClientboundGamePacket;
use uuid::Uuid;

#[derive(Clone, Debug, AzBuf, PartialEq, ClientboundGamePacket)]
pub struct ClientboundWaypoint {
    pub operation: WaypointOperation,
    pub waypoint: TrackedWaypoint,
}

#[derive(AzBuf, Copy, Clone, Debug, PartialEq)]
pub enum WaypointOperation {
    Track,
    Untrack,
    Update,
}

#[derive(AzBuf, Clone, Debug, PartialEq)]
pub struct TrackedWaypoint {
    pub identifier: WaypointIdentifier,
    pub icon: WaypointIcon,
    pub data: WaypointData,
}

#[derive(AzBuf, Clone, Debug, PartialEq)]
pub enum WaypointIdentifier {
    String(String),
    Uuid(Uuid),
}

#[derive(Clone, Debug, PartialEq)]
pub struct WaypointIcon {
    pub style: ResourceLocation,
    pub color: Option<RgbColor>,
}
impl AzaleaWrite for WaypointIcon {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), io::Error> {
        self.style.azalea_write(buf)?;
        let color = self.color.map(|c| CompactRgbColor {
            r: c.red(),
            g: c.green(),
            b: c.blue(),
        });
        color.azalea_write(buf)?;
        Ok(())
    }
}
impl AzaleaRead for WaypointIcon {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let style = ResourceLocation::azalea_read(buf)?;
        let color = Option::<CompactRgbColor>::azalea_read(buf)?;
        let color = color.map(|c| RgbColor::new(c.r, c.g, c.b));

        Ok(Self { style, color })
    }
}

// usually RgbColor is encoded as 4 bytes, except here where it's 3
#[derive(AzBuf)]
struct CompactRgbColor {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(AzBuf, Clone, Debug, PartialEq)]
pub enum WaypointData {
    Empty,
    Vec3i(Vec3i),
    Chunk {
        #[var]
        x: i32,
        #[var]
        z: i32,
    },
    Azimuth {
        angle: f32,
    },
}

// (wg stands for worldgen)

use std::{
    fmt::{self, Display},
    str::FromStr,
};

use azalea_buf::AzBuf;

/// A type of world heightmap.
///
/// See `azalea_world::heightmap` for more info.
#[derive(AzBuf, Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum HeightmapKind {
    WorldSurfaceWg,
    WorldSurface,
    OceanFloorWg,
    OceanFloor,
    MotionBlocking,
    MotionBlockingNoLeaves,
}

impl FromStr for HeightmapKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "WORLD_SURFACE_WG" => Ok(HeightmapKind::WorldSurfaceWg),
            "WORLD_SURFACE" => Ok(HeightmapKind::WorldSurface),
            "OCEAN_FLOOR_WG" => Ok(HeightmapKind::OceanFloorWg),
            "OCEAN_FLOOR" => Ok(HeightmapKind::OceanFloor),
            "MOTION_BLOCKING" => Ok(HeightmapKind::MotionBlocking),
            "MOTION_BLOCKING_NO_LEAVES" => Ok(HeightmapKind::MotionBlockingNoLeaves),
            _ => Err(()),
        }
    }
}

impl Display for HeightmapKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HeightmapKind::WorldSurfaceWg => write!(f, "WORLD_SURFACE_WG"),
            HeightmapKind::WorldSurface => write!(f, "WORLD_SURFACE"),
            HeightmapKind::OceanFloorWg => write!(f, "OCEAN_FLOOR_WG"),
            HeightmapKind::OceanFloor => write!(f, "OCEAN_FLOOR"),
            HeightmapKind::MotionBlocking => write!(f, "MOTION_BLOCKING"),
            HeightmapKind::MotionBlockingNoLeaves => write!(f, "MOTION_BLOCKING_NO_LEAVES"),
        }
    }
}

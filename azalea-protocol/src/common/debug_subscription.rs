use std::fmt::Debug;

use azalea_buf::AzBuf;
use azalea_core::position::{BlockPos, Vec3};
use azalea_registry::{Block, GameEvent, PointOfInterestKind};

// see DebugSubscriptions.java
macro_rules! debug_subscription_enum {
    ($name:ident, $ty: ident) => {
        #[derive(Clone, Debug, AzBuf)]
        pub enum $name {
            DedicatedServerTickTime($ty<()>),
            Bees($ty<DebugBeeInfo>),
            Brains($ty<DebugBrainDump>),
            Breezes($ty<DebugBreezeInfo>),
            GoalSelectors($ty<DebugGoalInfo>),
            EntityPaths($ty<DebugPathInfo>),
            EntityBlockIntersections($ty<DebugEntityBlockIntersection>),
            BeeHives($ty<DebugHiveInfo>),
            Pois($ty<DebugPoiInfo>),
            RedstoneWireOrientations($ty<DebugRedstoneOrientation>),
            VillageSections($ty<()>),
            Raids($ty<Vec<BlockPos>>),
            Structures($ty<Vec<DebugStructureInfo>>),
            GameEventListeners($ty<DebugGameEventListenerInfo>),
            NeighborUpdates($ty<BlockPos>),
            GameEvents($ty<DebugGameEventInfo>),
        }
    };
}

// we need the values to exist as required and optional, so we create two nearly
// identical enums with a macro
debug_subscription_enum! { DebugSubscriptionEvent, Passthrough }
type Passthrough<T> = T;
debug_subscription_enum! { DebugSubscriptionUpdate, Option }

#[derive(Clone, Debug, AzBuf)]
pub struct DebugBeeInfo {
    pub hive_pos: Option<BlockPos>,
    pub flower_pos: Option<BlockPos>,
    #[var]
    pub travel_ticks: i32,
    pub blacklisted_hives: Vec<BlockPos>,
}
#[derive(Clone, Debug, AzBuf)]
pub struct DebugBrainDump {
    pub name: String,
    pub profession: String,
    pub xp: i32,
    pub health: f32,
    pub max_health: f32,
    pub inventory: String,
    pub wants_golem: bool,
    pub anger_level: i32,
    pub activities: Vec<String>,
    pub behaviors: Vec<String>,
    pub memories: Vec<String>,
    pub gossips: Vec<String>,
    pub pois: Vec<BlockPos>,
    pub potential_pois: Vec<BlockPos>,
}

#[derive(Clone, Debug, AzBuf)]
pub struct DebugBreezeInfo {
    #[var]
    pub attack_target: Option<i32>,
    pub jump_target: Option<BlockPos>,
}

#[derive(Clone, Debug, AzBuf)]
pub struct DebugGoalInfo {
    #[var]
    pub priority: i32,
    pub is_running: bool,
    #[limit(255)]
    pub name: String,
}

#[derive(Clone, Debug, AzBuf)]
pub struct DebugPathInfo {
    pub path: MinecraftPath,
    pub max_node_distance: f32,
}

#[derive(Clone, Copy, Debug, AzBuf)]
pub enum DebugEntityBlockIntersection {
    InBlock,
    InFluid,
    InAir,
}

#[derive(Clone, Debug, AzBuf)]
pub struct DebugHiveInfo {
    pub kind: Block,
    #[var]
    pub occupant_count: i32,
    #[var]
    pub honey_level: i32,
    pub sedated: bool,
}

#[derive(Clone, Debug, AzBuf)]
pub struct DebugPoiInfo {
    pub pos: BlockPos,
    pub poi_kind: PointOfInterestKind,
    #[var]
    pub free_ticket_count: i32,
}

#[derive(Clone, Debug, AzBuf)]
pub struct DebugRedstoneOrientation {
    #[var]
    pub id: u32,
}

#[derive(Clone, Debug, AzBuf)]
pub struct DebugStructureInfo {
    pub bounding_box: StructureBoundingBox,
    pub pieces: Vec<StructurePiece>,
}

#[derive(Clone, Debug, AzBuf)]
pub struct DebugGameEventListenerInfo {
    #[var]
    pub listener_radius: i32,
}

#[derive(Clone, Debug, AzBuf)]
pub struct DebugGameEventInfo {
    pub event: GameEvent,
    pub pos: Vec3,
}

#[derive(Clone, Debug, AzBuf)]
pub struct StructureBoundingBox {
    pub min: BlockPos,
    pub max: BlockPos,
}

#[derive(Clone, Debug, AzBuf)]
pub struct StructurePiece {
    pub bounding_box: StructureBoundingBox,
    pub is_start: bool,
}

#[derive(Clone, Debug, AzBuf)]
pub struct MinecraftPath {
    pub reached: bool,
    pub next_node_index: i32,
    pub block_pos: BlockPos,
    pub nodes: Vec<MinecraftPathNode>,
    pub debug_data: MinecraftPathDebugData,
}

#[derive(Clone, Debug, AzBuf)]
pub struct MinecraftPathNode {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub contents: MinecraftPathNodeContents,
}

#[derive(Clone, Debug, AzBuf)]
pub struct MinecraftPathNodeContents {
    pub walked_distance: f32,
    pub cost_malus: f32,
    pub closed: bool,
    pub kind: MinecraftPathNodeKind,
    pub f: f32,
}

// PathType.java
#[derive(Clone, Copy, Debug, AzBuf)]
pub enum MinecraftPathNodeKind {
    Blocked,
    Open,
    Walkable,
    WalkableDoor,
    Trapdoor,
    PowderSnow,
    DangerPowderSnow,
    Fence,
    Lava,
    Water,
    WaterBorder,
    Rail,
    UnpassableRail,
    DangerFire,
    DamageFire,
    DangerOther,
    DamageOther,
    DoorOpen,
    DoorWoodClosed,
    DoorIronClosed,
    Breach,
    Leaves,
    StickyHoney,
    Cocoa,
    DamageCautious,
    DangerTrapdoor,
}

#[derive(Clone, Debug, AzBuf)]
pub struct MinecraftPathDebugData {
    pub target_nodes: Vec<MinecraftPathNode>,
    pub open_set: Vec<MinecraftPathNode>,
    pub closed_set: Vec<MinecraftPathNode>,
}

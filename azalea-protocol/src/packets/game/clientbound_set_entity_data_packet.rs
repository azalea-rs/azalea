use crate::{
    mc_buf::{Readable, Writable},
    packets::{McBufReadable, McBufWritable},
};
use azalea_chat::component::Component;
use azalea_core::{BlockPos, Direction, Slot};
use packet_macros::{GamePacket, McBufReadable, McBufWritable};
use std::io::{Read, Write};
use uuid::Uuid;

#[derive(Clone, Debug, GamePacket)]
pub struct ClientboundSetEntityDataPacket {
    #[varint]
    pub id: i32,
    pub metadata: Vec<EntityDataItem>,
}

#[derive(Clone, Debug)]
pub struct EntityDataItem {
    // we can't identify what the index is for here because we don't know the
    // entity type
    pub index: u8,
    pub value: EntityDataValue,
}

impl McBufReadable for Vec<EntityDataItem> {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        let mut metadata = Vec::new();
        loop {
            let index = buf.read_byte()?;
            if index == 0xff {
                break;
            }
            let value = EntityDataValue::read_into(buf)?;
            metadata.push(EntityDataItem { index, value });
        }
        Ok(metadata)
    }
}

impl McBufWritable for Vec<EntityDataItem> {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        for item in self {
            buf.write_byte(item.index)?;
            item.value.write_into(buf)?;
        }
        buf.write_byte(0xff)?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum EntityDataValue {
    Byte(u8),
    // varint
    Int(i32),
    Float(f32),
    String(String),
    Component(Component),
    OptionalComponent(Option<Component>),
    ItemStack(Slot),
    Boolean(bool),
    Rotations { x: f32, y: f32, z: f32 },
    BlockPos(BlockPos),
    OptionalBlockPos(Option<BlockPos>),
    Direction(Direction),
    OptionalUuid(Option<Uuid>),
    // 0 for absent (implies air); otherwise, a block state ID as per the global palette
    // this is a varint
    OptionalBlockState(Option<i32>),
    CompoundTag(azalea_nbt::Tag),
    Particle(Particle),
    VillagerData(VillagerData),
    // 0 for absent; 1 + actual value otherwise. Used for entity IDs.
    OptionalUnsignedInt(Option<u32>),
    Pose(Pose),
}

impl McBufReadable for EntityDataValue {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        let type_ = buf.read_varint()?;
        Ok(match type_ {
            0 => EntityDataValue::Byte(buf.read_byte()?),
            1 => EntityDataValue::Int(buf.read_varint()?),
            2 => EntityDataValue::Float(buf.read_float()?),
            3 => EntityDataValue::String(buf.read_utf()?),
            4 => EntityDataValue::Component(Component::read_into(buf)?),
            5 => EntityDataValue::OptionalComponent(Option::<Component>::read_into(buf)?),
            6 => EntityDataValue::ItemStack(Slot::read_into(buf)?),
            7 => EntityDataValue::Boolean(buf.read_boolean()?),
            8 => EntityDataValue::Rotations {
                x: buf.read_float()?,
                y: buf.read_float()?,
                z: buf.read_float()?,
            },
            9 => EntityDataValue::BlockPos(BlockPos::read_into(buf)?),
            10 => EntityDataValue::OptionalBlockPos(Option::<BlockPos>::read_into(buf)?),
            11 => EntityDataValue::Direction(Direction::read_into(buf)?),
            12 => EntityDataValue::OptionalUuid(Option::<Uuid>::read_into(buf)?),
            13 => EntityDataValue::OptionalBlockState({
                let val = i32::read_into(buf)?;
                if val == 0 {
                    None
                } else {
                    Some(val)
                }
            }),
            14 => EntityDataValue::CompoundTag(azalea_nbt::Tag::read_into(buf)?),
            15 => EntityDataValue::Particle(Particle::read_into(buf)?),
            16 => EntityDataValue::VillagerData(VillagerData::read_into(buf)?),
            17 => EntityDataValue::OptionalUnsignedInt({
                let val = buf.read_varint()?;
                if val == 0 {
                    None
                } else {
                    Some((val - 1) as u32)
                }
            }),
            18 => EntityDataValue::Pose(Pose::read_into(buf)?),
            _ => return Err(format!("Unknown entity data type: {}", type_)),
        })
    }
}

impl McBufWritable for EntityDataValue {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        todo!();
    }
}

#[derive(Clone, Debug, Copy, McBufReadable, McBufWritable)]
pub enum Pose {
    Standing = 0,
    FallFlying = 1,
    Sleeping = 2,
    Swimming = 3,
    SpinAttack = 4,
    Sneaking = 5,
    LongJumping = 6,
    Dying = 7,
}

#[derive(Debug, Clone, McBufReadable, McBufWritable)]
pub struct VillagerData {
    #[varint]
    type_: u32,
    #[varint]
    profession: u32,
    #[varint]
    level: u32,
}

#[derive(Debug, Clone, McBufReadable, McBufWritable)]
pub struct Particle {
    #[varint]
    pub id: i32,
    pub data: ParticleData,
}

#[derive(Clone, Debug)]
pub enum ParticleData {
    AmbientEntityEffect,
    AngryVillager,
    Block(BlockParticle),
    BlockMarker(BlockParticle),
    Bubble,
    Cloud,
    Crit,
    DamageIndicator,
    DragonBreath,
    DrippingLava,
    FallingLava,
    LandingLava,
    DrippingWater,
    FallingWater,
    Dust(DustParticle),
    DustColorTransition(DustColorTransitionParticle),
    Effect,
    ElderGuardian,
    EnchantedHit,
    Enchant,
    EndRod,
    EntityEffect,
    ExplosionEmitter,
    Explosion,
    FallingDust(BlockParticle),
    Firework,
    Fishing,
    Flame,
    SoulFireFlame,
    Soul,
    Flash,
    HappyVillager,
    Composter,
    Heart,
    InstantEffect,
    Item(ItemParticle),
    Vibration(VibrationParticle),
    ItemSlime,
    ItemSnowball,
    LargeSmoke,
    Lava,
    Mycelium,
    Note,
    Poof,
    Portal,
    Rain,
    Smoke,
    Sneeze,
    Spit,
    SquidInk,
    SweepAttack,
    TotemOfUndying,
    Underwater,
    Splash,
    Witch,
    BubblePop,
    CurrentDown,
    BubbleColumnUp,
    Nautilus,
    Dolphin,
    CampfireCozySmoke,
    CampfireSignalSmoke,
    DrippingHoney,
    FallingHoney,
    LandingHoney,
    FallingNectar,
    FallingSporeBlossom,
    Ash,
    CrimsonSpore,
    WarpedSpore,
    SporeBlossomAir,
    DrippingObsidianTear,
    FallingObsidianTear,
    LandingObsidianTear,
    ReversePortal,
    WhiteAsh,
    SmallFlame,
    Snowflake,
    DrippingDripstoneLava,
    FallingDripstoneLava,
    DrippingDripstoneWater,
    FallingDripstoneWater,
    GlowSquidInk,
    Glow,
    WaxOn,
    WaxOff,
    ElectricSpark,
    Scrape,
}

#[derive(Debug, Clone, McBufReadable, McBufWritable)]
pub struct BlockParticle {
    #[varint]
    pub block_state: i32,
}
#[derive(Debug, Clone, McBufReadable, McBufWritable)]
pub struct DustParticle {
    /// Red value, 0-1
    pub red: f32,
    /// Green value, 0-1
    pub green: f32,
    /// Blue value, 0-1
    pub blue: f32,
    /// The scale, will be clamped between 0.01 and 4.
    pub scale: f32,
}

#[derive(Debug, Clone, McBufReadable, McBufWritable)]
pub struct DustColorTransitionParticle {
    /// Red value, 0-1
    pub from_red: f32,
    /// Green value, 0-1
    pub from_green: f32,
    /// Blue value, 0-1
    pub from_blue: f32,
    /// The scale, will be clamped between 0.01 and 4.
    pub scale: f32,
    /// Red value, 0-1
    pub to_red: f32,
    /// Green value, 0-1
    pub to_green: f32,
    /// Blue value, 0-1
    pub to_blue: f32,
}

#[derive(Debug, Clone, McBufReadable, McBufWritable)]
pub struct ItemParticle {
    pub item: Slot,
}

#[derive(Debug, Clone, McBufReadable, McBufWritable)]
pub struct VibrationParticle {
    pub origin: BlockPos,
    pub position_type: String,
    pub block_position: BlockPos,
    #[varint]
    pub entity_id: u32,
    #[varint]
    pub ticks: u32,
}

impl McBufReadable for ParticleData {
    fn read_into(buf: &mut impl Read) -> Result<Self, String> {
        let id = buf.read_varint()?;
        Ok(match id {
            0 => ParticleData::AmbientEntityEffect,
            1 => ParticleData::AngryVillager,
            2 => ParticleData::Block(BlockParticle::read_into(buf)?),
            3 => ParticleData::BlockMarker(BlockParticle::read_into(buf)?),
            4 => ParticleData::Bubble,
            5 => ParticleData::Cloud,
            6 => ParticleData::Crit,
            7 => ParticleData::DamageIndicator,
            8 => ParticleData::DragonBreath,
            9 => ParticleData::DrippingLava,
            10 => ParticleData::FallingLava,
            11 => ParticleData::LandingLava,
            12 => ParticleData::DrippingWater,
            13 => ParticleData::FallingWater,
            14 => ParticleData::Dust(DustParticle::read_into(buf)?),
            15 => ParticleData::DustColorTransition(DustColorTransitionParticle::read_into(buf)?),
            16 => ParticleData::Effect,
            17 => ParticleData::ElderGuardian,
            18 => ParticleData::EnchantedHit,
            19 => ParticleData::Enchant,
            20 => ParticleData::EndRod,
            21 => ParticleData::EntityEffect,
            22 => ParticleData::ExplosionEmitter,
            23 => ParticleData::Explosion,
            24 => ParticleData::FallingDust(BlockParticle::read_into(buf)?),
            25 => ParticleData::Firework,
            26 => ParticleData::Fishing,
            27 => ParticleData::Flame,
            28 => ParticleData::SoulFireFlame,
            29 => ParticleData::Soul,
            30 => ParticleData::Flash,
            31 => ParticleData::HappyVillager,
            32 => ParticleData::Composter,
            33 => ParticleData::Heart,
            34 => ParticleData::InstantEffect,
            35 => ParticleData::Item(ItemParticle::read_into(buf)?),
            36 => ParticleData::Vibration(VibrationParticle::read_into(buf)?),
            37 => ParticleData::ItemSlime,
            38 => ParticleData::ItemSnowball,
            39 => ParticleData::LargeSmoke,
            40 => ParticleData::Lava,
            41 => ParticleData::Mycelium,
            42 => ParticleData::Note,
            43 => ParticleData::Poof,
            44 => ParticleData::Portal,
            45 => ParticleData::Rain,
            46 => ParticleData::Smoke,
            47 => ParticleData::Sneeze,
            48 => ParticleData::Spit,
            49 => ParticleData::SquidInk,
            50 => ParticleData::SweepAttack,
            51 => ParticleData::TotemOfUndying,
            52 => ParticleData::Underwater,
            53 => ParticleData::Splash,
            54 => ParticleData::Witch,
            55 => ParticleData::BubblePop,
            56 => ParticleData::CurrentDown,
            57 => ParticleData::BubbleColumnUp,
            58 => ParticleData::Nautilus,
            59 => ParticleData::Dolphin,
            60 => ParticleData::CampfireCozySmoke,
            61 => ParticleData::CampfireSignalSmoke,
            62 => ParticleData::DrippingHoney,
            63 => ParticleData::FallingHoney,
            64 => ParticleData::LandingHoney,
            65 => ParticleData::FallingNectar,
            66 => ParticleData::FallingSporeBlossom,
            67 => ParticleData::Ash,
            68 => ParticleData::CrimsonSpore,
            69 => ParticleData::WarpedSpore,
            70 => ParticleData::SporeBlossomAir,
            71 => ParticleData::DrippingObsidianTear,
            72 => ParticleData::FallingObsidianTear,
            73 => ParticleData::LandingObsidianTear,
            74 => ParticleData::ReversePortal,
            75 => ParticleData::WhiteAsh,
            76 => ParticleData::SmallFlame,
            77 => ParticleData::Snowflake,
            78 => ParticleData::DrippingDripstoneLava,
            79 => ParticleData::FallingDripstoneLava,
            80 => ParticleData::DrippingDripstoneWater,
            81 => ParticleData::FallingDripstoneWater,
            82 => ParticleData::GlowSquidInk,
            83 => ParticleData::Glow,
            84 => ParticleData::WaxOn,
            85 => ParticleData::WaxOff,
            86 => ParticleData::ElectricSpark,
            87 => ParticleData::Scrape,
            _ => return Err(format!("Unknown particle id: {}", id)),
        })
    }
}

impl McBufWritable for ParticleData {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        todo!()
    }
}

use crate::{BlockPos, Slot};
use azalea_buf::{BufReadError, McBuf, McBufReadable, McBufVarReadable, McBufWritable};
use std::io::{Cursor, Write};

#[derive(Debug, Clone, McBuf)]
pub struct Particle {
    #[var]
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

#[derive(Debug, Clone, McBuf)]
pub struct BlockParticle {
    #[var]
    pub block_state: i32,
}
#[derive(Debug, Clone, McBuf)]
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

#[derive(Debug, Clone, McBuf)]
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

#[derive(Debug, Clone, McBuf)]
pub struct ItemParticle {
    pub item: Slot,
}

#[derive(Debug, Clone, McBuf)]
pub struct VibrationParticle {
    pub origin: BlockPos,
    pub position_type: String,
    pub block_position: BlockPos,
    #[var]
    pub entity_id: u32,
    #[var]
    pub ticks: u32,
}

impl ParticleData {
    pub fn read_from_particle_id(buf: &mut Cursor<Vec<u8>>, id: u32) -> Result<Self, BufReadError> {
        Ok(match id {
            0 => ParticleData::AmbientEntityEffect,
            1 => ParticleData::AngryVillager,
            2 => ParticleData::Block(BlockParticle::read_from(buf)?),
            3 => ParticleData::BlockMarker(BlockParticle::read_from(buf)?),
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
            14 => ParticleData::Dust(DustParticle::read_from(buf)?),
            15 => ParticleData::DustColorTransition(DustColorTransitionParticle::read_from(buf)?),
            16 => ParticleData::Effect,
            17 => ParticleData::ElderGuardian,
            18 => ParticleData::EnchantedHit,
            19 => ParticleData::Enchant,
            20 => ParticleData::EndRod,
            21 => ParticleData::EntityEffect,
            22 => ParticleData::ExplosionEmitter,
            23 => ParticleData::Explosion,
            24 => ParticleData::FallingDust(BlockParticle::read_from(buf)?),
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
            35 => ParticleData::Item(ItemParticle::read_from(buf)?),
            36 => ParticleData::Vibration(VibrationParticle::read_from(buf)?),
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
            _ => return Err(BufReadError::UnexpectedEnumVariant { id: id as i32 }),
        })
    }
}

impl McBufReadable for ParticleData {
    fn read_from(buf: &mut Cursor<Vec<u8>>) -> Result<Self, BufReadError> {
        let id = u32::var_read_from(buf)?;
        ParticleData::read_from_particle_id(buf, id)
    }
}

impl McBufWritable for ParticleData {
    fn write_into(&self, _buf: &mut impl Write) -> Result<(), std::io::Error> {
        todo!()
    }
}

use crate::{BlockPos, Slot};
use azalea_buf::McBuf;

#[cfg_attr(feature = "bevy_ecs", derive(bevy_ecs::component::Component))]
#[derive(Debug, Clone, McBuf, Default)]
pub struct Particle {
    #[var]
    pub id: i32,
    pub data: ParticleData,
}

#[derive(Clone, Debug, McBuf, Default)]
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
    #[default]
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

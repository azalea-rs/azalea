use azalea_block::BlockState;
use azalea_buf::AzBuf;
use azalea_codec::Codec;
use azalea_core::{color::RgbColor, entity_id::MinecraftEntityId, position::BlockPos};
use azalea_registry_builtin::builtin::BlockKind;

use crate::{builtin::ParticleKind, item::ItemStack};

// the order of this enum must be kept in sync with ParticleKind, otherwise
// we get errors parsing particles.

/// A [`ParticleKind`] with data potentially attached to it.
#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
#[codec(tag = "type", tag_as = ParticleKind)]
#[codec(default_snake_case_key)]
pub enum Particle {
    AngryVillager,
    Block(BlockParticle),
    BlockMarker(BlockParticle),
    Bubble,
    SulfurBubbles,
    NoxiousGas,
    NoxiousGasCloud,
    Geyser(GeyserParticle),
    GeyserBase(GeyserBaseParticle),
    GeyserPoof(GeyserBaseParticle),
    GeyserPlume(GeyserParticle),
    Cloud,
    CopperFireFlame,
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
    EntityEffect(ColorParticle),
    ExplosionEmitter,
    Explosion,
    Gust,
    SmallGust,
    GustEmitterLarge,
    GustEmitterSmall,
    SonicBoom,
    FallingDust(BlockParticle),
    Firework,
    Fishing,
    Flame,
    Infested,
    CherryLeaves,
    PaleOakLeaves,
    RedPoplarLeaves,
    OrangePoplarLeaves,
    YellowPoplarLeaves,
    TintedLeaves,
    SculkSoul,
    SculkCharge(SculkChargeParticle),
    SculkChargePop,
    SoulFireFlame,
    Soul,
    Flash,
    HappyVillager,
    Composter,
    Heart,
    InstantEffect,
    Item(ItemParticle),
    Vibration(Box<VibrationParticle>),
    Trail,
    PauseMobGrowth,
    ResetMobGrowth,
    ItemSlime,
    ItemCobweb,
    ItemSnowball,
    LargeSmoke,
    Lava,
    Mycelium,
    Note,
    Poof,
    Portal,
    Rain,
    Smoke,
    WhiteSmoke,
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
    CampfireCosySmoke,
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
    Shriek(ShriekParticle),
    EggCrack,
    DustPlume,
    TrialSpawnerDetection,
    TrialSpawnerDetectionOminous,
    VaultConnection,
    DustPillar,
    OminousSpawning,
    RaidOmen,
    TrialOmen,
    BlockCrumble,
    Firefly,
    SulfurCubeGoo,
}

impl Default for Particle {
    fn default() -> Self {
        Self::EntityEffect(ColorParticle::default())
    }
}

#[derive(AzBuf, Clone, Debug, Default, PartialEq, Codec)]
pub struct BlockParticle {
    #[codec(with = BlockKind)]
    pub block_state: BlockState,
}
#[derive(AzBuf, Clone, Debug, Default, PartialEq, Codec)]
pub struct DustParticle {
    pub color: RgbColor,
    /// The scale, will be clamped between 0.01 and 4.
    pub scale: f32,
}

#[derive(AzBuf, Clone, Debug, Default, PartialEq, Codec)]
pub struct DustColorTransitionParticle {
    pub from: RgbColor,
    pub to: RgbColor,
    /// The scale, will be clamped between 0.01 and 4.
    pub scale: f32,
}

#[derive(AzBuf, Clone, Debug, Default, PartialEq, Codec)]
pub struct ColorParticle {
    pub color: RgbColor,
}

#[derive(AzBuf, Clone, Debug, Default, PartialEq, Codec)]
pub struct ItemParticle {
    pub item: ItemStack,
}

#[derive(AzBuf, Clone, Debug, Default, PartialEq, Codec)]
pub struct VibrationParticle {
    pub position: PositionSource,
    #[var]
    pub ticks: u32,
}

#[derive(AzBuf, Clone, Debug, PartialEq, Codec)]
pub enum PositionSource {
    Block(BlockPos),
    #[codec(skip)]
    Entity {
        #[var]
        id: MinecraftEntityId,
        y_offset: f32,
    },
}
impl Default for PositionSource {
    fn default() -> Self {
        // bad default but hopefully it never gets used anyways
        Self::Block(BlockPos::default())
    }
}

#[derive(AzBuf, Clone, Debug, Default, PartialEq, Codec)]
pub struct SculkChargeParticle {
    pub roll: f32,
}

#[derive(AzBuf, Clone, Debug, Default, PartialEq, Codec)]
pub struct ShriekParticle {
    /// The time in ticks before the particle is displayed.
    #[var]
    pub delay: i32,
}

#[derive(AzBuf, Clone, Debug, Default, PartialEq, Codec)]
pub struct GeyserParticle {
    pub water_blocks: i32,
}

#[derive(AzBuf, Clone, Debug, Default, PartialEq, Codec)]
pub struct GeyserBaseParticle {
    pub water_blocks: i32,
    pub burst_impulse_base: f32,
}

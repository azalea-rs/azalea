#![doc = include_str!("../README.md")]

mod menu;
pub mod operations;
pub mod tier;

#[doc(hidden)]
#[deprecated = "moved to `azalea_registry::item::ItemStack`."]
pub type ItemStack = azalea_registry::item::ItemStack;
#[doc(hidden)]
#[deprecated = "moved to `azalea_registry::item::ItemStackData`."]
pub type ItemStackData = azalea_registry::item::ItemStackData;
#[doc(hidden)]
#[deprecated = "moved to `azalea_registry::item::components`."]
pub mod components {
    macro_rules! re_export_components {
        ($($c:ident)*) => {
            $(
                pub type $c = azalea_registry::item::components::$c;
            )*
        };
    }
    // pasted from docs :>
    re_export_components!(
        AdditionalTradeCost
        AdventureModePredicate
        AssetInfo
        AttackRange
        AttributeModifier
        AttributeModifiers
        AttributeModifiersEntry
        AxolotlVariant
        BannerPatternLayer
        BannerPatterns
        BaseColor
        BeehiveOccupant
        Bees
        BlockEntityData
        BlockPredicate
        BlockState
        BlockStatePropertyMatcher
        BlocksAttacks
        BreakSound
        BucketEntityData
        BundleContents
        CanBreak
        CanPlaceOn
        CatCollar
        CatSoundVariant
        CatVariant
        ChargedProjectiles
        ChickenSoundVariant
        ChickenVariant
        Consumable
        Container
        ContainerLoot
        CowSoundVariant
        CowVariant
        CreativeSlotLock
        CustomData
        CustomModelData
        CustomName
        Damage
        DamageReduction
        DamageResistant
        DamageType
        DeathProtection
        DebugStickState
        DirectTrimMaterial
        Dye
        DyedColor
        Enchantable
        EnchantmentGlintOverride
        Enchantments
        EntityData
        Equippable
        FireworkExplosion
        Fireworks
        Food
        FoxVariant
        FrogVariant
        Glider
        HorseVariant
        Instrument
        InstrumentData
        IntangibleProjectile
        ItemDamageFunction
        ItemModel
        ItemName
        JukeboxPlayable
        JukeboxSongData
        KineticWeapon
        KineticWeaponCondition
        LlamaVariant
        Lock
        LodestoneTracker
        Lore
        MapDecorations
        MapId
        MaterialAssetGroup
        MaxDamage
        MaxStackSize
        MinimumAttackCharge
        MobEffectDetails
        MobEffectInstance
        MooshroomVariant
        NoteBlockSound
        OminousBottleAmplifier
        PaintingVariant
        PaintingVariantData
        ParrotVariant
        PartialProfile
        PiercingWeapon
        PigSoundVariant
        PigVariant
        PlayerSkinPatch
        PossibleEffect
        PotDecorations
        PotionContents
        PotionDurationScale
        Profile
        ProvidesBannerPatterns
        ProvidesTrimMaterial
        RabbitVariant
        Recipes
        RepairCost
        Repairable
        ResourceTexture
        SheepColor
        ShulkerColor
        StoredEnchantments
        SuspiciousStewEffect
        SuspiciousStewEffects
        SwingAnimation
        Tool
        ToolRule
        TooltipDisplay
        TooltipStyle
        Trim
        TropicalFishBaseColor
        TropicalFishPatternColor
        Unbreakable
        UseCooldown
        UseEffects
        UseRemainder
        VillagerVariant
        Weapon
        WolfCollar
        WolfSoundVariant
        WolfVariant
        WritableBookContent
        WrittenBookContent
        ZombieNautilusVariant
        AttributeModifierDisplay
        AxolotlVariantKind
        BlockStateValueMatcher
        DyeColor
        EquipmentSlot
        EquipmentSlotGroup
        FireworkExplosionShape
        FoxVariantKind
        HorseVariantKind
        ItemUseAnimation
        LlamaVariantKind
        MapPostProcessing
        MooshroomVariantKind
        ParrotVariantKind
        PartialOrFullProfile
        PlayerModelType
        RabbitVariantKind
        Rarity
        SalmonSize
        SwingAnimationKind
        TropicalFishPattern
    );
}
#[doc(hidden)]
#[deprecated = "moved to `azalea_registry::item`."]
pub mod item {
    pub use azalea_registry::item::*;
}

pub use menu::*;

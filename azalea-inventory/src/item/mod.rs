pub mod consume_effect;

pub trait MaxStackSizeExt {
    /// Get the maximum stack size for this item.
    ///
    /// This is a signed integer to be consistent with the `count` field of
    /// [`ItemStackData`].
    ///
    /// [`ItemStackData`]: crate::ItemStackData
    fn max_stack_size(&self) -> i32;

    /// Whether this item can be stacked with other items.
    ///
    /// This is equivalent to `self.max_stack_size() > 1`.
    fn stackable(&self) -> bool {
        self.max_stack_size() > 1
    }
}

impl MaxStackSizeExt for azalea_registry::Item {
    fn max_stack_size(&self) -> i32 {
        // Best effort, might have forgotten some niche item
        let name = self.to_string();
        if [
            Self::ShulkerBox,
            Self::Potion,
            Self::SplashPotion,
            Self::LingeringPotion,
            Self::AxolotlBucket,
            Self::Minecart,
            Self::Trident,
            Self::Mace,
            Self::Bow,
            Self::Crossbow,
            Self::TotemOfUndying,
            Self::Shield,
            Self::Cake,
            Self::Shears,
            Self::Bundle,
            Self::EnchantedBook,
            Self::DebugStick,
            Self::WritableBook,
        ]
            .contains(self)
            || name.ends_with("_shulker_box")
            || name.ends_with("_bucket")
            || name.ends_with("_pattern")
            || name.ends_with("_minecart")
            || name.ends_with("_boat")
            || name.ends_with("_helmet")
            || name.ends_with("_leggings")
            || name.ends_with("_chestplate")
            || name.ends_with("_boots")
            || name.ends_with("_sword")
            || name.ends_with("_axe")
            || name.ends_with("_pickaxe")
            || name.ends_with("_shovel")
            || name.ends_with("_hoe")
            || name.ends_with("_horse_armor")
            || name.ends_with("_soup")
            || name.ends_with("_bundle")
            || name.contains("music_disc_")
            || name.ends_with("_bed")
        {
            1
        } else if [Self::Bucket, Self::EnderPearl, Self::Snowball, Self::Egg, Self::ArmorStand, Self::WrittenBook].contains(self)
            || name.ends_with("_sign")
            || name.ends_with("_banner")
        {
            16
        } else {
            64
        }
    }
}

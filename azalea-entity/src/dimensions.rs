use azalea_core::{aabb::AABB, position::Vec3};
use azalea_registry::EntityKind;

#[derive(Debug, Default, Clone)]
pub struct EntityDimensions {
    pub width: f32,
    pub height: f32,
}

impl EntityDimensions {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub fn make_bounding_box(&self, pos: &Vec3) -> AABB {
        let radius = (self.width / 2.0) as f64;
        let height = self.height as f64;
        AABB {
            min: Vec3::new(pos.x - radius, pos.y, pos.z - radius),
            max: Vec3::new(pos.x + radius, pos.y + height, pos.z + radius),
        }
    }
}

impl From<EntityKind> for EntityDimensions {
    fn from(entity: EntityKind) -> Self {
        // this match statement is automatically generated by codegen/genentities.py,
        // don't edit it manually!
        match entity {
            EntityKind::AcaciaBoat => EntityDimensions::new(1.375, 0.5625),
            EntityKind::AcaciaChestBoat => EntityDimensions::new(1.375, 0.5625),
            EntityKind::Allay => EntityDimensions::new(0.35, 0.6),
            EntityKind::AreaEffectCloud => EntityDimensions::new(6.0, 0.5),
            EntityKind::Armadillo => EntityDimensions::new(0.7, 0.65),
            EntityKind::ArmorStand => EntityDimensions::new(0.5, 1.975),
            EntityKind::Arrow => EntityDimensions::new(0.5, 0.5),
            EntityKind::Axolotl => EntityDimensions::new(0.75, 0.42),
            EntityKind::BambooChestRaft => EntityDimensions::new(1.375, 0.5625),
            EntityKind::BambooRaft => EntityDimensions::new(1.375, 0.5625),
            EntityKind::Bat => EntityDimensions::new(0.5, 0.9),
            EntityKind::Bee => EntityDimensions::new(0.7, 0.6),
            EntityKind::BirchBoat => EntityDimensions::new(1.375, 0.5625),
            EntityKind::BirchChestBoat => EntityDimensions::new(1.375, 0.5625),
            EntityKind::Blaze => EntityDimensions::new(0.6, 1.8),
            EntityKind::BlockDisplay => EntityDimensions::new(0.0, 0.0),
            EntityKind::Bogged => EntityDimensions::new(0.6, 1.99),
            EntityKind::Breeze => EntityDimensions::new(0.6, 1.77),
            EntityKind::BreezeWindCharge => EntityDimensions::new(0.3125, 0.3125),
            EntityKind::Camel => EntityDimensions::new(1.7, 2.375),
            EntityKind::Cat => EntityDimensions::new(0.6, 0.7),
            EntityKind::CaveSpider => EntityDimensions::new(0.7, 0.5),
            EntityKind::CherryBoat => EntityDimensions::new(1.375, 0.5625),
            EntityKind::CherryChestBoat => EntityDimensions::new(1.375, 0.5625),
            EntityKind::ChestMinecart => EntityDimensions::new(0.98, 0.7),
            EntityKind::Chicken => EntityDimensions::new(0.4, 0.7),
            EntityKind::Cod => EntityDimensions::new(0.5, 0.3),
            EntityKind::CommandBlockMinecart => EntityDimensions::new(0.98, 0.7),
            EntityKind::Cow => EntityDimensions::new(0.9, 1.4),
            EntityKind::Creaking => EntityDimensions::new(0.9, 2.7),
            EntityKind::Creeper => EntityDimensions::new(0.6, 1.7),
            EntityKind::DarkOakBoat => EntityDimensions::new(1.375, 0.5625),
            EntityKind::DarkOakChestBoat => EntityDimensions::new(1.375, 0.5625),
            EntityKind::Dolphin => EntityDimensions::new(0.9, 0.6),
            EntityKind::Donkey => EntityDimensions::new(1.39648, 1.5),
            EntityKind::DragonFireball => EntityDimensions::new(1.0, 1.0),
            EntityKind::Drowned => EntityDimensions::new(0.6, 1.95),
            EntityKind::Egg => EntityDimensions::new(0.25, 0.25),
            EntityKind::ElderGuardian => EntityDimensions::new(1.9975, 1.9975),
            EntityKind::EndCrystal => EntityDimensions::new(2.0, 2.0),
            EntityKind::EnderDragon => EntityDimensions::new(16.0, 8.0),
            EntityKind::EnderPearl => EntityDimensions::new(0.25, 0.25),
            EntityKind::Enderman => EntityDimensions::new(0.6, 2.9),
            EntityKind::Endermite => EntityDimensions::new(0.4, 0.3),
            EntityKind::Evoker => EntityDimensions::new(0.6, 1.95),
            EntityKind::EvokerFangs => EntityDimensions::new(0.5, 0.8),
            EntityKind::ExperienceBottle => EntityDimensions::new(0.25, 0.25),
            EntityKind::ExperienceOrb => EntityDimensions::new(0.5, 0.5),
            EntityKind::EyeOfEnder => EntityDimensions::new(0.25, 0.25),
            EntityKind::FallingBlock => EntityDimensions::new(0.98, 0.98),
            EntityKind::Fireball => EntityDimensions::new(1.0, 1.0),
            EntityKind::FireworkRocket => EntityDimensions::new(0.25, 0.25),
            EntityKind::FishingBobber => EntityDimensions::new(0.25, 0.25),
            EntityKind::Fox => EntityDimensions::new(0.6, 0.7),
            EntityKind::Frog => EntityDimensions::new(0.5, 0.5),
            EntityKind::FurnaceMinecart => EntityDimensions::new(0.98, 0.7),
            EntityKind::Ghast => EntityDimensions::new(4.0, 4.0),
            EntityKind::Giant => EntityDimensions::new(3.6, 12.0),
            EntityKind::GlowItemFrame => EntityDimensions::new(0.5, 0.5),
            EntityKind::GlowSquid => EntityDimensions::new(0.8, 0.8),
            EntityKind::Goat => EntityDimensions::new(0.9, 1.3),
            EntityKind::Guardian => EntityDimensions::new(0.85, 0.85),
            EntityKind::HappyGhast => EntityDimensions::new(4.0, 4.0),
            EntityKind::Hoglin => EntityDimensions::new(1.39648, 1.4),
            EntityKind::HopperMinecart => EntityDimensions::new(0.98, 0.7),
            EntityKind::Horse => EntityDimensions::new(1.39648, 1.6),
            EntityKind::Husk => EntityDimensions::new(0.6, 1.95),
            EntityKind::Illusioner => EntityDimensions::new(0.6, 1.95),
            EntityKind::Interaction => EntityDimensions::new(0.0, 0.0),
            EntityKind::IronGolem => EntityDimensions::new(1.4, 2.7),
            EntityKind::Item => EntityDimensions::new(0.25, 0.25),
            EntityKind::ItemDisplay => EntityDimensions::new(0.0, 0.0),
            EntityKind::ItemFrame => EntityDimensions::new(0.5, 0.5),
            EntityKind::JungleBoat => EntityDimensions::new(1.375, 0.5625),
            EntityKind::JungleChestBoat => EntityDimensions::new(1.375, 0.5625),
            EntityKind::LeashKnot => EntityDimensions::new(0.375, 0.5),
            EntityKind::LightningBolt => EntityDimensions::new(0.0, 0.0),
            EntityKind::LingeringPotion => EntityDimensions::new(0.25, 0.25),
            EntityKind::Llama => EntityDimensions::new(0.9, 1.87),
            EntityKind::LlamaSpit => EntityDimensions::new(0.25, 0.25),
            EntityKind::MagmaCube => EntityDimensions::new(0.52, 0.52),
            EntityKind::MangroveBoat => EntityDimensions::new(1.375, 0.5625),
            EntityKind::MangroveChestBoat => EntityDimensions::new(1.375, 0.5625),
            EntityKind::Marker => EntityDimensions::new(0.0, 0.0),
            EntityKind::Minecart => EntityDimensions::new(0.98, 0.7),
            EntityKind::Mooshroom => EntityDimensions::new(0.9, 1.4),
            EntityKind::Mule => EntityDimensions::new(1.39648, 1.6),
            EntityKind::OakBoat => EntityDimensions::new(1.375, 0.5625),
            EntityKind::OakChestBoat => EntityDimensions::new(1.375, 0.5625),
            EntityKind::Ocelot => EntityDimensions::new(0.6, 0.7),
            EntityKind::OminousItemSpawner => EntityDimensions::new(0.25, 0.25),
            EntityKind::Painting => EntityDimensions::new(0.5, 0.5),
            EntityKind::PaleOakBoat => EntityDimensions::new(1.375, 0.5625),
            EntityKind::PaleOakChestBoat => EntityDimensions::new(1.375, 0.5625),
            EntityKind::Panda => EntityDimensions::new(1.3, 1.25),
            EntityKind::Parrot => EntityDimensions::new(0.5, 0.9),
            EntityKind::Phantom => EntityDimensions::new(0.9, 0.5),
            EntityKind::Pig => EntityDimensions::new(0.9, 0.9),
            EntityKind::Piglin => EntityDimensions::new(0.6, 1.95),
            EntityKind::PiglinBrute => EntityDimensions::new(0.6, 1.95),
            EntityKind::Pillager => EntityDimensions::new(0.6, 1.95),
            EntityKind::Player => EntityDimensions::new(0.6, 1.8),
            EntityKind::PolarBear => EntityDimensions::new(1.4, 1.4),
            EntityKind::Pufferfish => EntityDimensions::new(0.7, 0.7),
            EntityKind::Rabbit => EntityDimensions::new(0.4, 0.5),
            EntityKind::Ravager => EntityDimensions::new(1.95, 2.2),
            EntityKind::Salmon => EntityDimensions::new(0.7, 0.4),
            EntityKind::Sheep => EntityDimensions::new(0.9, 1.3),
            EntityKind::Shulker => EntityDimensions::new(1.0, 1.0),
            EntityKind::ShulkerBullet => EntityDimensions::new(0.3125, 0.3125),
            EntityKind::Silverfish => EntityDimensions::new(0.4, 0.3),
            EntityKind::Skeleton => EntityDimensions::new(0.6, 1.99),
            EntityKind::SkeletonHorse => EntityDimensions::new(1.39648, 1.6),
            EntityKind::Slime => EntityDimensions::new(0.52, 0.52),
            EntityKind::SmallFireball => EntityDimensions::new(0.3125, 0.3125),
            EntityKind::Sniffer => EntityDimensions::new(1.9, 1.75),
            EntityKind::SnowGolem => EntityDimensions::new(0.7, 1.9),
            EntityKind::Snowball => EntityDimensions::new(0.25, 0.25),
            EntityKind::SpawnerMinecart => EntityDimensions::new(0.98, 0.7),
            EntityKind::SpectralArrow => EntityDimensions::new(0.5, 0.5),
            EntityKind::Spider => EntityDimensions::new(1.4, 0.9),
            EntityKind::SplashPotion => EntityDimensions::new(0.25, 0.25),
            EntityKind::SpruceBoat => EntityDimensions::new(1.375, 0.5625),
            EntityKind::SpruceChestBoat => EntityDimensions::new(1.375, 0.5625),
            EntityKind::Squid => EntityDimensions::new(0.8, 0.8),
            EntityKind::Stray => EntityDimensions::new(0.6, 1.99),
            EntityKind::Strider => EntityDimensions::new(0.9, 1.7),
            EntityKind::Tadpole => EntityDimensions::new(0.4, 0.3),
            EntityKind::TextDisplay => EntityDimensions::new(0.0, 0.0),
            EntityKind::Tnt => EntityDimensions::new(0.98, 0.98),
            EntityKind::TntMinecart => EntityDimensions::new(0.98, 0.7),
            EntityKind::TraderLlama => EntityDimensions::new(0.9, 1.87),
            EntityKind::Trident => EntityDimensions::new(0.5, 0.5),
            EntityKind::TropicalFish => EntityDimensions::new(0.5, 0.4),
            EntityKind::Turtle => EntityDimensions::new(1.2, 0.4),
            EntityKind::Vex => EntityDimensions::new(0.4, 0.8),
            EntityKind::Villager => EntityDimensions::new(0.6, 1.95),
            EntityKind::Vindicator => EntityDimensions::new(0.6, 1.95),
            EntityKind::WanderingTrader => EntityDimensions::new(0.6, 1.95),
            EntityKind::Warden => EntityDimensions::new(0.9, 2.9),
            EntityKind::WindCharge => EntityDimensions::new(0.3125, 0.3125),
            EntityKind::Witch => EntityDimensions::new(0.6, 1.95),
            EntityKind::Wither => EntityDimensions::new(0.9, 3.5),
            EntityKind::WitherSkeleton => EntityDimensions::new(0.7, 2.4),
            EntityKind::WitherSkull => EntityDimensions::new(0.3125, 0.3125),
            EntityKind::Wolf => EntityDimensions::new(0.6, 0.85),
            EntityKind::Zoglin => EntityDimensions::new(1.39648, 1.4),
            EntityKind::Zombie => EntityDimensions::new(0.6, 1.95),
            EntityKind::ZombieHorse => EntityDimensions::new(1.39648, 1.6),
            EntityKind::ZombieVillager => EntityDimensions::new(0.6, 1.95),
            EntityKind::ZombifiedPiglin => EntityDimensions::new(0.6, 1.95),
        }
    }
}

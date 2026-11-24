use azalea_registry::builtin::ItemKind;

pub fn get_item_tier(item: ItemKind) -> Option<Tier> {
    use ItemKind::*;
    Some(match item {
        WoodenPickaxe | WoodenShovel | WoodenAxe | WoodenHoe | WoodenSword => Tier::Wood,
        StonePickaxe | StoneShovel | StoneAxe | StoneHoe | StoneSword => Tier::Stone,
        IronPickaxe | IronShovel | IronAxe | IronHoe | IronSword => Tier::Iron,
        DiamondPickaxe | DiamondShovel | DiamondAxe | DiamondHoe | DiamondSword => Tier::Diamond,
        GoldenPickaxe | GoldenShovel | GoldenAxe | GoldenHoe | GoldenSword => Tier::Gold,
        NetheritePickaxe | NetheriteShovel | NetheriteAxe | NetheriteHoe | NetheriteSword => {
            Tier::Netherite
        }
        _ => return None,
    })
}

pub enum Tier {
    Wood,
    Stone,
    Iron,
    Diamond,
    Gold,
    Netherite,
}

impl Tier {
    pub fn level(&self) -> u8 {
        match self {
            Tier::Wood => 0,
            Tier::Stone => 1,
            Tier::Iron => 2,
            Tier::Diamond => 3,
            Tier::Gold => 0, // gold is the same tier as wood
            Tier::Netherite => 4,
        }
    }
    pub fn speed(&self) -> f32 {
        match self {
            Tier::Wood => 2.,
            Tier::Stone => 4.,
            Tier::Iron => 6.,
            Tier::Diamond => 8.,
            Tier::Gold => 12.,
            Tier::Netherite => 9.,
        }
    }
}

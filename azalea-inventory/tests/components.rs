use std::collections::HashMap;

use azalea_chat::{
    FormattedText,
    style::{Style, TextColor},
    text_component::TextComponent,
};
use azalea_core::{checksum::get_checksum, registry_holder::RegistryHolder};
use azalea_inventory::{
    ItemStack,
    components::{
        AdventureModePredicate, AttributeModifier, AttributeModifierDisplay,
        AttributeModifierOperation, AttributeModifiers, AttributeModifiersEntry, BlockPredicate,
        CanPlaceOn, ChargedProjectiles, CustomData, CustomName, Enchantments, EquipmentSlotGroup,
        JukeboxPlayable, Lore, MapColor, Rarity,
    },
};
use azalea_registry::{Attribute, Block, DataRegistry, Enchantment, Item};
use simdnbt::owned::{BaseNbt, Nbt, NbtCompound, NbtList, NbtTag};

#[test]
fn test_custom_name_checksum() {
    let c = CustomName {
        name: FormattedText::from("meow"),
    };
    assert_eq!(get_checksum(&c, &Default::default()).unwrap().0, 2222287064);
}
#[test]
fn test_custom_name_checksum_2() {
    let c = CustomName {
        name: TextComponent::new("meow")
            .with_style(
                Style::new()
                    .color(Some(TextColor::parse("red").unwrap()))
                    .underlined(true),
            )
            .into(),
    };

    println!("{:?}", c);
    assert_eq!(get_checksum(&c, &Default::default()).unwrap().0, 187682122);
}
#[test]
fn test_map_color_checksum() {
    let c = MapColor { color: 1 };
    assert_eq!(get_checksum(&c, &Default::default()).unwrap().0, 1565579036);
}
#[test]
fn test_lore_checksum() {
    let c = Lore {
        lines: vec!["first".into(), "second".into()],
    };
    assert_eq!(get_checksum(&c, &Default::default()).unwrap().0, 1545409323);
}
#[test]
fn test_rarity_checksum() {
    let c = Rarity::Rare;
    assert_eq!(get_checksum(&c, &Default::default()).unwrap().0, 2874400570);
}
#[test]
fn test_enchantments_checksum() {
    let mut registry_holder = RegistryHolder::default();
    registry_holder.append(
        "enchantment".into(),
        vec![
            ("sharpness".into(), Some(NbtCompound::default())),
            ("knockback".into(), Some(NbtCompound::default())),
        ],
    );
    let c = Enchantments {
        levels: HashMap::from_iter([(Enchantment::new_raw(0), 5), (Enchantment::new_raw(1), 1)]),
    };
    assert_eq!(get_checksum(&c, &registry_holder).unwrap().0, 3717391112);
}
#[test]
fn test_can_place_on_checksum() {
    let c = CanPlaceOn {
        predicate: AdventureModePredicate {
            predicates: vec![BlockPredicate {
                blocks: Some(vec![Block::GrassBlock].into()),
                properties: None,
                nbt: None,
            }],
        },
    };

    assert_eq!(get_checksum(&c, &Default::default()).unwrap().0, 227436005);
}

#[test]
fn test_custom_data_nbt() {
    let c = CustomData {
        nbt: Nbt::Some(BaseNbt::new(
            "",
            NbtCompound::from_values(vec![
                ("meow".into(), "mrrp".into()),
                (
                    "nya".into(),
                    NbtList::Compound(vec![
                        NbtTag::Int(1).into(),
                        NbtTag::Int(2).into(),
                        NbtCompound::new(),
                        NbtCompound::from_values(vec![("data".into(), NbtTag::Byte(1))]),
                    ])
                    .into(),
                ),
            ]),
        )),
    };
    assert_eq!(get_checksum(&c, &Default::default()).unwrap().0, 1035780974);
}
#[test]
fn test_attribute_modifiers_checksum() {
    // attribute_modifiers=[{type:"minecraft:scale",slot:"hand",id:"example:grow",
    // amount:4,operation:"add_multiplied_base"}]
    let c = AttributeModifiers {
        modifiers: vec![AttributeModifiersEntry {
            kind: Attribute::Scale,
            modifier: AttributeModifier {
                id: "example:grow".into(),
                amount: 4.0,
                operation: AttributeModifierOperation::AddMultipliedBase,
            },
            slot: EquipmentSlotGroup::Hand,
            display: AttributeModifierDisplay::Default,
        }],
    };

    println!("{}", serde_json::to_string(&c).unwrap());

    assert_eq!(get_checksum(&c, &Default::default()).unwrap().0, 2501379836);
}

#[test]
fn test_firework_explosion_checksum() {
    let c = AttributeModifiers {
        modifiers: vec![AttributeModifiersEntry {
            kind: Attribute::Scale,
            modifier: AttributeModifier {
                id: "example:grow".into(),
                amount: 4.0,
                operation: AttributeModifierOperation::AddMultipliedBase,
            },
            slot: EquipmentSlotGroup::Hand,
            display: AttributeModifierDisplay::Default,
        }],
    };

    println!("{}", serde_json::to_string(&c).unwrap());

    assert_eq!(get_checksum(&c, &Default::default()).unwrap().0, 2501379836);
}

#[test]
fn test_charged_projectile_checksum() {
    let c = ChargedProjectiles {
        items: vec![ItemStack::from(Item::MusicDiscCat)],
    };

    println!("{}", serde_json::to_string(&c).unwrap());

    assert_eq!(get_checksum(&c, &Default::default()).unwrap().0, 3435761017);
}

#[test]
fn test_charged_projectile_with_components_checksum() {
    // /give @s stick[minecraft:charged_projectiles=[{id: music_disc_cat,
    // components: {"!minecraft:jukebox_playable": {}, charged_projectiles: [{id:
    // music_disc_cat}]}}]]

    let c = ChargedProjectiles {
        items: vec![
            ItemStack::from(Item::MusicDiscCat)
                .with_component::<JukeboxPlayable>(None)
                .with_component(ChargedProjectiles {
                    items: vec![ItemStack::from(Item::MusicDiscCat)],
                }),
        ],
    };

    // println!("{}", serde_json::to_string(&c).unwrap());

    let todo = "todo";
    // assert_eq!(get_checksum(&c, &Default::default()).unwrap().0, 170375255);
}

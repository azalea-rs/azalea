use std::collections::HashMap;

use azalea_chat::{
    FormattedText,
    style::{Style, TextColor},
    text_component::TextComponent,
};
use azalea_core::{checksum::get_checksum, registry_holder::RegistryHolder};
use azalea_inventory::components::{
    BlockPredicate, CanPlaceOn, CustomName, Enchantments, Lore, MapColor, Rarity,
};
use azalea_registry::{DataRegistry, Enchantment};
use simdnbt::owned::NbtCompound;

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
                blocks: vec![Block::GrassBlock],
                properties: None,
                nbt: None,
            }],
        },
    };
    assert_eq!(get_checksum(&c, &Default::default()).unwrap().0, 227436005);
}

// TODO: implement serialize for nbt and then uncomment this
/*
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
    assert_eq!(checksum_for(&c).unwrap().0, 1035780974);
}
*/

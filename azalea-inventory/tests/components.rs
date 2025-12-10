use azalea_chat::{
    FormattedText,
    style::{Style, TextColor},
    text_component::TextComponent,
};
use azalea_core::{
    attribute_modifier_operation::AttributeModifierOperation,
    checksum::get_checksum,
    position::{BlockPos, GlobalPos},
};
use azalea_inventory::{
    ItemStack,
    components::{
        AdventureModePredicate, AttributeModifier, AttributeModifierDisplay, AttributeModifiers,
        AttributeModifiersEntry, BlockPredicate, CanPlaceOn, ChargedProjectiles, CustomData,
        CustomName, EquipmentSlotGroup, Glider, JukeboxPlayable, LodestoneTracker, Lore, MapColor,
        PotDecorations, Rarity,
    },
};
use azalea_registry::builtin::{Attribute, BlockKind, ItemKind};
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
// #[test]
// fn test_enchantments_checksum() {
//     let mut registry_holder = RegistryHolder::default();
//     registry_holder.append(
//         "enchantment".into(),
//         vec![
//             ("sharpness".into(), Some(NbtCompound::default())),
//             ("knockback".into(), Some(NbtCompound::default())),
//         ],
//     );
//     println!("registry holder: {registry_holder:?}");
//     let c = Enchantments {
//         levels: HashMap::from_iter([(Enchantment::new_raw(0), 5),
// (Enchantment::new_raw(1), 1)]),     };
//     assert_eq!(get_checksum(&c, &registry_holder).unwrap().0, 3717391112);
// }
#[test]
fn test_can_place_on_checksum() {
    let c = CanPlaceOn {
        predicate: AdventureModePredicate {
            predicates: vec![BlockPredicate {
                blocks: Some(vec![BlockKind::GrassBlock].into()),
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

    assert_eq!(get_checksum(&c, &Default::default()).unwrap().0, 2501379836);
}

#[test]
fn test_charged_projectile_checksum() {
    let c = ChargedProjectiles {
        items: vec![ItemStack::from(ItemKind::MusicDiscCat)],
    };

    assert_eq!(get_checksum(&c, &Default::default()).unwrap().0, 3435761017);
}

#[test]
fn test_charged_projectile_with_components_checksum() {
    let c = ChargedProjectiles {
        items: vec![
            ItemStack::from(ItemKind::MusicDiscCat)
                .with_component::<JukeboxPlayable>(None)
                .with_component(ChargedProjectiles {
                    items: vec![ItemStack::from(ItemKind::MusicDiscCat)],
                }),
        ],
    };

    assert_eq!(get_checksum(&c, &Default::default()).unwrap().0, 170375255);
}

#[test]
fn test_lodestone_tracker_checksum() {
    let c = LodestoneTracker {
        target: Some(GlobalPos {
            dimension: "meow".into(),
            pos: BlockPos::new(1, 2, 3),
        }),
        tracked: true,
    };

    assert_eq!(get_checksum(&c, &Default::default()).unwrap().0, 4138292505);
}

#[test]
fn test_pot_decorations_checksum() {
    let c = PotDecorations {
        items: vec![
            ItemKind::Stick,
            ItemKind::Brick,
            ItemKind::Brick,
            ItemKind::Brick,
        ],
    };

    assert_eq!(get_checksum(&c, &Default::default()).unwrap().0, 1951715383);
}

#[test]
fn test_glider_checksum() {
    let c = Glider;
    assert_eq!(get_checksum(&c, &Default::default()).unwrap().0, 3312760008);
}

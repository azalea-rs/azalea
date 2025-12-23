use azalea_client::test_utils::prelude::*;
use azalea_entity::Attributes;
use azalea_inventory::{ItemStack, components::Enchantments};
use azalea_protocol::packets::{
    ConnectionProtocol,
    config::{ClientboundFinishConfiguration, ClientboundRegistryData},
    game::ClientboundContainerSetSlot,
};
use azalea_registry::{Registry, builtin::ItemKind, data::Enchantment, identifier::Identifier};
use simdnbt::owned::{NbtCompound, NbtTag};

#[test]
fn test_enchantments() {
    let _lock = init();

    let mut s = Simulation::new(ConnectionProtocol::Configuration);
    s.receive_packet(ClientboundRegistryData {
        registry_id: Identifier::new("minecraft:dimension_type"),
        entries: vec![(
            Identifier::new("minecraft:overworld"),
            Some(NbtCompound::from_values(vec![
                ("height".into(), NbtTag::Int(384)),
                ("min_y".into(), NbtTag::Int(-64)),
            ])),
        )]
        .into_iter()
        .collect(),
    });
    // actual registry data copied from vanilla
    s.receive_packet(ClientboundRegistryData {
        registry_id: Identifier::new("minecraft:enchantment"),
        entries: vec![(
            Identifier::new("minecraft:efficiency"),
            Some(NbtCompound::from([
                (
                    "description",
                    [("translate", "enchantment.minecraft.efficiency".into())].into(),
                ),
                ("anvil_cost", 1.into()),
                (
                    "max_cost",
                    [("base", 51.into()), ("per_level_above_first", 10.into())].into(),
                ),
                (
                    "min_cost",
                    [("base", 1.into()), ("per_level_above_first", 10.into())].into(),
                ),
                (
                    "effects",
                    [(
                        "minecraft:attributes",
                        [
                            ("operation", "add_value".into()),
                            ("attribute", "minecraft:mining_efficiency".into()),
                            (
                                "amount",
                                [
                                    ("type", "minecraft:levels_squared".into()),
                                    ("added", 1.0f32.into()),
                                ]
                                .into(),
                            ),
                            ("id", "minecraft:enchantment.efficiency".into()),
                        ]
                        .into(),
                    )]
                    .into(),
                ),
                ("max_level", 5.into()),
                ("weight", 10.into()),
                ("slots", ["mainhand"].into()),
                ("supported_items", "#minecraft:enchantable/mining".into()),
            ])),
        )]
        .into_iter()
        .collect(),
    });
    s.tick();
    s.receive_packet(ClientboundFinishConfiguration);
    s.tick();
    s.receive_packet(default_login_packet());
    s.tick();

    fn efficiency(simulation: &mut Simulation) -> f64 {
        simulation.query_self::<&Attributes, _>(|c| c.mining_efficiency.calculate())
    }

    assert_eq!(efficiency(&mut s), 0.);

    s.receive_packet(ClientboundContainerSetSlot {
        container_id: 0,
        state_id: 1,
        slot: *azalea_inventory::Player::HOTBAR_SLOTS.start() as u16,
        item_stack: ItemKind::DiamondPickaxe.into(),
    });
    s.tick();

    // still 0 efficiency
    assert_eq!(efficiency(&mut s), 0.);

    s.receive_packet(ClientboundContainerSetSlot {
        container_id: 0,
        state_id: 2,
        slot: *azalea_inventory::Player::HOTBAR_SLOTS.start() as u16,
        item_stack: ItemStack::from(ItemKind::DiamondPickaxe).with_component(Enchantments {
            levels: [(Enchantment::from_u32(0).unwrap(), 1)].into(),
        }),
    });
    s.tick();

    // level 1 gives us value 2
    assert_eq!(efficiency(&mut s), 2.);

    s.receive_packet(ClientboundContainerSetSlot {
        container_id: 0,
        state_id: 1,
        slot: *azalea_inventory::Player::HOTBAR_SLOTS.start() as u16,
        item_stack: ItemKind::DiamondPickaxe.into(),
    });
    s.tick();

    // enchantment is cleared, so back to 0
    assert_eq!(efficiency(&mut s), 0.);
}

use azalea_client::test_utils::prelude::*;
use azalea_core::{
    position::{BlockPos, ChunkPos, Vec3},
    resource_location::ResourceLocation,
};
use azalea_entity::LookDirection;
use azalea_protocol::{
    common::movements::{PositionMoveRotation, RelativeMovements},
    packets::{
        ConnectionProtocol,
        config::{ClientboundFinishConfiguration, ClientboundRegistryData},
        game::{
            ClientboundBlockUpdate, ClientboundPlayerPosition, ServerboundAcceptTeleportation,
            ServerboundGamePacket,
        },
    },
};
use azalea_registry::{Block, DataRegistry, DimensionType};
use simdnbt::owned::{NbtCompound, NbtTag};

#[test]
fn test_clamp_look_direction_on_teleport() {
    init_tracing();

    let mut simulation = Simulation::new(ConnectionProtocol::Configuration);
    let sent_packets = SentPackets::new(&mut simulation);

    simulation.receive_packet(ClientboundRegistryData {
        registry_id: ResourceLocation::new("minecraft:dimension_type"),
        entries: vec![(
            ResourceLocation::new("minecraft:overworld"),
            Some(NbtCompound::from_values(vec![
                ("height".into(), NbtTag::Int(384)),
                ("min_y".into(), NbtTag::Int(-64)),
            ])),
        )]
        .into_iter()
        .collect(),
    });
    simulation.tick();
    simulation.receive_packet(ClientboundFinishConfiguration);
    simulation.tick();

    simulation.receive_packet(make_basic_login_packet(
        DimensionType::new_raw(0), // overworld
        ResourceLocation::new("minecraft:overworld"),
    ));
    simulation.tick();

    sent_packets.expect_tick_end();
    sent_packets.expect_empty();

    // receive a chunk so the player is "loaded" now
    simulation.receive_packet(make_basic_empty_chunk(ChunkPos::new(0, 0), (384 + 64) / 16));
    simulation.receive_packet(ClientboundBlockUpdate {
        pos: BlockPos::new(1, 1, 3),
        block_state: Block::Stone.into(),
    });
    simulation.receive_packet(ClientboundPlayerPosition {
        id: 1,
        change: PositionMoveRotation {
            pos: Vec3::ZERO,
            delta: Vec3::ZERO,
            look_direction: LookDirection::new(-134.99998, 0.0),
        },
        relative: RelativeMovements::all_absolute(),
    });
    simulation.tick();
    sent_packets.expect("AcceptTeleportation", |p| {
        matches!(
            p,
            ServerboundGamePacket::AcceptTeleportation(ServerboundAcceptTeleportation { id: 1 })
        )
    });
    sent_packets.expect("MovePlayerPosRot", |p| {
        let ServerboundGamePacket::MovePlayerPosRot(p) = p else {
            return false;
        };
        p.look_direction == LookDirection::new(225.00002, 0.)
    });
}

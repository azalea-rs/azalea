use azalea_chat::FormattedText;
use azalea_client::{inventory::Inventory, test_utils::prelude::*};
use azalea_core::position::ChunkPos;
use azalea_protocol::packets::{
    ConnectionProtocol,
    game::{ClientboundContainerClose, ClientboundOpenScreen, ClientboundSetChunkCacheCenter},
};
use azalea_registry::MenuKind;

#[test]
fn test_close_open_container() {
    init_tracing();

    let mut simulation = Simulation::new(ConnectionProtocol::Game);

    simulation.receive_packet(default_login_packet());
    simulation.tick();
    // receive a chunk so the player is "loaded" now
    simulation.receive_packet(ClientboundSetChunkCacheCenter { x: 1, z: 23 });
    simulation.receive_packet(make_basic_empty_chunk(
        ChunkPos::new(1, 23),
        (384 + 64) / 16,
    ));
    simulation.tick();

    // ensure no container is open
    simulation.with_component(|inventory: &Inventory| {
        assert!(inventory.container_menu.is_none());
        assert_eq!(inventory.id, 0);
    });

    // open a container
    simulation.receive_packet(ClientboundOpenScreen {
        container_id: 1,
        menu_type: MenuKind::Generic9x3,
        title: FormattedText::default(),
    });
    simulation.tick();

    simulation.with_component(|inventory: &Inventory| {
        assert!(inventory.container_menu.is_some());
        assert_eq!(inventory.id, 1);
    });

    // close and open
    simulation.receive_packet(ClientboundContainerClose { container_id: 1 });
    simulation.receive_packet(ClientboundOpenScreen {
        container_id: 2,
        menu_type: MenuKind::Generic9x3,
        title: FormattedText::default(),
    });
    simulation.tick();
    simulation.with_component(|inventory: &Inventory| {
        // ensure that the new container was opened
        assert!(inventory.container_menu.is_some());
        assert_eq!(inventory.id, 2);
    });

    // close with the wrong container id should still close
    simulation.receive_packet(ClientboundContainerClose { container_id: 123 });
    simulation.tick();
    simulation.with_component(|inventory: &Inventory| {
        assert!(inventory.container_menu.is_none());
        assert_eq!(inventory.id, 0);
    });
}

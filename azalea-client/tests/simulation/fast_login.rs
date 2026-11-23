use azalea_client::{InConfigState, test_utils::prelude::*};
use azalea_core::identifier::Identifier;
use azalea_entity::metadata::Health;
use azalea_protocol::packets::{
    ConnectionProtocol,
    config::{ClientboundFinishConfiguration, ClientboundRegistryData},
    game::ClientboundSetHealth,
};
use simdnbt::owned::{NbtCompound, NbtTag};

#[test]
fn test_fast_login() {
    let _lock = init();

    let mut simulation = Simulation::new(ConnectionProtocol::Configuration);
    assert!(simulation.has_component::<InConfigState>());
    receive_default_registries(&mut simulation);
    simulation.receive_packet(ClientboundFinishConfiguration);
    // note that there's no simulation tick here
    simulation.receive_packet(ClientboundSetHealth {
        health: 15.,
        food: 20,
        saturation: 20.,
    });
    simulation.tick();
    // we need a second tick to handle the state switch properly
    simulation.tick();
    assert_eq!(*simulation.component::<Health>(), 15.);
}

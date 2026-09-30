use azalea_client::{movement::LastSentInput, test_utils::prelude::*};
use azalea_core::{
    entity_id::MinecraftEntityId,
    position::{ChunkPos, Vec3},
};
use azalea_entity::{Jumping, LookDirection, MobEffectData, metadata::FallFlying};
use azalea_inventory::{
    ItemStack,
    components::{Damage, MaxDamage},
};
use azalea_protocol::{
    common::movements::{PositionMoveRotation, RelativeMovements},
    packets::{
        ConnectionProtocol,
        game::{
            ClientboundContainerSetSlot, ClientboundPlayerPosition, ClientboundSetChunkCacheCenter,
            ClientboundUpdateMobEffect, ServerboundGamePacket, s_player_command::Action,
        },
    },
};
use azalea_registry::builtin::{ItemKind, MobEffect};

const CHEST_SLOT: u16 = 6;
fn airborne_simulation() -> Simulation {
    let mut simulation = Simulation::new(ConnectionProtocol::Game);

    simulation.receive_packet(default_login_packet());
    simulation.tick();
    simulation.receive_packet(ClientboundSetChunkCacheCenter { x: 0, z: 0 });
    simulation.receive_packet(make_basic_empty_chunk(ChunkPos::new(0, 0), (384 + 64) / 16));
    simulation.receive_packet(ClientboundPlayerPosition {
        id: 1,
        change: PositionMoveRotation {
            pos: Vec3::new(0.5, 100., 0.5),
            delta: Vec3::ZERO,
            look_direction: LookDirection::default(),
        },
        relative: RelativeMovements::all_absolute(),
    });
    simulation.tick();
    simulation.tick();

    //
    // process_fall_flying_activation needs LastSentInput with jump=false
    // to detect a rising edge and LastSentInput only exists once an
    // input packet has been sent so tap jump once (with no elytra
    // equipped, so it does nothing) to get it into the state we want
    //

    simulation.with_component_mut::<Jumping>(|j| **j = true);
    simulation.tick();
    simulation.with_component_mut::<Jumping>(|j| **j = false);
    simulation.tick();
    simulation
}

fn equip_chest(simulation: &mut Simulation, item_stack: ItemStack) {
    simulation.receive_packet(ClientboundContainerSetSlot {
        container_id: 0,
        state_id: 1,
        slot: CHEST_SLOT,
        item_stack,
    });
    simulation.tick();
}

/// press the jump key for a tick and return whether azalea started fall
/// flying and whether a StartFallFlying packet was sent to server
fn press_jump(simulation: &mut Simulation, sent_packets: &SentPackets) -> (bool, bool) {
    simulation.with_component_mut::<Jumping>(|j| **j = true);
    simulation.tick();
    simulation.tick();

    assert!(
        simulation
            .get_component::<LastSentInput>()
            .is_some_and(|i| i.0.jump),
        "the jump input should have been sent"
    );

    let mut sent_start_fall_flying = false;
    while let Some(packet) = sent_packets.next() {
        if let ServerboundGamePacket::PlayerCommand(p) = packet
            && p.action == Action::StartFallFlying
        {
            sent_start_fall_flying = true;
        }
    }
    (
        *simulation.component::<FallFlying>(),
        sent_start_fall_flying,
    )
}

/// somthing else i was trying elytra and no levitation, both vanilla and azalea
/// start fall flying. the harness made worked
#[test]
fn test_elytra_starts_when_allowed() {
    let _lock = init();
    let mut simulation = airborne_simulation();
    let sent_packets = SentPackets::new(&mut simulation);
    equip_chest(&mut simulation, ItemKind::Elytra.into());
    sent_packets.clear();

    let (fall_flying, sent) = press_jump(&mut simulation, &sent_packets);
    assert!(fall_flying, "control: healthy elytra should start gliding");
    assert!(sent, "control: StartFallFlying should be sent");
}

/// bug i found vanilla LivingEntity.canGlide returns false while the entity has
/// the Levitation effect and the bot like starts fall flying anyway
#[test]
fn test_elytra_does_not_start_while_levitating() {
    let _lock = init();
    let mut simulation = airborne_simulation();
    let sent_packets = SentPackets::new(&mut simulation);
    equip_chest(&mut simulation, ItemKind::Elytra.into());

    let entity_id: MinecraftEntityId = simulation.minecraft_entity_id();
    simulation.receive_packet(ClientboundUpdateMobEffect {
        entity_id,
        mob_effect: MobEffect::Levitation,
        data: MobEffectData {
            amplifier: 0,
            duration: 200,
            ..Default::default()
        },
    });
    simulation.tick();
    sent_packets.clear();

    let (fall_flying, sent) = press_jump(&mut simulation, &sent_packets);
    assert!(
        !fall_flying,
        "vanilla does not start gliding while levitating"
    );
    assert!(
        !sent,
        "vanilla would not send StartFallFlying while levitating"
    );
}

/// bug that i found when testing vanilla LivingEntity.canGlideUsing requires
/// !itemStack.nextDamageWillBreak() this is a todo already
#[test]
fn test_elytra_does_not_start_when_about_to_break() {
    let _lock = init();
    let mut simulation = airborne_simulation();
    let sent_packets = SentPackets::new(&mut simulation);
    // elytra has 432 max durability, so 431 damage means the next damage would
    // break it
    let elytra = ItemStack::from(ItemKind::Elytra).with_component(Damage { amount: 431 });
    let max_damage = elytra.get_component::<MaxDamage>().map(|m| m.amount);
    assert_eq!(
        max_damage,
        Some(432),
        "test setup: elytra should be damageable"
    );
    equip_chest(&mut simulation, elytra);
    sent_packets.clear();

    let (fall_flying, sent) = press_jump(&mut simulation, &sent_packets);
    assert!(
        !fall_flying,
        "vanilla does not start gliding with an elytra that is about to break"
    );
    assert!(
        !sent,
        "vanilla would not send StartFallFlying with a broken elytra"
    );
}

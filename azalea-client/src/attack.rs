use azalea_core::GameMode;
use azalea_entity::{
    metadata::{ShiftKeyDown, Sprinting},
    Physics,
};
use azalea_protocol::packets::game::serverbound_interact_packet::{
    self, ServerboundInteractPacket,
};
use azalea_world::MinecraftEntityId;
use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;

use crate::{
    interact::SwingArmEvent,
    local_player::{LocalGameMode, SendPacketEvent},
    Client,
};

pub struct AttackPlugin;
impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AttackEvent>()
            .add_systems(Update, handle_attack_event);
    }
}

impl Client {
    pub fn attack(&mut self, entity_id: MinecraftEntityId) {
        self.ecs.lock().send_event(AttackEvent {
            entity: self.entity,
            target: entity_id,
        });
    }
}

#[derive(Default, Component)]
pub struct TicksSinceLastAttack(pub u32);

#[derive(Event)]
pub struct AttackEvent {
    pub entity: Entity,
    pub target: MinecraftEntityId,
}
pub fn handle_attack_event(
    mut events: EventReader<AttackEvent>,
    mut query: Query<(
        &LocalGameMode,
        &mut TicksSinceLastAttack,
        &mut Physics,
        &mut Sprinting,
        &mut ShiftKeyDown,
    )>,
    mut send_packet_events: EventWriter<SendPacketEvent>,
    mut swing_arm_event: EventWriter<SwingArmEvent>,
) {
    for event in events.iter() {
        let (game_mode, mut ticks_since_last_attack, mut physics, mut sprinting, sneaking) =
            query.get_mut(event.entity).unwrap();

        swing_arm_event.send(SwingArmEvent {
            entity: event.entity,
        });
        send_packet_events.send(SendPacketEvent {
            entity: event.entity,
            packet: ServerboundInteractPacket {
                entity_id: *event.target,
                action: serverbound_interact_packet::ActionType::Attack,
                using_secondary_action: **sneaking,
            }
            .get(),
        });

        // we can't attack if we're in spectator mode but it still sends the attack
        // packet
        if game_mode.current == GameMode::Spectator {
            continue;
        };

        ticks_since_last_attack.0 = 0;

        physics.delta = physics.delta.multiply(0.6, 1.0, 0.6);
        **sprinting = false;
    }
}

mod events;

use azalea_protocol::packets::config::*;
use azalea_protocol::packets::ConnectionProtocol;
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemState;
pub use events::*;
use tracing::{debug, warn};

use super::as_system;
use crate::client::InConfigState;
use crate::disconnect::DisconnectEvent;
use crate::packet::game::KeepAliveEvent;
use crate::raw_connection::RawConnection;
use crate::{declare_packet_handlers, InstanceHolder};

pub fn process_packet_events(ecs: &mut World) {
    let mut events_owned = Vec::new();
    let mut system_state: SystemState<EventReader<ReceiveConfigPacketEvent>> =
        SystemState::new(ecs);
    let mut events = system_state.get_mut(ecs);
    for ReceiveConfigPacketEvent {
        entity: player_entity,
        packet,
    } in events.read()
    {
        // we do this so `ecs` isn't borrowed for the whole loop
        events_owned.push((*player_entity, packet.clone()));
    }
    for (player_entity, packet) in events_owned {
        let mut handler = ConfigPacketHandler {
            player: player_entity,
            ecs,
        };

        declare_packet_handlers!(
            ClientboundConfigPacket,
            packet,
            handler,
            [
                cookie_request,
                custom_payload,
                disconnect,
                finish_configuration,
                keep_alive,
                ping,
                reset_chat,
                registry_data,
                resource_pack_pop,
                resource_pack_push,
                store_cookie,
                transfer,
                update_enabled_features,
                update_tags,
                select_known_packs,
                custom_report_details,
                server_links,
            ]
        );
    }
}

pub struct ConfigPacketHandler<'a> {
    pub ecs: &'a mut World,
    pub player: Entity,
}
impl ConfigPacketHandler<'_> {
    pub fn registry_data(&mut self, p: ClientboundRegistryData) {
        as_system::<Query<&mut InstanceHolder>>(self.ecs, |mut query| {
            let instance_holder = query.get_mut(self.player).unwrap();
            let mut instance = instance_holder.instance.write();

            // add the new registry data
            instance.registries.append(p.registry_id, p.entries);
        });
    }

    pub fn custom_payload(&mut self, p: ClientboundCustomPayload) {
        debug!("Got custom payload packet {p:?}");
    }

    pub fn disconnect(&mut self, p: ClientboundDisconnect) {
        warn!("Got disconnect packet {p:?}");
        as_system::<EventWriter<_>>(self.ecs, |mut events| {
            events.send(DisconnectEvent {
                entity: self.player,
                reason: Some(p.reason),
            });
        });
    }

    pub fn finish_configuration(&mut self, p: ClientboundFinishConfiguration) {
        debug!("got FinishConfiguration packet: {p:?}");

        as_system::<(Commands, Query<&mut RawConnection>)>(
            self.ecs,
            |(mut commands, mut query)| {
                let mut raw_conn = query.get_mut(self.player).unwrap();

                raw_conn
                    .write_packet(ServerboundFinishConfiguration)
                    .expect(
                        "we should be in the right state and encoding this packet shouldn't fail",
                    );
                raw_conn.set_state(ConnectionProtocol::Game);

                // these components are added now that we're going to be in the Game state
                commands
                    .entity(self.player)
                    .remove::<InConfigState>()
                    .insert(crate::JoinedClientBundle::default());
            },
        );
    }

    pub fn keep_alive(&mut self, p: ClientboundKeepAlive) {
        debug!(
            "Got keep alive packet (in configuration) {p:?} for {:?}",
            self.player
        );

        as_system::<(Query<&RawConnection>, EventWriter<_>)>(self.ecs, |(query, mut events)| {
            let raw_conn = query.get(self.player).unwrap();

            events.send(KeepAliveEvent {
                entity: self.player,
                id: p.id,
            });
            raw_conn
                .write_packet(ServerboundKeepAlive { id: p.id })
                .unwrap();
        });
    }

    pub fn ping(&mut self, p: ClientboundPing) {
        debug!("Got ping packet (in configuration) {p:?}");

        as_system::<Query<&RawConnection>>(self.ecs, |query| {
            let raw_conn = query.get(self.player).unwrap();

            raw_conn.write_packet(ServerboundPong { id: p.id }).unwrap();
        });
    }

    pub fn resource_pack_push(&mut self, p: ClientboundResourcePackPush) {
        debug!("Got resource pack push packet {p:?}");

        as_system::<Query<&RawConnection>>(self.ecs, |query| {
            let raw_conn = query.get(self.player).unwrap();

            // always accept resource pack
            raw_conn
                .write_packet(ServerboundResourcePack {
                    id: p.id,
                    action: s_resource_pack::Action::Accepted,
                })
                .unwrap();
        });
    }

    pub fn resource_pack_pop(&mut self, p: ClientboundResourcePackPop) {
        debug!("Got resource pack pop packet {p:?}");
    }

    pub fn update_enabled_features(&mut self, p: ClientboundUpdateEnabledFeatures) {
        debug!("Got update enabled features packet {p:?}");
    }

    pub fn update_tags(&mut self, _p: ClientboundUpdateTags) {
        debug!("Got update tags packet");
    }

    pub fn cookie_request(&mut self, p: ClientboundCookieRequest) {
        debug!("Got cookie request packet {p:?}");

        as_system::<Query<&RawConnection>>(self.ecs, |query| {
            let raw_conn = query.get(self.player).unwrap();

            raw_conn
                .write_packet(ServerboundCookieResponse {
                    key: p.key,
                    // cookies aren't implemented
                    payload: None,
                })
                .unwrap();
        });
    }

    pub fn reset_chat(&mut self, p: ClientboundResetChat) {
        debug!("Got reset chat packet {p:?}");
    }

    pub fn store_cookie(&mut self, p: ClientboundStoreCookie) {
        debug!("Got store cookie packet {p:?}");
    }

    pub fn transfer(&mut self, p: ClientboundTransfer) {
        debug!("Got transfer packet {p:?}");
    }

    pub fn select_known_packs(&mut self, p: ClientboundSelectKnownPacks) {
        debug!("Got select known packs packet {p:?}");

        as_system::<Query<&RawConnection>>(self.ecs, |query| {
            let raw_conn = query.get(self.player).unwrap();

            // resource pack management isn't implemented
            raw_conn
                .write_packet(ServerboundSelectKnownPacks {
                    known_packs: vec![],
                })
                .unwrap();
        });
    }

    pub fn server_links(&mut self, p: ClientboundServerLinks) {
        debug!("Got server links packet {p:?}");
    }

    pub fn custom_report_details(&mut self, p: ClientboundCustomReportDetails) {
        debug!("Got custom report details packet {p:?}");
    }
}

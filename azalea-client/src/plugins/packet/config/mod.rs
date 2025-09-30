mod events;

use std::io::Cursor;

use azalea_entity::LocalEntity;
use azalea_protocol::{
    packets::{ConnectionProtocol, config::*},
    read::{ReadPacketError, deserialize_packet},
};
use bevy_ecs::prelude::*;
pub use events::*;
use tracing::{debug, warn};

use super::{as_system, declare_packet_handlers};
use crate::{
    client::InConfigState,
    connection::RawConnection,
    disconnect::DisconnectEvent,
    local_player::InstanceHolder,
    packet::game::{KeepAliveEvent, ResourcePackEvent},
};

pub fn process_raw_packet(
    ecs: &mut World,
    player: Entity,
    raw_packet: &[u8],
) -> Result<(), Box<ReadPacketError>> {
    let packet = deserialize_packet(&mut Cursor::new(raw_packet))?;
    process_packet(ecs, player, &packet);
    Ok(())
}

pub fn process_packet(ecs: &mut World, player: Entity, packet: &ClientboundConfigPacket) {
    let mut handler = ConfigPacketHandler { player, ecs };

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
            clear_dialog,
            show_dialog,
            code_of_conduct,
        ]
    );
}

pub struct ConfigPacketHandler<'a> {
    pub ecs: &'a mut World,
    pub player: Entity,
}
impl ConfigPacketHandler<'_> {
    pub fn registry_data(&mut self, p: &ClientboundRegistryData) {
        as_system::<Query<&InstanceHolder>>(self.ecs, |mut query| {
            let instance_holder = query.get_mut(self.player).unwrap();
            let mut instance = instance_holder.instance.write();

            // add the new registry data
            instance
                .registries
                .append(p.registry_id.clone(), p.entries.clone());
        });
    }

    pub fn custom_payload(&mut self, p: &ClientboundCustomPayload) {
        debug!("Got custom payload packet {p:?}");
    }

    pub fn disconnect(&mut self, p: &ClientboundDisconnect) {
        warn!("Got disconnect packet {p:?}");
        as_system::<MessageWriter<_>>(self.ecs, |mut events| {
            events.write(DisconnectEvent {
                entity: self.player,
                reason: Some(p.reason.clone()),
            });
        });
    }

    pub fn finish_configuration(&mut self, _p: &ClientboundFinishConfiguration) {
        debug!("got FinishConfiguration packet");

        as_system::<(Commands, Query<&mut RawConnection>)>(
            self.ecs,
            |(mut commands, mut query)| {
                let mut raw_conn = query.get_mut(self.player).unwrap();
                raw_conn.state = ConnectionProtocol::Game;

                commands.trigger(SendConfigPacketEvent::new(
                    self.player,
                    ServerboundFinishConfiguration,
                ));

                // these components are added now that we're going to be in the Game state
                commands
                    .entity(self.player)
                    .remove::<InConfigState>()
                    .insert((
                        crate::JoinedClientBundle::default(),
                        // localentity should already be added, but in case the user forgot or
                        // something we also add it here
                        LocalEntity,
                    ));
            },
        );
    }

    pub fn keep_alive(&mut self, p: &ClientboundKeepAlive) {
        debug!(
            "Got keep alive packet (in configuration) {p:?} for {:?}",
            self.player
        );

        as_system::<(Commands, MessageWriter<_>)>(self.ecs, |(mut commands, mut events)| {
            events.write(KeepAliveEvent {
                entity: self.player,
                id: p.id,
            });
            commands.trigger(SendConfigPacketEvent::new(
                self.player,
                ServerboundKeepAlive { id: p.id },
            ));
        });
    }

    pub fn ping(&mut self, p: &ClientboundPing) {
        debug!("Got ping packet (in configuration) {p:?}");

        as_system::<Commands>(self.ecs, |mut commands| {
            commands.trigger(ConfigPingEvent {
                entity: self.player,
                packet: p.clone(),
            });
        });
    }

    pub fn resource_pack_push(&mut self, p: &ClientboundResourcePackPush) {
        debug!("Got resource pack push packet {p:?}");

        as_system::<MessageWriter<_>>(self.ecs, |mut events| {
            events.write(ResourcePackEvent {
                entity: self.player,
                id: p.id,
                url: p.url.to_owned(),
                hash: p.hash.to_owned(),
                required: p.required,
                prompt: p.prompt.to_owned(),
            });
        });
    }

    pub fn resource_pack_pop(&mut self, p: &ClientboundResourcePackPop) {
        debug!("Got resource pack pop packet {p:?}");
    }

    pub fn update_enabled_features(&mut self, p: &ClientboundUpdateEnabledFeatures) {
        debug!("Got update enabled features packet {p:?}");
    }

    pub fn update_tags(&mut self, _p: &ClientboundUpdateTags) {
        debug!("Got update tags packet");
    }

    pub fn cookie_request(&mut self, p: &ClientboundCookieRequest) {
        debug!("Got cookie request packet {p:?}");

        as_system::<Commands>(self.ecs, |mut commands| {
            commands.trigger(SendConfigPacketEvent::new(
                self.player,
                ServerboundCookieResponse {
                    key: p.key.clone(),
                    // cookies aren't implemented
                    payload: None,
                },
            ));
        });
    }

    pub fn reset_chat(&mut self, p: &ClientboundResetChat) {
        debug!("Got reset chat packet {p:?}");
    }

    pub fn store_cookie(&mut self, p: &ClientboundStoreCookie) {
        debug!("Got store cookie packet {p:?}");
    }

    pub fn transfer(&mut self, p: &ClientboundTransfer) {
        debug!("Got transfer packet {p:?}");
    }

    pub fn select_known_packs(&mut self, p: &ClientboundSelectKnownPacks) {
        debug!("Got select known packs packet {p:?}");

        as_system::<Commands>(self.ecs, |mut commands| {
            // resource pack management isn't implemented
            commands.trigger(SendConfigPacketEvent::new(
                self.player,
                ServerboundSelectKnownPacks {
                    known_packs: vec![],
                },
            ));
        });
    }

    pub fn server_links(&mut self, p: &ClientboundServerLinks) {
        debug!("Got server links packet {p:?}");
    }

    pub fn custom_report_details(&mut self, p: &ClientboundCustomReportDetails) {
        debug!("Got custom report details packet {p:?}");
    }

    pub fn clear_dialog(&mut self, p: &ClientboundClearDialog) {
        debug!("Got clear dialog packet {p:?}");
    }
    pub fn show_dialog(&mut self, p: &ClientboundShowDialog) {
        debug!("Got show dialog packet {p:?}");
    }
    pub fn code_of_conduct(&mut self, p: &ClientboundCodeOfConduct) {
        debug!("Got code of conduct packet {p:?}");
    }
}

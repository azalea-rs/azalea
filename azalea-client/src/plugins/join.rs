use std::{net::SocketAddr, sync::Arc};

use azalea_entity::{LocalEntity, indexing::EntityUuidIndex};
use azalea_protocol::{
    ServerAddress,
    connect::{Connection, ConnectionError, Proxy},
    packets::{
        ClientIntention, ConnectionProtocol, PROTOCOL_VERSION,
        handshake::ServerboundIntention,
        login::{ClientboundLoginPacket, ServerboundHello, ServerboundLoginPacket},
    },
};
use azalea_world::Instance;
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_tasks::{IoTaskPool, Task, futures_lite::future};
use parking_lot::RwLock;
use tokio::sync::mpsc;
use tracing::{debug, warn};

use super::events::LocalPlayerEvents;
use crate::{
    Account, JoinError, LocalPlayerBundle,
    connection::RawConnection,
    packet::login::{InLoginState, SendLoginPacketEvent},
};

/// A plugin that allows bots to join servers.
pub struct JoinPlugin;
impl Plugin for JoinPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartJoinServerEvent>().add_systems(
            Update,
            (handle_start_join_server_event, poll_create_connection_task),
        );
    }
}

#[derive(Event, Debug)]
pub struct StartJoinServerEvent {
    pub account: Account,
    pub address: ServerAddress,
    pub resolved_address: SocketAddr,
    pub proxy: Option<Proxy>,
    pub event_sender: Option<mpsc::UnboundedSender<crate::Event>>,

    pub start_join_callback_tx: Option<StartJoinCallback>,
}

// this is mpsc instead of oneshot so it can be cloned (since it's sent in an
// event)
#[derive(Component, Debug, Clone)]
pub struct StartJoinCallback(pub mpsc::UnboundedSender<Result<Entity, JoinError>>);

pub fn handle_start_join_server_event(
    mut commands: Commands,
    mut events: EventReader<StartJoinServerEvent>,
    mut entity_uuid_index: ResMut<EntityUuidIndex>,
) {
    for event in events.read() {
        let uuid = event.account.uuid_or_offline();
        let entity = if let Some(entity) = entity_uuid_index.get(&uuid) {
            debug!("Reusing entity {entity:?} for client");
            entity
        } else {
            let entity = commands.spawn_empty().id();
            debug!("Created new entity {entity:?} for client");
            // add to the uuid index
            entity_uuid_index.insert(uuid, entity);
            entity
        };

        let mut entity_mut = commands.entity(entity);
        entity_mut.insert((
            // add the Account to the entity now so plugins can access it earlier
            event.account.to_owned(),
            // localentity is always present for our clients, even if we're not actually logged
            // in
            LocalEntity,
            // we don't insert InLoginState until we actually create the connection. note that
            // there's no InHandshakeState component since we switch off of the handshake state
            // immediately when the connection is created
        ));

        if let Some(event_sender) = &event.event_sender {
            // this is optional so we don't leak memory in case the user doesn't want to
            // handle receiving packets
            entity_mut.insert(LocalPlayerEvents(event_sender.clone()));
        }
        if let Some(start_join_callback) = &event.start_join_callback_tx {
            entity_mut.insert(start_join_callback.clone());
        }

        let task_pool = IoTaskPool::get();
        let resolved_addr = event.resolved_address;
        let address = event.address.clone();
        let proxy = event.proxy.clone();
        let task = task_pool.spawn(async_compat::Compat::new(
            create_conn_and_send_intention_packet(resolved_addr, address, proxy),
        ));

        entity_mut.insert(CreateConnectionTask(task));
    }
}

async fn create_conn_and_send_intention_packet(
    resolved_addr: SocketAddr,
    address: ServerAddress,
    proxy: Option<Proxy>,
) -> Result<LoginConn, ConnectionError> {
    let mut conn = if let Some(proxy) = proxy {
        Connection::new_with_proxy(&resolved_addr, proxy).await?
    } else {
        Connection::new(&resolved_addr).await?
    };

    conn.write(ServerboundIntention {
        protocol_version: PROTOCOL_VERSION,
        hostname: address.host.clone(),
        port: address.port,
        intention: ClientIntention::Login,
    })
    .await?;

    let conn = conn.login();

    Ok(conn)
}

type LoginConn = Connection<ClientboundLoginPacket, ServerboundLoginPacket>;

#[derive(Component)]
pub struct CreateConnectionTask(pub Task<Result<LoginConn, ConnectionError>>);

pub fn poll_create_connection_task(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut CreateConnectionTask,
        &Account,
        Option<&StartJoinCallback>,
    )>,
) {
    for (entity, mut task, account, mut start_join_callback) in query.iter_mut() {
        if let Some(poll_res) = future::block_on(future::poll_once(&mut task.0)) {
            let mut entity_mut = commands.entity(entity);
            entity_mut.remove::<CreateConnectionTask>();
            let conn = match poll_res {
                Ok(conn) => conn,
                Err(err) => {
                    warn!("failed to create connection: {err}");
                    if let Some(cb) = start_join_callback.take() {
                        let _ = cb.0.send(Err(err.into()));
                    }
                    return;
                }
            };

            let (read_conn, write_conn) = conn.into_split();
            let (read_conn, write_conn) = (read_conn.raw, write_conn.raw);

            let instance = Instance::default();
            let instance_holder = crate::local_player::InstanceHolder::new(
                entity,
                // default to an empty world, it'll be set correctly later when we
                // get the login packet
                Arc::new(RwLock::new(instance)),
            );

            entity_mut.insert((
                // these stay when we switch to the game state
                LocalPlayerBundle {
                    raw_connection: RawConnection::new(
                        read_conn,
                        write_conn,
                        ConnectionProtocol::Login,
                    ),
                    client_information: crate::ClientInformation::default(),
                    instance_holder,
                    metadata: azalea_entity::metadata::PlayerMetadataBundle::default(),
                },
                InLoginState,
            ));

            commands.trigger(SendLoginPacketEvent::new(
                entity,
                ServerboundHello {
                    name: account.username.clone(),
                    profile_id: account.uuid_or_offline(),
                },
            ));

            if let Some(cb) = start_join_callback.take() {
                let _ = cb.0.send(Ok(entity));
            }
        }
    }
}

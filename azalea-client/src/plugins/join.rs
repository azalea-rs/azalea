use std::{net::SocketAddr, sync::Arc};

use azalea_entity::{LocalEntity, indexing::EntityUuidIndex};
use azalea_protocol::{
    ServerAddress,
    common::client_information::ClientInformation,
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
    Account, LocalPlayerBundle,
    connection::RawConnection,
    packet::login::{InLoginState, SendLoginPacketEvent},
};

/// A plugin that allows bots to join servers.
pub struct JoinPlugin;
impl Plugin for JoinPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<StartJoinServerEvent>()
            .add_message::<ConnectionFailedEvent>()
            .add_systems(
                Update,
                (
                    handle_start_join_server_event.before(super::login::poll_auth_task),
                    poll_create_connection_task,
                )
                    .chain(),
            );
    }
}

/// An event to make a client join the server and be added to our swarm.
///
/// This won't do anything if a client with the Account UUID is already
/// connected to the server.
#[derive(Message, Debug)]
pub struct StartJoinServerEvent {
    pub account: Account,
    pub connect_opts: ConnectOpts,
    pub event_sender: Option<mpsc::UnboundedSender<crate::Event>>,

    // this is mpsc instead of oneshot so it can be cloned (since it's sent in an event)
    pub start_join_callback_tx: Option<mpsc::UnboundedSender<Entity>>,
}

/// Options for how the connection to the server will be made. These are
/// persisted on reconnects.
///
/// This is inserted as a component on clients to make auto-reconnecting work.
#[derive(Debug, Clone, Component)]
pub struct ConnectOpts {
    pub address: ServerAddress,
    pub resolved_address: SocketAddr,
    pub proxy: Option<Proxy>,
}

/// An event that's sent when creating the TCP connection and sending the first
/// packet fails.
///
/// This isn't sent if we're kicked later, see [`DisconnectEvent`].
///
/// [`DisconnectEvent`]: crate::disconnect::DisconnectEvent
#[derive(Message)]
pub struct ConnectionFailedEvent {
    pub entity: Entity,
    pub error: ConnectionError,
}

pub fn handle_start_join_server_event(
    mut commands: Commands,
    mut events: MessageReader<StartJoinServerEvent>,
    mut entity_uuid_index: ResMut<EntityUuidIndex>,
    connection_query: Query<&RawConnection>,
) {
    for event in events.read() {
        let uuid = event.account.uuid_or_offline();
        let entity = if let Some(entity) = entity_uuid_index.get(&uuid) {
            debug!("Reusing entity {entity:?} for client");

            // check if it's already connected
            if let Ok(conn) = connection_query.get(entity)
                && conn.is_alive()
            {
                if let Some(start_join_callback_tx) = &event.start_join_callback_tx {
                    warn!(
                        "Received StartJoinServerEvent for {entity:?} but it's already connected. Ignoring the event but replying with Ok."
                    );
                    let _ = start_join_callback_tx.send(entity);
                } else {
                    warn!(
                        "Received StartJoinServerEvent for {entity:?} but it's already connected. Ignoring the event."
                    );
                }
                return;
            }

            entity
        } else {
            let entity = commands.spawn_empty().id();
            debug!("Created new entity {entity:?} for client");
            // add to the uuid index
            entity_uuid_index.insert(uuid, entity);
            entity
        };

        if let Some(start_join_callback) = &event.start_join_callback_tx {
            let _ = start_join_callback.send(entity);
        }

        let mut entity_mut = commands.entity(entity);

        entity_mut.insert((
            // add the Account to the entity now so plugins can access it earlier
            event.account.to_owned(),
            // localentity is always present for our clients, even if we're not actually logged
            // in
            LocalEntity,
            // this is inserted early so the user can always access and modify it
            ClientInformation::default(),
            // ConnectOpts is inserted as a component here
            event.connect_opts.clone(),
            // we don't insert InLoginState until we actually create the connection. note that
            // there's no InHandshakeState component since we switch off of the handshake state
            // immediately when the connection is created
        ));

        if let Some(event_sender) = &event.event_sender {
            // this is optional so we don't leak memory in case the user doesn't want to
            // handle receiving packets
            entity_mut.insert(LocalPlayerEvents(event_sender.clone()));
        }

        let task_pool = IoTaskPool::get();
        let connect_opts = event.connect_opts.clone();
        let task = task_pool.spawn(async_compat::Compat::new(
            create_conn_and_send_intention_packet(connect_opts),
        ));

        entity_mut.insert(CreateConnectionTask(task));
    }
}

async fn create_conn_and_send_intention_packet(
    opts: ConnectOpts,
) -> Result<LoginConn, ConnectionError> {
    let mut conn = if let Some(proxy) = opts.proxy {
        Connection::new_with_proxy(&opts.resolved_address, proxy).await?
    } else {
        Connection::new(&opts.resolved_address).await?
    };

    conn.write(ServerboundIntention {
        protocol_version: PROTOCOL_VERSION,
        hostname: opts.address.host.clone(),
        port: opts.address.port,
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
    mut query: Query<(Entity, &mut CreateConnectionTask, &Account)>,
    mut connection_failed_events: MessageWriter<ConnectionFailedEvent>,
) {
    for (entity, mut task, account) in query.iter_mut() {
        if let Some(poll_res) = future::block_on(future::poll_once(&mut task.0)) {
            let mut entity_mut = commands.entity(entity);
            entity_mut.remove::<CreateConnectionTask>();
            let conn = match poll_res {
                Ok(conn) => conn,
                Err(error) => {
                    warn!("failed to create connection: {error}");
                    connection_failed_events.write(ConnectionFailedEvent { entity, error });
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
        }
    }
}

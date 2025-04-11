use std::{fmt::Debug, io::Cursor, mem, sync::Arc};

use azalea_crypto::Aes128CfbEnc;
use azalea_protocol::{
    connect::{RawReadConnection, RawWriteConnection},
    packets::{
        ConnectionProtocol, Packet, ProtocolPacket, config::ClientboundConfigPacket,
        game::ClientboundGamePacket, login::ClientboundLoginPacket,
    },
    read::{ReadPacketError, deserialize_packet},
    write::serialize_packet,
};
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_tasks::{IoTaskPool, futures_lite::future};
use thiserror::Error;
use tokio::{
    io::AsyncWriteExt,
    net::tcp::OwnedWriteHalf,
    sync::mpsc::{self},
};
use tracing::{debug, error};

use super::packet::{
    config::ReceiveConfigPacketEvent, game::ReceiveGamePacketEvent, login::ReceiveLoginPacketEvent,
};
use crate::packet::{config, game, login};

pub struct ConnectionPlugin;
impl Plugin for ConnectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, read_packets);
    }
}

pub fn read_packets(ecs: &mut World) {
    // receive_game_packet_events: EventWriter<ReceiveGamePacketEvent>,
    let mut query = ecs.query::<(Entity, &mut RawConnection)>();

    let mut entities_handling_packets = Vec::new();
    let mut entities_with_injected_packets = Vec::new();
    for (entity, mut raw_conn) in query.iter_mut(ecs) {
        let state = raw_conn.state;

        if !raw_conn.injected_clientbound_packets.is_empty() {
            entities_with_injected_packets.push((
                entity,
                state,
                mem::take(&mut raw_conn.injected_clientbound_packets),
            ));
        }

        let Some(net_conn) = raw_conn.network.take() else {
            // means it's a networkless connection
            continue;
        };
        entities_handling_packets.push((entity, state, net_conn));
    }

    let mut queued_packet_events = QueuedPacketEvents::default();

    // handle injected packets, see the comment on
    // RawConnection::injected_clientbound_packets for more info
    for (entity, mut state, raw_packets) in entities_with_injected_packets {
        for raw_packet in raw_packets {
            handle_raw_packet(
                ecs,
                &raw_packet,
                entity,
                &mut state,
                None,
                &mut queued_packet_events,
            )
            .unwrap();

            // update the state and for the client
            let (_, mut raw_conn_component) = query.get_mut(ecs, entity).unwrap();
            raw_conn_component.state = state;
        }
    }

    // we pass the mutable state and net_conn into the handlers so they're allowed
    // to mutate it
    for (entity, mut state, mut net_conn) in entities_handling_packets {
        loop {
            match net_conn.reader.try_read() {
                Ok(Some(raw_packet)) => {
                    let raw_packet = Arc::<[u8]>::from(raw_packet);
                    if let Err(e) = handle_raw_packet(
                        ecs,
                        &raw_packet,
                        entity,
                        &mut state,
                        Some(&mut net_conn),
                        &mut queued_packet_events,
                    ) {
                        error!("Error reading packet: {e}");
                    }
                }
                Ok(None) => {
                    // no packets available
                    break;
                }
                Err(err) => {
                    log_for_error(&err);
                    break;
                }
            }
        }

        // this needs to be done at some point every update, so we do it here right
        // after the handlers are called
        net_conn.poll_writer();

        // update the state and network connections for the client
        let (_, mut raw_conn_component) = query.get_mut(ecs, entity).unwrap();
        raw_conn_component.state = state;
        raw_conn_component.network = Some(net_conn);
    }

    queued_packet_events.send_events(ecs);
}

#[derive(Default)]
pub struct QueuedPacketEvents {
    login: Vec<ReceiveLoginPacketEvent>,
    config: Vec<ReceiveConfigPacketEvent>,
    game: Vec<ReceiveGamePacketEvent>,
}
impl QueuedPacketEvents {
    fn send_events(&mut self, ecs: &mut World) {
        ecs.send_event_batch(self.login.drain(..));
        ecs.send_event_batch(self.config.drain(..));
        ecs.send_event_batch(self.game.drain(..));
    }
}

fn log_for_error(error: &ReadPacketError) {
    if !matches!(*error, ReadPacketError::ConnectionClosed) {
        error!("Error reading packet from Client: {error:?}");
    }
}

/// The client's connection to the server.
#[derive(Component)]
pub struct RawConnection {
    /// The network connection to the server.
    ///
    /// This isn't guaranteed to be present, for example during the main packet
    /// handlers or at all times during tests.
    ///
    /// You shouldn't rely on this. Instead, use the events for sending packets
    /// like [`SendPacketEvent`](crate::packet::game::SendPacketEvent) /
    /// [`SendConfigPacketEvent`](crate::packet::config::SendConfigPacketEvent)
    /// / [`SendLoginPacketEvent`](crate::packet::login::SendLoginPacketEvent).
    ///
    /// To check if we haven't disconnected from the server, use
    /// [`Self::is_alive`].
    network: Option<NetworkConnection>,
    pub state: ConnectionProtocol,
    is_alive: bool,

    /// This exists for internal testing purposes and probably shouldn't be used
    /// for normal bots. It's basically a way to make our client think it
    /// received a packet from the server without needing to interact with the
    /// network.
    pub injected_clientbound_packets: Vec<Box<[u8]>>,
}
impl RawConnection {
    pub fn new(
        reader: RawReadConnection,
        writer: RawWriteConnection,
        state: ConnectionProtocol,
    ) -> Self {
        let task_pool = IoTaskPool::get();

        let (network_packet_writer_tx, network_packet_writer_rx) =
            mpsc::unbounded_channel::<Box<[u8]>>();

        let writer_task =
            task_pool.spawn(write_task(network_packet_writer_rx, writer.write_stream));

        let mut conn = Self::new_networkless(state);
        conn.network = Some(NetworkConnection {
            reader,
            enc_cipher: writer.enc_cipher,
            network_packet_writer_tx,
            writer_task,
        });

        conn
    }

    pub fn new_networkless(state: ConnectionProtocol) -> Self {
        Self {
            network: None,
            state,
            is_alive: true,
            injected_clientbound_packets: Vec::new(),
        }
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }

    /// Write a packet to the server without emitting any events.
    ///
    /// This is called by the handlers for [`SendPacketEvent`],
    /// [`SendConfigPacketEvent`], and [`SendLoginPacketEvent`].
    ///
    /// [`SendPacketEvent`]: crate::packet::game::SendPacketEvent
    /// [`SendConfigPacketEvent`]: crate::packet::config::SendConfigPacketEvent
    /// [`SendLoginPacketEvent`]: crate::packet::login::SendLoginPacketEvent
    pub fn write<P: ProtocolPacket + Debug>(
        &mut self,
        packet: impl Packet<P>,
    ) -> Result<(), WritePacketError> {
        if let Some(network) = &mut self.network {
            let packet = packet.into_variant();
            let raw_packet = serialize_packet(&packet)?;
            network.write_raw(&raw_packet)?;
        }
        Ok(())
    }

    pub fn net_conn(&mut self) -> Option<&mut NetworkConnection> {
        self.network.as_mut()
    }
}

pub fn handle_raw_packet(
    ecs: &mut World,
    raw_packet: &[u8],
    entity: Entity,
    state: &mut ConnectionProtocol,
    net_conn: Option<&mut NetworkConnection>,
    queued_packet_events: &mut QueuedPacketEvents,
) -> Result<(), Box<ReadPacketError>> {
    let stream = &mut Cursor::new(raw_packet);
    match state {
        ConnectionProtocol::Handshake => {
            unreachable!()
        }
        ConnectionProtocol::Game => {
            let packet = Arc::new(deserialize_packet::<ClientboundGamePacket>(stream)?);
            game::process_packet(ecs, entity, packet.as_ref());
            queued_packet_events
                .game
                .push(ReceiveGamePacketEvent { entity, packet });
        }
        ConnectionProtocol::Status => {
            unreachable!()
        }
        ConnectionProtocol::Login => {
            let packet = Arc::new(deserialize_packet::<ClientboundLoginPacket>(stream)?);
            login::process_packet(ecs, entity, &packet, state, net_conn);
            queued_packet_events
                .login
                .push(ReceiveLoginPacketEvent { entity, packet });
        }
        ConnectionProtocol::Configuration => {
            let packet = Arc::new(deserialize_packet::<ClientboundConfigPacket>(stream)?);
            config::process_packet(ecs, entity, &packet);
            queued_packet_events
                .config
                .push(ReceiveConfigPacketEvent { entity, packet });
        }
    };

    Ok(())
}

pub struct NetworkConnection {
    reader: RawReadConnection,
    // compression threshold is in the RawReadConnection
    pub enc_cipher: Option<Aes128CfbEnc>,

    pub writer_task: bevy_tasks::Task<()>,
    /// A queue of raw TCP packets to send. These will not be modified further,
    /// they should already be serialized and encrypted and everything before
    /// being added here.
    network_packet_writer_tx: mpsc::UnboundedSender<Box<[u8]>>,
}
impl NetworkConnection {
    pub fn write_raw(&mut self, raw_packet: &[u8]) -> Result<(), WritePacketError> {
        let network_packet = azalea_protocol::write::encode_to_network_packet(
            raw_packet,
            self.reader.compression_threshold,
            &mut self.enc_cipher,
        );
        self.network_packet_writer_tx
            .send(network_packet.into_boxed_slice())?;
        Ok(())
    }

    pub fn poll_writer(&mut self) {
        future::block_on(future::poll_once(&mut self.writer_task));
    }

    pub fn set_compression_threshold(&mut self, threshold: Option<u32>) {
        self.reader.compression_threshold = threshold;
    }
    /// Set the encryption key that is used to encrypt and decrypt packets. It's
    /// the same for both reading and writing.
    pub fn set_encryption_key(&mut self, key: [u8; 16]) {
        let (enc_cipher, dec_cipher) = azalea_crypto::create_cipher(&key);
        self.reader.dec_cipher = Some(dec_cipher);
        self.enc_cipher = Some(enc_cipher);
    }
}

async fn write_task(
    mut network_packet_writer_rx: mpsc::UnboundedReceiver<Box<[u8]>>,
    mut write_half: OwnedWriteHalf,
) {
    while let Some(network_packet) = network_packet_writer_rx.recv().await {
        if let Err(e) = write_half.write_all(&network_packet).await {
            debug!("Error writing packet to server: {e}");
            break;
        };
    }
}

#[derive(Error, Debug)]
pub enum WritePacketError {
    #[error("Wrong protocol state: expected {expected:?}, got {got:?}")]
    WrongState {
        expected: ConnectionProtocol,
        got: ConnectionProtocol,
    },
    #[error(transparent)]
    Encoding(#[from] azalea_protocol::write::PacketEncodeError),
    #[error(transparent)]
    SendError {
        #[from]
        #[backtrace]
        source: mpsc::error::SendError<Box<[u8]>>,
    },
}

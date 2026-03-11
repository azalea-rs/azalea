use std::{
    fmt::Debug,
    io::Cursor,
    mem,
    sync::{
        Arc,
        atomic::{self, AtomicBool},
    },
    time::Instant,
};

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
use rayon::prelude::*;
use thiserror::Error;
use tokio::{
    io::AsyncWriteExt,
    net::tcp::OwnedWriteHalf,
    sync::mpsc::{self},
};
use tracing::{debug, error, info, trace, warn};

use super::packet::{
    config::ReceiveConfigPacketEvent, game::ReceiveGamePacketEvent, login::ReceiveLoginPacketEvent,
};
use crate::packet::{config, game, login};

/// Configuration for packet processing budget.
/// Controls how many packets to process per read_packets cycle to prevent
/// CPU starvation while ensuring timely responses to KeepAlive and other
/// critical packets.
#[derive(Resource, Clone, Copy, Debug)]
pub struct PacketProcessingBudget {
    /// Maximum time in milliseconds to spend processing packets per PreUpdate
    /// cycle
    pub max_ms_per_cycle: f64,
    /// Maximum number of packets to process per entity per cycle
    pub max_packets_per_bot: u32,
}

impl Default for PacketProcessingBudget {
    fn default() -> Self {
        Self {
            max_ms_per_cycle: 20.0,
            max_packets_per_bot: 50,
        }
    }
}

/// State for rotating through entities to ensure fairness.
/// Each frame, entities are rotated so different bots get priority.
#[derive(Resource, Default, Debug)]
pub struct PacketProcessingState {
    /// Index to rotate entities slice by (incremented each frame)
    pub next_start_index: usize,
}

pub struct ConnectionPlugin;
impl Plugin for ConnectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PacketProcessingBudget>()
            .init_resource::<PacketProcessingState>()
            .add_systems(PreUpdate, (read_packets, poll_all_writer_tasks).chain());
    }
}

pub fn read_packets(ecs: &mut World) {
    let start_time = Instant::now();
    let mut entity_and_conn_query = ecs.query::<(Entity, &mut RawConnection)>();
    let mut conn_query = ecs.query::<&mut RawConnection>();

    let mut entities_handling_packets = Vec::new();
    let mut total_packets: u32 = 0;
    let mut entities_with_injected_packets = Vec::new();
    for (entity, mut raw_conn) in entity_and_conn_query.iter_mut(ecs) {
        if !raw_conn.injected_clientbound_packets.is_empty() {
            entities_with_injected_packets.push((
                entity,
                mem::take(&mut raw_conn.injected_clientbound_packets),
            ));
        }

        if raw_conn.network.is_none() {
            // no network connection, don't bother with the normal packet handling
            continue;
        }

        entities_handling_packets.push(entity);
    }

    let mut queued_packet_events = QueuedPacketEvents::default();

    // handle injected packets, see the comment on
    // RawConnection::injected_clientbound_packets for more info
    for (entity, raw_packets) in entities_with_injected_packets {
        for raw_packet in raw_packets {
            let conn = conn_query.get(ecs, entity).unwrap();
            let state = conn.state;

            trace!("Received injected packet with bytes: {raw_packet:?}");
            if let Err(e) =
                handle_raw_packet(ecs, &raw_packet, entity, state, &mut queued_packet_events)
            {
                error!("Error reading injected packet: {e}");
            }
        }
    }

    // Read and process packets from all entities.
    // Login/Config packets are processed inline (sequential) to handle protocol
    // state transitions correctly (Login→Config→Game).
    // Game packets are batched for parallel deserialization via rayon.
    let mut game_raw_packets: Vec<(Entity, Box<[u8]>)> = Vec::new();
    let mut disconnected_entities: Vec<Entity> = Vec::new();

    // Get packet budget and processing state
    let budget = ecs
        .get_resource::<PacketProcessingBudget>()
        .copied()
        .unwrap_or_default();
    let mut proc_state = ecs
        .remove_resource::<PacketProcessingState>()
        .unwrap_or_default();

    // Rotate entity processing order for fairness across frames
    if !entities_handling_packets.is_empty() {
        let start = proc_state.next_start_index % entities_handling_packets.len();
        let mut rotated = entities_handling_packets.clone();
        rotated.rotate_left(start);
        proc_state.next_start_index = proc_state.next_start_index.wrapping_add(1);
        entities_handling_packets = rotated;
    }

    // Read all packets from all entities without budget limits.
    // Budget enforcement happens during processing phase.
    let read_start = Instant::now();
    for entity in &entities_handling_packets {
        loop {
            let mut conn = conn_query.get_mut(ecs, *entity).unwrap();
            let net_conn = conn.net_conn().unwrap();
            let read_res = net_conn.reader.try_read();
            let state = conn.state;
            match read_res {
                Ok(Some(raw_packet)) => {
                    total_packets += 1;
                    match state {
                        // Login/Config: deserialize + process immediately so state
                        // transitions take effect before reading the next packet.
                        ConnectionProtocol::Login | ConnectionProtocol::Configuration => {
                            if let Err(e) = handle_raw_packet(
                                ecs,
                                &raw_packet,
                                *entity,
                                state,
                                &mut queued_packet_events,
                            ) {
                                error!("Error reading packet: {e}");
                            }
                        }
                        // Game: collect for rayon parallel deserialization
                        ConnectionProtocol::Game => {
                            game_raw_packets.push((*entity, raw_packet));
                        }
                        ConnectionProtocol::Handshake | ConnectionProtocol::Status => {
                            unreachable!()
                        }
                    }
                }
                Ok(None) => {
                    break;
                }
                Err(err) => {
                    log_for_error(&err);

                    if matches!(
                        &*err,
                        ReadPacketError::IoError { .. } | ReadPacketError::ConnectionClosed
                    ) {
                        info!("Server closed connection");
                        disconnected_entities.push(*entity);
                    }

                    break;
                }
            }
        }
    }

    // Restore state for next cycle
    ecs.insert_resource(proc_state);
    let read_elapsed = read_start.elapsed();

    // Mark disconnected entities
    for entity in disconnected_entities {
        let mut conn = conn_query.get_mut(ecs, entity).unwrap();
        conn.network = None;
        conn.is_alive = false;
    }

    // Parallel deserialization of Game packets (rayon, no ECS access)
    let deser_start = Instant::now();
    let deserialized: Vec<(
        Entity,
        Result<Arc<ClientboundGamePacket>, Box<ReadPacketError>>,
    )> = game_raw_packets
        .par_iter()
        .map(|(entity, raw_packet)| {
            let stream = &mut Cursor::new(raw_packet.as_ref());
            let result = deserialize_packet::<ClientboundGamePacket>(stream).map(|p| Arc::new(p));
            (*entity, result)
        })
        .collect();
    let deser_elapsed = deser_start.elapsed();

    // Process deserialized Game packets (sequential, needs &mut World).
    // Enforce time budget during processing: if exhausted, skip non-essential
    // packets.
    let processing_start = Instant::now();
    let time_budget = std::time::Duration::from_secs_f64(budget.max_ms_per_cycle / 1000.0);

    // Check if accumulated read + deserialization time exceeds (50ms - time_budget)
    let total_tick_budget = std::time::Duration::from_millis(50);
    let accumulated_time = start_time.elapsed();
    let budget_exhausted = accumulated_time > (total_tick_budget - time_budget);

    let mut skipped_packets = 0;
    let mut proc_slow_packets = 0u32;
    for (entity, result) in deserialized {
        match result {
            Ok(packet) => {
                // If budget exhausted, skip non-essential packets to defer processing
                if budget_exhausted && !is_essential_game_packet(&packet) {
                    skipped_packets += 1;
                    continue;
                }

                let proc_packet_start = Instant::now();
                trace!("Packet: {packet:?}");
                game::process_packet(ecs, entity, packet.as_ref());
                let proc_packet_elapsed = proc_packet_start.elapsed();
                if proc_packet_elapsed.as_millis() > 1 {
                    trace!(
                        packet_type = "ClientboundGamePacket",
                        duration_ms = proc_packet_elapsed.as_millis(),
                        entity = ?entity,
                        "Slow packet processing"
                    );
                    proc_slow_packets += 1;
                }

                queued_packet_events
                    .game
                    .push(ReceiveGamePacketEvent { entity, packet });
            }
            Err(e) => {
                error!("Error reading packet: {e}");
            }
        }
    }
    let proc_elapsed = processing_start.elapsed();

    let total_processed = game_raw_packets.len() - skipped_packets as usize;
    let elapsed = start_time.elapsed();

    if budget_exhausted {
        debug!(
            elapsed_ms = elapsed.as_millis(),
            total_packets,
            budget_ms = budget.max_ms_per_cycle,
            "Packet processing time budget exhausted; deferred remaining packets"
        );
    }

    if elapsed > total_tick_budget {
        warn!(
            duration_ms = elapsed.as_millis(),
            total_packets, "read_packets exceeded 50ms budget (one tick)"
        );
    }

    let write_start = Instant::now();
    queued_packet_events.write_messages(ecs);
    let write_elapsed = write_start.elapsed();

    debug!(
        total_duration_ms = elapsed.as_millis(),
        read_duration_ms = read_elapsed.as_millis(),
        deser_duration_ms = deser_elapsed.as_millis(),
        proc_duration_ms = proc_elapsed.as_millis(),
        write_duration_ms = write_elapsed.as_millis(),
        total_packets,
        processed_packets = total_processed,
        skipped_packets,
        entities_with_packets = entities_handling_packets.len(),
        proc_slow_packets,
        budget_exhausted,
        login_events = queued_packet_events.login.len(),
        config_events = queued_packet_events.config.len(),
        game_events = queued_packet_events.game.len(),
        "read_packets cycle complete"
    );
}

/// Returns true for packets that bots must process to stay connected and
/// navigate. Entity tracking, sounds, particles, UI, and other cosmetic
/// packets are skipped to keep the tick loop fast.
fn is_essential_game_packet(packet: &ClientboundGamePacket) -> bool {
    matches!(
        packet,
        // Connection & lifecycle
        ClientboundGamePacket::KeepAlive(_)
        | ClientboundGamePacket::Ping(_)
        | ClientboundGamePacket::Disconnect(_)
        | ClientboundGamePacket::Login(_)
        | ClientboundGamePacket::Respawn(_)
        | ClientboundGamePacket::StartConfiguration(_)
        // Position & movement
        | ClientboundGamePacket::PlayerPosition(_)
        | ClientboundGamePacket::PlayerRotation(_)
        | ClientboundGamePacket::PlayerAbilities(_)
        | ClientboundGamePacket::SetDefaultSpawnPosition(_)
        // World data (chunks, blocks — needed for pathfinding)
        | ClientboundGamePacket::LevelChunkWithLight(_)
        | ClientboundGamePacket::ForgetLevelChunk(_)
        | ClientboundGamePacket::ChunkBatchStart(_)
        | ClientboundGamePacket::ChunkBatchFinished(_)
        | ClientboundGamePacket::SetChunkCacheCenter(_)
        | ClientboundGamePacket::SetChunkCacheRadius(_)
        | ClientboundGamePacket::BlockUpdate(_)
        | ClientboundGamePacket::SectionBlocksUpdate(_)
        | ClientboundGamePacket::LightUpdate(_)
        // Game state
        | ClientboundGamePacket::GameEvent(_)
        | ClientboundGamePacket::SetHealth(_)
        | ClientboundGamePacket::SetExperience(_)
        | ClientboundGamePacket::PlayerCombatKill(_)
        | ClientboundGamePacket::ChangeDifficulty(_)
        | ClientboundGamePacket::SetTime(_)
        // Initialization
        | ClientboundGamePacket::Commands(_)
        | ClientboundGamePacket::UpdateRecipes(_)
        | ClientboundGamePacket::UpdateTags(_)
        | ClientboundGamePacket::CustomPayload(_)
        // Chat (for bot commands)
        | ClientboundGamePacket::SystemChat(_)
        // Border
        | ClientboundGamePacket::InitializeBorder(_)
        // Tick control
        | ClientboundGamePacket::TickingState(_)
        | ClientboundGamePacket::TickingStep(_)
        // Bundle delimiter (framing)
        | ClientboundGamePacket::BundleDelimiter(_)
    )
}

fn poll_all_writer_tasks(mut conn_query: Query<&mut RawConnection>) {
    for mut conn in conn_query.iter_mut() {
        if let Some(net_conn) = &mut conn.network {
            // this needs to be done at some point every update to make sure packets are
            // actually sent to the network

            if net_conn.poll_writer().is_some() {
                // means the writer task ended
                conn.network = None;
                conn.is_alive = false;
            }
        }
    }
}

#[derive(Default)]
pub struct QueuedPacketEvents {
    login: Vec<ReceiveLoginPacketEvent>,
    config: Vec<ReceiveConfigPacketEvent>,
    game: Vec<ReceiveGamePacketEvent>,
}
impl QueuedPacketEvents {
    fn write_messages(&mut self, ecs: &mut World) {
        ecs.write_message_batch(self.login.drain(..));
        ecs.write_message_batch(self.config.drain(..));
        ecs.write_message_batch(self.game.drain(..));
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
    /// like [`SendGamePacketEvent`](crate::packet::game::SendGamePacketEvent) /
    /// [`SendConfigPacketEvent`](crate::packet::config::SendConfigPacketEvent)
    /// / [`SendLoginPacketEvent`](crate::packet::login::SendLoginPacketEvent).
    ///
    /// To check if we haven't disconnected from the server, use
    /// [`Self::is_alive`].
    pub(crate) network: Option<NetworkConnection>,
    pub state: ConnectionProtocol,
    pub(crate) is_alive: bool,

    /// This exists for internal testing purposes and probably shouldn't be used
    /// for normal bots.
    ///
    /// It's basically a way to make our client think it received a packet from
    /// the server without needing to interact with the network.
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
    /// This is called by the handlers for [`SendGamePacketEvent`],
    /// [`SendConfigPacketEvent`], and [`SendLoginPacketEvent`].
    ///
    /// [`SendGamePacketEvent`]: crate::packet::game::SendGamePacketEvent
    /// [`SendConfigPacketEvent`]: crate::packet::config::SendConfigPacketEvent
    /// [`SendLoginPacketEvent`]: crate::packet::login::SendLoginPacketEvent
    pub fn write<P: ProtocolPacket + Debug>(
        &mut self,
        packet: impl Packet<P>,
    ) -> Result<(), WritePacketError> {
        if let Some(network) = &mut self.network {
            network.write(packet)?;
        } else {
            static WARNED: AtomicBool = AtomicBool::new(false);
            if !WARNED.swap(true, atomic::Ordering::Relaxed) {
                debug!(
                    "tried to write packet to the network but there is no NetworkConnection. if you're trying to send a packet from the handler function, use self.write instead"
                );
            }
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
    state: ConnectionProtocol,
    queued_packet_events: &mut QueuedPacketEvents,
) -> Result<(), Box<ReadPacketError>> {
    let stream = &mut Cursor::new(raw_packet);
    match state {
        ConnectionProtocol::Handshake => {
            unreachable!()
        }
        ConnectionProtocol::Game => {
            let deser_start = Instant::now();
            let packet = Arc::new(deserialize_packet::<ClientboundGamePacket>(stream)?);
            let deser_elapsed = deser_start.elapsed();
            if deser_elapsed.as_millis() > 1 {
                trace!(
                    packet_type = "ClientboundGamePacket",
                    duration_ms = deser_elapsed.as_millis(),
                    entity = ?entity,
                    "Slow deserialization"
                );
            }
            trace!("Packet: {packet:?}");
            game::process_packet(ecs, entity, packet.as_ref());
            queued_packet_events
                .game
                .push(ReceiveGamePacketEvent { entity, packet });
        }
        ConnectionProtocol::Status => {
            unreachable!()
        }
        ConnectionProtocol::Login => {
            let deser_start = Instant::now();
            let packet = Arc::new(deserialize_packet::<ClientboundLoginPacket>(stream)?);
            let deser_elapsed = deser_start.elapsed();
            if deser_elapsed.as_millis() > 1 {
                trace!(
                    packet_type = "ClientboundLoginPacket",
                    duration_ms = deser_elapsed.as_millis(),
                    entity = ?entity,
                    "Slow deserialization"
                );
            }
            trace!("Packet: {packet:?}");
            login::process_packet(ecs, entity, &packet);
            queued_packet_events
                .login
                .push(ReceiveLoginPacketEvent { entity, packet });
        }
        ConnectionProtocol::Configuration => {
            let deser_start = Instant::now();
            let packet = Arc::new(deserialize_packet::<ClientboundConfigPacket>(stream)?);
            let deser_elapsed = deser_start.elapsed();
            if deser_elapsed.as_millis() > 1 {
                trace!(
                    packet_type = "ClientboundConfigPacket",
                    duration_ms = deser_elapsed.as_millis(),
                    entity = ?entity,
                    "Slow deserialization"
                );
            }
            trace!("Packet: {packet:?}");
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
    /// A queue of raw TCP packets to send.
    ///
    /// These will not be modified further, they should already be serialized
    /// and compressed and encrypted before being added here.
    network_packet_writer_tx: mpsc::UnboundedSender<Box<[u8]>>,
}
impl NetworkConnection {
    pub fn write<P: ProtocolPacket + Debug>(
        &mut self,
        packet: impl Packet<P>,
    ) -> Result<(), WritePacketError> {
        let packet = packet.into_variant();
        let raw_packet = serialize_packet(&packet)?;
        self.write_raw(&raw_packet)?;

        Ok(())
    }

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

    /// Makes sure packets get sent and returns Some(()) if the connection has
    /// closed.
    pub fn poll_writer(&mut self) -> Option<()> {
        let poll_once_res = future::poll_once(&mut self.writer_task);
        future::block_on(poll_once_res)
    }

    pub fn set_compression_threshold(&mut self, threshold: Option<u32>) {
        trace!("Set compression threshold to {threshold:?}");
        self.reader.compression_threshold = threshold;
    }
    /// Set the encryption key that is used to encrypt and decrypt packets.
    ///
    /// The same key is used for both reading and writing.
    pub fn set_encryption_key(&mut self, key: [u8; 16]) {
        trace!("Enabled protocol encryption");
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

    trace!("write task is done");
}

#[derive(Debug, Error)]
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

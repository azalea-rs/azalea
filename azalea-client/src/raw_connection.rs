use std::fmt::Debug;
use std::sync::Arc;

use azalea_protocol::{
    connect::{RawReadConnection, RawWriteConnection},
    packets::{ConnectionProtocol, Packet, ProtocolPacket},
    read::ReadPacketError,
    write::serialize_packet,
};
use bevy_ecs::prelude::*;
use parking_lot::Mutex;
use thiserror::Error;
use tokio::sync::mpsc::{
    self,
    error::{SendError, TrySendError},
};
use tracing::error;

/// A component for clients that can read and write packets to the server. This
/// works with raw bytes, so you'll have to serialize/deserialize packets
/// yourself. It will do the compression and encryption for you though.
#[derive(Component)]
pub struct RawConnection {
    pub reader: RawConnectionReader,
    pub writer: RawConnectionWriter,

    /// Packets sent to this will be sent to the server.
    /// A task that reads packets from the server. The client is disconnected
    /// when this task ends.
    pub read_packets_task: tokio::task::JoinHandle<()>,
    /// A task that writes packets from the server.
    pub write_packets_task: tokio::task::JoinHandle<()>,

    pub connection_protocol: ConnectionProtocol,
}

#[derive(Clone)]
pub struct RawConnectionReader {
    pub incoming_packet_queue: Arc<Mutex<Vec<Box<[u8]>>>>,
    pub run_schedule_sender: mpsc::Sender<()>,
}
#[derive(Clone)]
pub struct RawConnectionWriter {
    pub outgoing_packets_sender: mpsc::UnboundedSender<Box<[u8]>>,
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
        source: SendError<Box<[u8]>>,
    },
}

impl RawConnection {
    pub fn new(
        run_schedule_sender: mpsc::Sender<()>,
        connection_protocol: ConnectionProtocol,
        raw_read_connection: RawReadConnection,
        raw_write_connection: RawWriteConnection,
    ) -> Self {
        let (outgoing_packets_sender, outgoing_packets_receiver) = mpsc::unbounded_channel();

        let incoming_packet_queue = Arc::new(Mutex::new(Vec::new()));

        let reader = RawConnectionReader {
            incoming_packet_queue: incoming_packet_queue.clone(),
            run_schedule_sender,
        };
        let writer = RawConnectionWriter {
            outgoing_packets_sender,
        };

        let read_packets_task = tokio::spawn(reader.clone().read_task(raw_read_connection));
        let write_packets_task = tokio::spawn(
            writer
                .clone()
                .write_task(raw_write_connection, outgoing_packets_receiver),
        );

        Self {
            reader,
            writer,
            read_packets_task,
            write_packets_task,
            connection_protocol,
        }
    }

    pub fn write_raw_packet(&self, raw_packet: Box<[u8]>) -> Result<(), WritePacketError> {
        self.writer.outgoing_packets_sender.send(raw_packet)?;
        Ok(())
    }

    /// Write the packet with the given state to the server.
    ///
    /// # Errors
    ///
    /// Returns an error if the packet is not valid for the current state, or if
    /// encoding it failed somehow (like it's too big or something).
    pub fn write_packet<P: ProtocolPacket + Debug>(
        &self,
        packet: impl Packet<P>,
    ) -> Result<(), WritePacketError> {
        let packet = packet.into_variant();
        let raw_packet = serialize_packet(&packet)?;
        self.write_raw_packet(raw_packet)?;

        Ok(())
    }

    /// Returns whether the connection is still alive.
    pub fn is_alive(&self) -> bool {
        !self.read_packets_task.is_finished()
    }

    pub fn incoming_packet_queue(&self) -> Arc<Mutex<Vec<Box<[u8]>>>> {
        self.reader.incoming_packet_queue.clone()
    }

    pub fn set_state(&mut self, connection_protocol: ConnectionProtocol) {
        self.connection_protocol = connection_protocol;
    }
}

impl RawConnectionReader {
    /// Loop that reads from the connection and adds the packets to the queue +
    /// runs the schedule.
    pub async fn read_task(self, mut read_conn: RawReadConnection) {
        fn log_for_error(error: &ReadPacketError) {
            if !matches!(*error, ReadPacketError::ConnectionClosed) {
                error!("Error reading packet from Client: {error:?}");
            }
        }

        loop {
            match read_conn.read().await {
                Ok(raw_packet) => {
                    let mut incoming_packet_queue = self.incoming_packet_queue.lock();

                    incoming_packet_queue.push(raw_packet);
                    // this makes it so packets received at the same time are guaranteed to be
                    // handled in the same tick. this is also an attempt at making it so we can't
                    // receive any packets in the ticks/updates after being disconnected.
                    loop {
                        let raw_packet = match read_conn.try_read() {
                            Ok(p) => p,
                            Err(err) => {
                                log_for_error(&err);
                                return;
                            }
                        };
                        let Some(raw_packet) = raw_packet else { break };
                        incoming_packet_queue.push(raw_packet);
                    }

                    // tell the client to run all the systems
                    if self.run_schedule_sender.try_send(()) == Err(TrySendError::Closed(())) {
                        // the client was dropped
                        break;
                    }
                }
                Err(err) => {
                    log_for_error(&err);
                    return;
                }
            }
        }
    }
}

impl RawConnectionWriter {
    /// Consume the [`ServerboundGamePacket`] queue and actually write the
    /// packets to the server. It's like this so writing packets doesn't need to
    /// be awaited.
    ///
    /// [`ServerboundGamePacket`]: azalea_protocol::packets::game::ServerboundGamePacket
    pub async fn write_task(
        self,
        mut write_conn: RawWriteConnection,
        mut outgoing_packets_receiver: mpsc::UnboundedReceiver<Box<[u8]>>,
    ) {
        while let Some(raw_packet) = outgoing_packets_receiver.recv().await {
            if let Err(err) = write_conn.write(&raw_packet).await {
                error!("Disconnecting because we couldn't write a packet: {err}.");
                break;
            };
        }
        // receiver is automatically closed when it's dropped
    }
}

impl Drop for RawConnection {
    /// Stop every active task when this `RawConnection` is dropped.
    fn drop(&mut self) {
        self.read_packets_task.abort();
        self.write_packets_task.abort();
    }
}

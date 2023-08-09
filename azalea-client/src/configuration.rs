use std::sync::Arc;

use azalea_protocol::{
    connect::{ReadConnection, WriteConnection},
    packets::configuration::{ClientboundConfigurationPacket, ServerboundConfigurationPacket},
};
use bevy_ecs::prelude::*;
use tokio::{sync::mpsc, task::JoinHandle};

/// A local player that's in the configuration protocol state.
#[derive(Component)]
pub struct ConfigurationLocalPlayer {
    packet_writer: mpsc::UnboundedSender<ServerboundConfigurationPacket>,

    // it has to be a mutex so it can be taken out from the task,
    // and Option so the Connection can be reconstructed
    pub(crate) read_conn:
        Arc<tokio::sync::Mutex<Option<ReadConnection<ClientboundConfigurationPacket>>>>,
    pub(crate) write_conn:
        Arc<tokio::sync::Mutex<Option<WriteConnection<ServerboundConfigurationPacket>>>>,

    /// A task that reads packets from the server. The client is disconnected
    /// when this task ends.
    pub(crate) read_packets_task: JoinHandle<()>,
    /// A task that writes packets from the server.
    pub(crate) write_packets_task: JoinHandle<()>,
}

impl ConfigurationLocalPlayer {
    /// Create a new `LocalPlayer`.
    pub fn new(
        packet_writer: mpsc::UnboundedSender<ServerboundConfigurationPacket>,

        read_conn: Arc<tokio::sync::Mutex<Option<ReadConnection<ClientboundConfigurationPacket>>>>,
        write_conn: Arc<
            tokio::sync::Mutex<Option<WriteConnection<ServerboundConfigurationPacket>>>,
        >,

        read_packets_task: JoinHandle<()>,
        write_packets_task: JoinHandle<()>,
    ) -> Self {
        ConfigurationLocalPlayer {
            packet_writer,

            read_conn,
            write_conn,

            read_packets_task,
            write_packets_task,
        }
    }

    /// Write a packet directly to the server.
    pub fn write_packet(&self, packet: ServerboundConfigurationPacket) {
        self.packet_writer
            .send(packet)
            .expect("write_packet shouldn't be able to be called if the connection is closed");
    }
}

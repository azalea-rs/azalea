use azalea_protocol::packets::configuration::ServerboundConfigurationPacket;
use bevy_ecs::prelude::*;
use tokio::{sync::mpsc, task::JoinHandle};

/// A local player that's in the configuration protocol state.
#[derive(Component)]
pub struct ConfiguringLocalPlayer {
    packet_writer: mpsc::UnboundedSender<ServerboundConfigurationPacket>,

    /// A task that reads packets from the server. The client is disconnected
    /// when this task ends.
    pub(crate) read_packets_task: JoinHandle<()>,
    /// A task that writes packets from the server.
    pub(crate) write_packets_task: JoinHandle<()>,
}

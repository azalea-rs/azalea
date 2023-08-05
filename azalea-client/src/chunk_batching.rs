//! Used for Minecraft's chunk batching introduced in 23w31a (1.20.2). It's used
//! for making the server spread out how often it sends us chunk packets
//! depending on our receiving speed.

use std::time::{Duration, Instant};

use azalea_protocol::packets::game::serverbound_chunk_batch_received_packet::ServerboundChunkBatchReceivedPacket;
use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;

use crate::local_player::{handle_send_packet_event, SendPacketEvent};

pub struct ChunkBatchingPlugin;
impl Plugin for ChunkBatchingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_chunk_batch_start_event,
                handle_chunk_batch_finished_event,
            )
                .chain()
                .before(handle_send_packet_event),
        );
    }
}

#[derive(Component, Clone, Debug)]
pub struct ChunkBatchInfo {
    pub start_time: Instant,
    pub accumulator: ChunkReceiveSpeedAccumulator,
}

#[derive(Event)]
pub struct ChunkBatchStartEvent {
    pub entity: Entity,
}
#[derive(Event)]
pub struct ChunkBatchFinishedEvent {
    pub entity: Entity,
    pub batch_size: u32,
}

pub fn handle_chunk_batch_start_event(
    mut query: Query<&mut ChunkBatchInfo>,
    mut events: EventReader<ChunkBatchStartEvent>,
) {
    for event in events.iter() {
        if let Ok(mut chunk_batch_info) = query.get_mut(event.entity) {
            chunk_batch_info.start_time = Instant::now();
        }
    }
}

pub fn handle_chunk_batch_finished_event(
    mut query: Query<&mut ChunkBatchInfo>,
    mut events: EventReader<ChunkBatchFinishedEvent>,
    mut send_packets: EventWriter<SendPacketEvent>,
) {
    for event in events.iter() {
        if let Ok(mut chunk_batch_info) = query.get_mut(event.entity) {
            let batch_duration = chunk_batch_info.start_time.elapsed();
            if event.batch_size > 0 {
                chunk_batch_info
                    .accumulator
                    .accumulate(event.batch_size, batch_duration);
            }
            let millis_per_chunk =
                f64::max(0., chunk_batch_info.accumulator.get_millis_per_chunk());
            let desired_batch_size = if millis_per_chunk == 0. {
                // make it the server's problem instead
                f32::NAN
            } else {
                (25. / millis_per_chunk) as f32
            };
            send_packets.send(SendPacketEvent {
                entity: event.entity,
                packet: ServerboundChunkBatchReceivedPacket { desired_batch_size }.get(),
            });
        }
    }
}

#[derive(Clone, Debug)]
pub struct ChunkReceiveSpeedAccumulator {
    batch_sizes: Vec<u32>,
    /// as milliseconds
    batch_durations: Vec<u32>,
    index: usize,
    filled_size: usize,
}
impl ChunkReceiveSpeedAccumulator {
    pub fn new(capacity: usize) -> Self {
        Self {
            batch_sizes: vec![0; capacity],
            batch_durations: vec![0; capacity],
            index: 0,
            filled_size: 0,
        }
    }

    pub fn accumulate(&mut self, batch_size: u32, batch_duration: Duration) {
        self.batch_sizes[self.index] = batch_size;
        self.batch_durations[self.index] =
            f32::clamp(batch_duration.as_millis() as f32, 0., 15000.) as u32;
        self.index = (self.index + 1) % self.batch_sizes.len();
        if self.filled_size < self.batch_sizes.len() {
            self.filled_size += 1;
        }
    }

    pub fn get_millis_per_chunk(&self) -> f64 {
        let mut total_batch_size = 0;
        let mut total_batch_duration = 0;
        for i in 0..self.filled_size {
            total_batch_size += self.batch_sizes[i];
            total_batch_duration += self.batch_durations[i];
        }
        if total_batch_size == 0 {
            return 0.;
        }
        total_batch_duration as f64 / total_batch_size as f64
    }
}

impl Default for ChunkBatchInfo {
    fn default() -> Self {
        Self {
            start_time: Instant::now(),
            accumulator: ChunkReceiveSpeedAccumulator::new(50),
        }
    }
}

//! Adds utility functions that all depend on the pathfinder.

pub mod process;

use azalea_client::Client;
use azalea_core::{position::BlockPos, tick::GameTick};
use bevy_app::Update;

use crate::app::{App, Plugin};

use self::process::{Process, SetActiveProcessEvent};

pub struct PathfinderExtrasPlugin;

impl Plugin for PathfinderExtrasPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SetActiveProcessEvent>()
            .add_systems(Update, process::set_active_pathfinder_process_listener)
            .add_systems(GameTick, process::process_tick);
    }
}

pub trait PathfinderExtrasClientExt {
    fn set_active_pathfinder_process(&self, process: Process);
    fn mine_area(&self, corner1: BlockPos, corner2: BlockPos);
}

impl PathfinderExtrasClientExt for Client {
    fn set_active_pathfinder_process(&self, process: Process) {
        self.ecs.lock().send_event(SetActiveProcessEvent {
            entity: self.entity,
            process,
        });
    }

    fn mine_area(&self, corner1: BlockPos, corner2: BlockPos) {
        self.set_active_pathfinder_process(Process::MineArea { corner1, corner2 });
    }
}

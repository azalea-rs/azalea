//! A debugging plugin that logs the duration of ECS `Update`s every tick.

use std::time::{Duration, Instant};

use azalea::prelude::Resource;
use bevy_app::Plugin;
use bevy_ecs::{schedule::IntoScheduleConfigs, system::ResMut};

pub struct MsptPlugin;
impl Plugin for MsptPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.insert_resource(MsptData {
            last_log_time: Instant::now(),
            this_update_start_time: Instant::now(),
            update_times: Vec::new(),
        })
        .add_systems(bevy_app::PreUpdate, on_update_start)
        .add_systems(
            bevy_app::PostUpdate,
            (on_update_end, log_mspt_stats).chain(),
        );
    }
}

#[derive(Resource)]
struct MsptData {
    this_update_start_time: Instant,
    last_log_time: Instant,

    update_times: Vec<Duration>,
}
fn log_mspt_stats(mut stats: ResMut<MsptData>) {
    if stats.last_log_time.elapsed() < Duration::from_secs(1) {
        return;
    }
    stats.last_log_time = Instant::now();

    let mut fastest_update_duration = None;
    let mut summed_update_durations = Duration::ZERO;
    let mut num_updates = 0;
    for update in stats.update_times.drain(..) {
        summed_update_durations += update;
        num_updates += 1;
        let Some(fastest_update) = &mut fastest_update_duration else {
            fastest_update_duration = Some(update);
            continue;
        };
        if update < *fastest_update {
            *fastest_update = update;
        }
    }

    if num_updates > 0
        && let Some(fastest_update_duration) = fastest_update_duration
    {
        let avg_update_duration = summed_update_durations / num_updates;

        println!();
        println!("Average update duration: {avg_update_duration:?}");
        println!("Fastest update duration: {fastest_update_duration:?}");
    }
}
fn on_update_start(mut data: ResMut<MsptData>) {
    data.this_update_start_time = Instant::now();
}
fn on_update_end(mut data: ResMut<MsptData>) {
    let elapsed = data.this_update_start_time.elapsed();
    data.update_times.push(elapsed);
}

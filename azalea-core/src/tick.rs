use bevy_ecs::schedule::ScheduleLabel;

#[derive(ScheduleLabel, Hash, Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct GameTick;

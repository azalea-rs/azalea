use bevy_ecs::schedule::ScheduleLabel;

/// A Bevy schedule that runs every Minecraft game tick, i.e. every 50ms.
///
/// Many client systems run on this schedule, the most important one being
/// physics.
///
/// This schedule runs either zero or one times after every Bevy `Update`.
#[derive(ScheduleLabel, Hash, Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct GameTick;

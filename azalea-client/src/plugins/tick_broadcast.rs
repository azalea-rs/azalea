use azalea_core::tick::GameTick;
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use derive_more::Deref;
use tokio::sync::broadcast;

/// A resource that contains a [`broadcast::Sender`] that will be sent every
/// Minecraft tick.
///
/// This is useful for running code every schedule from async user code.
///
/// ```
/// use azalea_client::tick_broadcast::TickBroadcast;
/// # async fn example(client: azalea_client::Client) {
/// let mut receiver = {
///     let ecs = client.ecs.lock();
///     let tick_broadcast = ecs.resource::<TickBroadcast>();
///     tick_broadcast.subscribe()
/// };
/// while receiver.recv().await.is_ok() {
///     // do something
/// }
/// # }
/// ```
#[derive(Deref, Resource)]
pub struct TickBroadcast(broadcast::Sender<()>);
/// A resource that contains a [`broadcast::Sender`] that will be sent every
/// Azalea ECS Update.
///
/// Also see [`TickBroadcast`].
#[derive(Deref, Resource)]
pub struct UpdateBroadcast(broadcast::Sender<()>);

pub fn send_tick_broadcast(tick_broadcast: ResMut<TickBroadcast>) {
    let _ = tick_broadcast.0.send(());
}
pub fn send_update_broadcast(update_broadcast: ResMut<UpdateBroadcast>) {
    let _ = update_broadcast.0.send(());
}
/// A plugin that makes the [`UpdateBroadcast`] and [`TickBroadcast`] resources
/// available.
pub struct TickBroadcastPlugin;
impl Plugin for TickBroadcastPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TickBroadcast(broadcast::channel(1).0))
            .insert_resource(UpdateBroadcast(broadcast::channel(1).0))
            .add_systems(GameTick, send_tick_broadcast)
            .add_systems(Update, send_update_broadcast);
    }
}

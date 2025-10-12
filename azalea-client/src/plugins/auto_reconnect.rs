//! Auto-reconnect to the server when the client is kicked.
//!
//! See [`AutoReconnectPlugin`] for more information.

use std::time::{Duration, Instant};

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

use super::{
    disconnect::DisconnectEvent,
    events::LocalPlayerEvents,
    join::{ConnectOpts, ConnectionFailedEvent, StartJoinServerEvent},
};
use crate::Account;

/// The default delay that Azalea will use for reconnecting our clients.
///
/// See [`AutoReconnectPlugin`] for more information.
pub const DEFAULT_RECONNECT_DELAY: Duration = Duration::from_secs(5);

/// A default plugin that makes clients automatically rejoin the server when
/// they're disconnected.
///
/// The reconnect delay is configurable globally or per-client with the
/// [`AutoReconnectDelay`] resource/component. Auto reconnecting can be disabled
/// by removing the resource from the ECS.
///
/// The delay defaults to [`DEFAULT_RECONNECT_DELAY`].
pub struct AutoReconnectPlugin;
impl Plugin for AutoReconnectPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AutoReconnectDelay::new(DEFAULT_RECONNECT_DELAY))
            .add_systems(
                Update,
                (start_rejoin_on_disconnect, rejoin_after_delay)
                    .chain()
                    .before(super::join::handle_start_join_server_event),
            );
    }
}

pub fn start_rejoin_on_disconnect(
    mut commands: Commands,
    mut disconnect_events: MessageReader<DisconnectEvent>,
    mut connection_failed_events: MessageReader<ConnectionFailedEvent>,
    auto_reconnect_delay_res: Option<Res<AutoReconnectDelay>>,
    auto_reconnect_delay_query: Query<&AutoReconnectDelay>,
) {
    for entity in disconnect_events
        .read()
        .map(|e| e.entity)
        .chain(connection_failed_events.read().map(|e| e.entity))
    {
        let Some(delay) = get_delay(
            &auto_reconnect_delay_res,
            auto_reconnect_delay_query,
            entity,
        ) else {
            // no auto reconnect
            continue;
        };

        let reconnect_after = Instant::now() + delay;
        commands.entity(entity).insert(InternalReconnectAfter {
            instant: reconnect_after,
        });
    }
}

fn get_delay(
    auto_reconnect_delay_res: &Option<Res<AutoReconnectDelay>>,
    auto_reconnect_delay_query: Query<&AutoReconnectDelay>,
    entity: Entity,
) -> Option<Duration> {
    let delay = if let Ok(c) = auto_reconnect_delay_query.get(entity) {
        Some(c.delay)
    } else {
        auto_reconnect_delay_res.as_ref().map(|r| r.delay)
    };

    if delay == Some(Duration::MAX) {
        // if the duration is set to max, treat that as autoreconnect being disabled
        return None;
    }
    delay
}

pub fn rejoin_after_delay(
    mut commands: Commands,
    mut join_events: MessageWriter<StartJoinServerEvent>,
    query: Query<(
        Entity,
        &InternalReconnectAfter,
        &Account,
        &ConnectOpts,
        Option<&LocalPlayerEvents>,
    )>,
) {
    for (entity, reconnect_after, account, connect_opts, local_player_events) in query.iter() {
        if Instant::now() >= reconnect_after.instant {
            // don't keep trying to reconnect
            commands.entity(entity).remove::<InternalReconnectAfter>();

            // our Entity will be reused since the account has the same uuid
            join_events.write(StartJoinServerEvent {
                account: account.clone(),
                connect_opts: connect_opts.clone(),
                // not actually necessary since we're reusing the same entity and LocalPlayerEvents
                // isn't removed, but this is more readable and just in case it's changed in the
                // future
                event_sender: local_player_events.map(|e| e.0.clone()),
                start_join_callback_tx: None,
            });
        }
    }
}

/// A resource *and* component that indicates how long to wait before
/// reconnecting when we're kicked.
///
/// Initially, it's a resource in the ECS set to 5 seconds. You can modify
/// the resource to update the global reconnect delay, or insert it as a
/// component to set the individual delay for a single client.
///
/// You can also remove this resource from the ECS to disable the default
/// auto-reconnecting behavior. Inserting the resource/component again will not
/// make clients that were already disconnected automatically reconnect.
#[derive(Resource, Component, Debug, Clone)]
pub struct AutoReconnectDelay {
    pub delay: Duration,
}
impl AutoReconnectDelay {
    pub fn new(delay: Duration) -> Self {
        Self { delay }
    }
}

/// This is inserted when we're disconnected and indicates when we'll reconnect.
///
/// This is set based on [`AutoReconnectDelay`].
#[derive(Component, Debug, Clone)]
pub struct InternalReconnectAfter {
    pub instant: Instant,
}

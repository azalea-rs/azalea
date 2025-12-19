//! Arbitrary data sent by the server that gets temporarily stored on the
//! client.

use std::collections::HashMap;

use azalea_protocol::packets::{
    config,
    game::{self},
    login,
};
use azalea_registry::identifier::Identifier;
use bevy_app::{App, Plugin};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    event::EntityEvent,
    observer::On,
    system::{Commands, Query},
};
use tracing::warn;

use crate::{
    InConfigState, InGameState,
    packet::{
        config::SendConfigPacketEvent,
        game::SendGamePacketEvent,
        login::{InLoginState, SendLoginPacketEvent},
    },
};

pub struct CookiesPlugin;
impl Plugin for CookiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_request_cookie)
            .add_observer(handle_store_cookie);
    }
}

/// A component that holds arbitrary data sent by the server, that our client
/// temporarily stores and persists across transfers.
#[derive(Component, Default)]
pub struct ServerCookies {
    pub map: HashMap<Identifier, Vec<u8>>,
}

#[derive(EntityEvent)]
pub struct RequestCookieEvent {
    pub entity: Entity,
    pub key: Identifier,
}
#[derive(EntityEvent)]
pub struct StoreCookieEvent {
    pub entity: Entity,
    pub key: Identifier,
    pub payload: Vec<u8>,
}

#[allow(clippy::type_complexity)]
pub fn handle_request_cookie(
    request_cookie: On<RequestCookieEvent>,
    mut commands: Commands,
    query: Query<(
        Option<&ServerCookies>,
        Option<&InGameState>,
        Option<&InConfigState>,
        Option<&InLoginState>,
    )>,
) {
    let Ok((server_cookies, in_game_state, in_config_state, in_login_state)) =
        query.get(request_cookie.entity)
    else {
        return;
    };

    let key = request_cookie.key.clone();
    let payload = server_cookies.and_then(|c| c.map.get(&key)).cloned();

    if in_game_state.is_some() {
        commands.trigger(SendGamePacketEvent::new(
            request_cookie.entity,
            game::ServerboundCookieResponse { key, payload },
        ));
    } else if in_config_state.is_some() {
        commands.trigger(SendConfigPacketEvent::new(
            request_cookie.entity,
            config::ServerboundCookieResponse { key, payload },
        ));
    } else if in_login_state.is_some() {
        commands.trigger(SendLoginPacketEvent::new(
            request_cookie.entity,
            login::ServerboundCookieResponse { key, payload },
        ));
    } else {
        warn!("got RequestCookieEvent while in an unknown state")
    }
}
pub fn handle_store_cookie(
    store_cookie: On<StoreCookieEvent>,
    mut query: Query<&mut ServerCookies>,
) {
    if let Ok(mut server_cookies) = query.get_mut(store_cookie.entity) {
        server_cookies
            .map
            .insert(store_cookie.key.clone(), store_cookie.payload.clone());
    } else {
        warn!("got StoreCookieEvent for a client without ServerCookies")
    }
}

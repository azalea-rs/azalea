#[cfg(feature = "online-mode")]
use azalea_auth::sessionserver::ClientSessionServerError;
use azalea_protocol::{
    connect::Proxy,
    packets::login::{ClientboundHello, ServerboundCustomQueryAnswer, ServerboundKey},
};
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_tasks::{IoTaskPool, Task, futures_lite::future};
use thiserror::Error;
use tracing::{debug, error, trace, warn};

use super::{
    connection::RawConnection,
    packet::login::{ReceiveCustomQueryEvent, ReceiveHelloEvent, SendLoginPacketEvent},
};
use crate::{Account, join::ConnectOpts};

/// Some systems that run during the `login` state.
pub struct LoginPlugin;
impl Plugin for LoginPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_receive_hello_event)
            .add_systems(Update, (poll_auth_task, reply_to_custom_queries));
    }
}

fn handle_receive_hello_event(
    receive_hello: On<ReceiveHelloEvent>,
    mut commands: Commands,
    query: Query<&ConnectOpts>,
) {
    let task_pool = IoTaskPool::get();

    let account = receive_hello.account.clone();
    let packet = receive_hello.packet.clone();
    let client = receive_hello.entity;

    // we store the auth proxy in the ConnectOpts component to make it easily
    // configurable. that component should've definitely been inserted by now, but
    // if it somehow wasn't then we should let the user know.
    let connect_opts = if let Ok(opts) = query.get(client) {
        opts.sessionserver_proxy.clone()
    } else {
        warn!("ConnectOpts component missing on a client ({client}) that got ReceiveHelloEvent");
        None
    };

    let task = task_pool.spawn(auth_with_account(account, packet, connect_opts));
    commands.entity(client).insert(AuthTask(task));
}

/// A marker component on our clients that indicates that the server is
/// online-mode and the client has authenticated their join with Mojang.
#[derive(Component)]
pub struct IsAuthenticated;

pub fn poll_auth_task(
    mut commands: Commands,
    mut query: Query<(Entity, &mut AuthTask, &mut RawConnection)>,
) {
    for (entity, mut auth_task, mut raw_conn) in query.iter_mut() {
        if let Some(poll_res) = future::block_on(future::poll_once(&mut auth_task.0)) {
            debug!("Finished auth");
            commands
                .entity(entity)
                .remove::<AuthTask>()
                .insert(IsAuthenticated);
            match poll_res {
                Ok((packet, private_key)) => {
                    // we use this instead of SendLoginPacketEvent to ensure that it's sent right
                    // before encryption is enabled. i guess another option would be to make a
                    // Trigger+observer for set_encryption_key; the current implementation is
                    // simpler though.
                    if let Err(e) = raw_conn.write(packet) {
                        error!("Error sending key packet: {e:?}");
                    }
                    if let Some(net_conn) = raw_conn.net_conn() {
                        net_conn.set_encryption_key(private_key);
                    }
                }
                Err(err) => {
                    error!("Error during authentication: {err:?}");
                }
            }
        }
    }
}

type PrivateKey = [u8; 16];

#[derive(Component)]
pub struct AuthTask(Task<Result<(ServerboundKey, PrivateKey), AuthWithAccountError>>);

#[derive(Debug, Error)]
pub enum AuthWithAccountError {
    #[error("Failed to encrypt the challenge from the server for {0:?}")]
    Encryption(ClientboundHello),
    #[cfg(feature = "online-mode")]
    #[error("{0}")]
    SessionServer(#[from] ClientSessionServerError),
    #[cfg(feature = "online-mode")]
    #[error("Couldn't refresh access token: {0}")]
    Auth(#[from] azalea_auth::AuthError),
}

pub async fn auth_with_account(
    account: Account,
    packet: ClientboundHello,
    proxy: Option<Proxy>,
) -> Result<(ServerboundKey, PrivateKey), AuthWithAccountError> {
    let Ok(encrypt_res) = azalea_crypto::encrypt(&packet.public_key, &packet.challenge) else {
        return Err(AuthWithAccountError::Encryption(packet));
    };
    let key_packet = ServerboundKey {
        key_bytes: encrypt_res.encrypted_public_key,
        encrypted_challenge: encrypt_res.encrypted_challenge,
    };
    let private_key = encrypt_res.secret_key;

    #[cfg(not(feature = "online-mode"))]
    let _ = (account, proxy);

    #[cfg(feature = "online-mode")]
    if packet.should_authenticate {
        let Some(access_token) = &account.access_token else {
            // offline mode account, no need to do auth
            return Ok((key_packet, private_key));
        };

        // keep track of the number of times we tried authenticating so we can give up
        // after too many
        let mut attempts: usize = 1;

        let proxy = proxy.map(Proxy::into);

        while let Err(err) = {
            use azalea_auth::sessionserver::{self, SessionServerJoinOpts};

            let access_token = access_token.lock().clone();

            let uuid = &account
                .uuid
                .expect("Uuid must be present if access token is present.");

            let proxy = proxy.clone();

            // this is necessary since reqwest usually depends on tokio and we're using
            // `futures` here
            async_compat::Compat::new(sessionserver::join(SessionServerJoinOpts {
                access_token: &access_token,
                public_key: &packet.public_key,
                private_key: &private_key,
                uuid,
                server_id: &packet.server_id,
                proxy,
            }))
            .await
        } {
            if attempts >= 2 {
                // if this is the second attempt and we failed
                // both times, give up
                return Err(err.into());
            }
            if matches!(
                err,
                ClientSessionServerError::InvalidSession
                    | ClientSessionServerError::ForbiddenOperation
            ) {
                // uh oh, we got an invalid session and have
                // to reauthenticate now
                async_compat::Compat::new(account.refresh()).await?;
            } else {
                return Err(err.into());
            }
            attempts += 1;
        }
    }

    Ok((key_packet, private_key))
}

pub fn reply_to_custom_queries(
    mut commands: Commands,
    mut events: MessageReader<ReceiveCustomQueryEvent>,
) {
    for event in events.read() {
        trace!("Maybe replying to custom query: {event:?}");
        if event.disabled {
            continue;
        }

        commands.trigger(SendLoginPacketEvent::new(
            event.entity,
            ServerboundCustomQueryAnswer {
                transaction_id: event.packet.transaction_id,
                data: None,
            },
        ));
    }
}

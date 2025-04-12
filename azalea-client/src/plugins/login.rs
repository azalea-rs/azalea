use azalea_auth::sessionserver::ClientSessionServerError;
use azalea_protocol::packets::login::{ClientboundHello, ServerboundKey};
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_tasks::{IoTaskPool, Task, futures_lite::future};
use tracing::{debug, error};

use super::{connection::RawConnection, packet::login::ReceiveHelloEvent};
use crate::{Account, JoinError};

pub struct LoginPlugin;
impl Plugin for LoginPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_receive_hello_event)
            .add_systems(Update, poll_auth_task);
    }
}

fn handle_receive_hello_event(trigger: Trigger<ReceiveHelloEvent>, mut commands: Commands) {
    let task_pool = IoTaskPool::get();

    let account = trigger.account.clone();
    let packet = trigger.packet.clone();
    let player = trigger.entity();

    let task = task_pool.spawn(auth_with_account(account, packet));
    commands.entity(player).insert(AuthTask(task));
}

fn poll_auth_task(
    mut commands: Commands,
    mut query: Query<(Entity, &mut AuthTask, &mut RawConnection)>,
) {
    for (entity, mut auth_task, mut raw_conn) in query.iter_mut() {
        if let Some(poll_res) = future::block_on(future::poll_once(&mut auth_task.0)) {
            debug!("Finished auth");
            commands.entity(entity).remove::<AuthTask>();
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
pub struct AuthTask(Task<Result<(ServerboundKey, PrivateKey), JoinError>>);

pub async fn auth_with_account(
    account: Account,
    packet: ClientboundHello,
) -> Result<(ServerboundKey, PrivateKey), JoinError> {
    let Ok(encrypt_res) = azalea_crypto::encrypt(&packet.public_key, &packet.challenge) else {
        return Err(JoinError::EncryptionError(packet));
    };
    let key_packet = ServerboundKey {
        key_bytes: encrypt_res.encrypted_public_key,
        encrypted_challenge: encrypt_res.encrypted_challenge,
    };
    let private_key = encrypt_res.secret_key;

    let Some(access_token) = &account.access_token else {
        // offline mode account, no need to do auth
        return Ok((key_packet, private_key));
    };

    // keep track of the number of times we tried authenticating so we can give up
    // after too many
    let mut attempts: usize = 1;

    while let Err(err) = {
        let access_token = access_token.lock().clone();

        let uuid = &account
            .uuid
            .expect("Uuid must be present if access token is present.");

        // this is necessary since reqwest usually depends on tokio and we're using
        // `futures` here
        async_compat::Compat::new(async {
            azalea_auth::sessionserver::join(
                &access_token,
                &packet.public_key,
                &private_key,
                uuid,
                &packet.server_id,
            )
            .await
        })
        .await
    } {
        if attempts >= 2 {
            // if this is the second attempt and we failed
            // both times, give up
            return Err(err.into());
        }
        if matches!(
            err,
            ClientSessionServerError::InvalidSession | ClientSessionServerError::ForbiddenOperation
        ) {
            // uh oh, we got an invalid session and have
            // to reauthenticate now
            account.refresh().await?;
        } else {
            return Err(err.into());
        }
        attempts += 1;
    }

    Ok((key_packet, private_key))
}

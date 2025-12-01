use std::time::{Duration, Instant};

use azalea_auth::certs::{Certificates, FetchCertificatesError};
use azalea_protocol::packets::game::{
    ServerboundChatSessionUpdate,
    s_chat_session_update::{ProfilePublicKeyData, RemoteChatSessionData},
};
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_tasks::{IoTaskPool, Task, futures_lite::future};
use chrono::Utc;
use tracing::{debug, error};
use uuid::Uuid;

use super::{chat, login::IsAuthenticated, packet::game::SendGamePacketEvent};
use crate::{Account, InGameState};

pub struct ChatSigningPlugin;
impl Plugin for ChatSigningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                request_certs_if_needed,
                poll_request_certs_task,
                handle_queued_certs_to_send,
            )
                .chain()
                .before(chat::handler::handle_send_chat_kind_event),
        );
    }
}

#[derive(Component)]
pub struct RequestCertsTask(pub Task<Result<Certificates, FetchCertificatesError>>);

/// A component that makes us have to wait until the given time to refresh the
/// certs.
///
/// This is used to avoid spamming requests if requesting certs fails. Usually,
/// we just check [`Certificates::expires_at`].
#[derive(Component, Debug)]
pub struct OnlyRefreshCertsAfter {
    pub refresh_at: Instant,
}
/// A component that's present when this client has sent its certificates to the
/// server.
///
/// This should be removed if you want to re-send the certs.
///
/// If you want to get the client's actual certificates, you can get that from
/// the `certs` in the [`Account`] component.
#[derive(Component)]
pub struct ChatSigningSession {
    pub session_id: Uuid,
    pub messages_sent: u32,
}

pub fn poll_request_certs_task(
    mut commands: Commands,
    mut query: Query<(Entity, &mut RequestCertsTask, &Account)>,
) {
    for (entity, mut auth_task, account) in query.iter_mut() {
        if let Some(poll_res) = future::block_on(future::poll_once(&mut auth_task.0)) {
            debug!("Finished requesting certs");
            commands.entity(entity).remove::<RequestCertsTask>();

            match poll_res {
                Ok(certs) => {
                    commands.entity(entity).insert(QueuedCertsToSend {
                        certs: certs.clone(),
                    });
                    *account.certs.lock() = Some(certs);
                }
                Err(err) => {
                    error!("Error requesting certs: {err:?}. Retrying in an hour.");

                    commands.entity(entity).insert(OnlyRefreshCertsAfter {
                        refresh_at: Instant::now() + Duration::from_secs(60 * 60),
                    });
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn request_certs_if_needed(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &Account,
            Option<&OnlyRefreshCertsAfter>,
            Option<&ChatSigningSession>,
        ),
        (
            Without<RequestCertsTask>,
            With<InGameState>,
            With<IsAuthenticated>,
        ),
    >,
) {
    for (entity, account, only_refresh_certs_after, chat_signing_session) in query.iter_mut() {
        if let Some(only_refresh_certs_after) = only_refresh_certs_after
            && only_refresh_certs_after.refresh_at > Instant::now()
        {
            continue;
        }

        let certs = account.certs.lock();
        let should_refresh = if let Some(certs) = &*certs {
            // certs were already requested and we're waiting for them to refresh

            // but maybe they weren't sent yet, in which case we still want to send the
            // certs
            if chat_signing_session.is_none() {
                true
            } else {
                Utc::now() > certs.expires_at
            }
        } else {
            true
        };
        drop(certs);

        if should_refresh && let Some(access_token) = &account.access_token {
            let task_pool = IoTaskPool::get();

            let access_token = access_token.lock().clone();
            debug!("Started task to fetch certs");
            let task = task_pool.spawn(async_compat::Compat::new(async move {
                azalea_auth::certs::fetch_certificates(&access_token).await
            }));
            commands
                .entity(entity)
                .insert(RequestCertsTask(task))
                .remove::<OnlyRefreshCertsAfter>();
        }
    }
}

/// A component that's present on players that should send their chat signing
/// certificates as soon as possible.
///
/// This is removed when the certificates get sent.
#[derive(Component)]
pub struct QueuedCertsToSend {
    pub certs: Certificates,
}

pub fn handle_queued_certs_to_send(
    mut commands: Commands,
    query: Query<(Entity, &QueuedCertsToSend), With<IsAuthenticated>>,
) {
    for (entity, queued_certs) in &query {
        let certs = &queued_certs.certs;

        let session_id = Uuid::new_v4();

        let chat_session = RemoteChatSessionData {
            session_id,
            profile_public_key: ProfilePublicKeyData {
                expires_at: certs.expires_at.timestamp_millis() as u64,
                key: certs.public_key_der.clone(),
                key_signature: certs.signature_v2.clone(),
            },
        };

        debug!("Sending chat signing certs to server");

        commands.trigger(SendGamePacketEvent::new(
            entity,
            ServerboundChatSessionUpdate { chat_session },
        ));
        commands
            .entity(entity)
            .remove::<QueuedCertsToSend>()
            .insert(ChatSigningSession {
                session_id,
                messages_sent: 0,
            });
    }
}

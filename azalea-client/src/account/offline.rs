use uuid::Uuid;

use crate::account::{Account, AccountTrait};

/// A type of account that does not perform any authentication and cannot join
/// online-mode servers.
///
/// This type is not intended to be used directly by the user. To actually make
/// an offline-mode account, see [`Account::offline`].
#[derive(Debug)]
pub struct OfflineAccount {
    username: String,
}
impl AccountTrait for OfflineAccount {
    fn username(&self) -> &str {
        &self.username
    }
    fn uuid(&self) -> Uuid {
        azalea_crypto::offline::generate_uuid(&self.username)
    }
    fn access_token(&self) -> Option<String> {
        None
    }
}

impl Account {
    /// An offline account does not authenticate with Microsoft's servers, and
    /// as such can only join offline mode servers.
    ///
    /// This is useful for testing in LAN worlds.
    pub fn offline(username: &str) -> Self {
        OfflineAccount {
            username: username.to_owned(),
        }
        .into()
    }
}

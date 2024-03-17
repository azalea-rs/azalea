use std::sync::Arc;

use async_trait::async_trait;
use bevy_ecs::component::Component;
use uuid::Uuid;

use crate::{
    certs::{Certificates, FetchCertificatesError},
    sessionserver::ClientSessionServerError,
};

#[async_trait]
pub trait Account: std::fmt::Debug + Send + Sync + 'static {
    async fn join(
        &self,
        public_key: &[u8],
        private_key: &[u8],
        server_id: &str,
    ) -> Result<(), ClientSessionServerError> {
        let server_hash = azalea_crypto::hex_digest(&azalea_crypto::digest_data(
            server_id.as_bytes(),
            public_key,
            private_key,
        ));
        let uuid = self.get_uuid();

        self.join_with_server_id_hash(uuid, server_hash).await
    }
    async fn join_with_server_id_hash(
        &self,
        uuid: Uuid,
        server_hash: String,
    ) -> Result<(), ClientSessionServerError>;

    async fn fetch_certificates(&self) -> Result<Certificates, FetchCertificatesError>;

    fn get_username(&self) -> String;
    fn get_uuid(&self) -> Uuid;

    fn is_online(&self) -> bool {
        true
    }
}

#[derive(Clone, Debug, Component)]
pub struct BoxedAccount(pub Arc<dyn Account>);

#[async_trait]
impl Account for BoxedAccount {
    async fn join_with_server_id_hash(
        &self,
        uuid: Uuid,
        server_hash: String,
    ) -> Result<(), ClientSessionServerError> {
        self.0.join_with_server_id_hash(uuid, server_hash).await
    }

    async fn fetch_certificates(&self) -> Result<Certificates, FetchCertificatesError> {
        self.0.fetch_certificates().await
    }

    fn get_username(&self) -> String {
        self.0.get_username()
    }

    fn get_uuid(&self) -> Uuid {
        self.0.get_uuid()
    }

    fn is_online(&self) -> bool {
        self.0.is_online()
    }
}

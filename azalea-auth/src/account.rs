use std::future::Future;

use bevy_ecs::component::Component;
use uuid::Uuid;

use crate::{certs::{Certificates, FetchCertificatesError}, sessionserver::ClientSessionServerError};

pub trait Account: Clone + Component {
    fn join(
        &self,
        public_key: &[u8],
        private_key: &[u8],
        server_id: &str,
    ) -> impl Future<Output = Result<(), ClientSessionServerError>> + Send {    
        let server_hash = azalea_crypto::hex_digest(&azalea_crypto::digest_data(
            server_id.as_bytes(),
            public_key,
            private_key,
        ));
        let uuid = self.get_uuid();
    
        self.join_with_server_id_hash(uuid, server_hash)
    }
    fn join_with_server_id_hash(&self, uuid: Uuid, server_hash: String) -> impl Future<Output = Result<(), ClientSessionServerError>> + Send;

    fn fetch_certificates(&self) -> impl Future<Output = Result<Certificates, FetchCertificatesError>> + Send;

    fn get_username(&self) -> String;
    fn get_uuid(&self) -> Uuid;

    fn is_online(&self) -> bool {
        true
    }
}
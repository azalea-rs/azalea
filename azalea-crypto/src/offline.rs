//! Offline-mode UUID generation.

use md5::{Digest, Md5};
use uuid::Uuid;

/// Return what the offline-mode UUIDv3 for the given username would be.
pub fn generate_uuid(username: &str) -> Uuid {
    uuid::Builder::from_md5_bytes(hash(format!("OfflinePlayer:{username}").as_bytes())).into_uuid()
}

fn hash(data: &[u8]) -> [u8; 16] {
    let mut hasher = Md5::new();

    hasher.update(data);

    let mut bytes = [0; 16];
    bytes.copy_from_slice(&hasher.finalize()[..16]);

    bytes
}

//! Cache auth information

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, Error)]
pub enum CacheError {
    #[error("Failed to read cache file")]
    ReadError(std::io::Error),
    #[error("Failed to write cache file")]
    WriteError(std::io::Error),
    #[error("Failed to parse cache file")]
    ParseError(serde_json::Error),
}

#[derive(Deserialize, Serialize)]
pub struct CachedAccount {
    pub email: String,
    /// Microsoft auth
    pub msa: ExpiringValue<crate::auth::AccessTokenResponse>,
    /// Xbox Live auth
    pub xbl: ExpiringValue<crate::auth::XboxLiveAuth>,
    /// Minecraft auth
    pub mca: ExpiringValue<crate::auth::MinecraftAuthResponse>,
    /// The user's Minecraft profile (i.e. username, UUID, skin)
    pub profile: crate::auth::ProfileResponse,
}

#[derive(Deserialize, Serialize)]
pub struct ExpiringValue<T> {
    /// Seconds since the UNIX epoch
    pub expires_at: u64,
    pub data: T,
}

impl<T> ExpiringValue<T> {
    pub fn is_expired(&self) -> bool {
        self.expires_at
            < SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64
    }

    /// Return the data if it's not expired, otherwise return `None`
    pub fn get(&self) -> Option<&T> {
        if self.is_expired() {
            None
        } else {
            Some(&self.data)
        }
    }
}

async fn get_entire_cache(cache_file: &Path) -> Result<Vec<CachedAccount>, CacheError> {
    let mut cache: Vec<CachedAccount> = Vec::new();
    if cache_file.exists() {
        let cache_file = File::open(cache_file)
            .await
            .map_err(CacheError::ReadError)?;
        // read the file into a string
        let mut cache_file = tokio::io::BufReader::new(cache_file);
        let mut contents = String::new();
        cache_file
            .read_to_string(&mut contents)
            .await
            .map_err(CacheError::ReadError)?;
        cache = serde_json::from_str(&contents).map_err(CacheError::ParseError)?;
    }
    Ok(cache)
}
async fn set_entire_cache(cache_file: &Path, cache: Vec<CachedAccount>) -> Result<(), CacheError> {
    let cache_file = File::create(cache_file)
        .await
        .map_err(CacheError::WriteError)?;
    let mut cache_file = tokio::io::BufWriter::new(cache_file);
    let cache = serde_json::to_string(&cache).map_err(CacheError::ParseError)?;
    cache_file
        .write_all(cache.as_bytes())
        .await
        .map_err(CacheError::WriteError)?;

    Ok(())
}

/// Gets cached data for the given email.
///
/// Technically it doesn't actually have to be an email since it's only the
/// cache key. I considered using usernames or UUIDs as the cache key, but
/// usernames change and no one has their UUID memorized.
pub async fn get_account_in_cache(cache_file: &Path, email: &str) -> Option<CachedAccount> {
    let cache = get_entire_cache(cache_file).await.unwrap_or_default();
    cache.into_iter().find(|account| account.email == email)
}

pub async fn set_account_in_cache(
    cache_file: &Path,
    email: &str,
    account: CachedAccount,
) -> Result<(), CacheError> {
    let mut cache = get_entire_cache(cache_file).await.unwrap_or_default();
    cache.retain(|account| account.email != email);
    cache.push(account);
    set_entire_cache(cache_file, cache).await
}

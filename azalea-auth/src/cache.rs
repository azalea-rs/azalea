//! Cache auth information

use std::{
    io,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::{
    fs::{self, File},
    io::{AsyncReadExt, AsyncWriteExt},
};
use tracing::{debug, trace};

#[derive(Debug, Error)]
pub enum CacheError {
    #[error("Failed to read cache file: {0}")]
    Read(io::Error),
    #[error("Failed to write cache file: {0}")]
    Write(io::Error),
    #[error("Failed to create cache file directory: {0}")]
    MkDir(io::Error),
    #[error("Failed to parse cache file: {0}")]
    Parse(serde_json::Error),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CachedAccount {
    pub cache_key: String,
    /// Microsoft auth
    pub msa: ExpiringValue<crate::auth::AccessTokenResponse>,
    /// Xbox Live auth
    pub xbl: ExpiringValue<crate::auth::XboxLiveAuth>,
    /// Minecraft auth
    pub mca: ExpiringValue<crate::auth::MinecraftAuthResponse>,
    /// The user's Minecraft profile (i.e. username, UUID, skin)
    pub profile: crate::auth::ProfileResponse,
}

#[derive(Deserialize, Serialize, Debug)]
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
                .as_secs()
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

impl<T: Clone> Clone for ExpiringValue<T> {
    fn clone(&self) -> Self {
        Self {
            expires_at: self.expires_at,
            data: self.data.clone(),
        }
    }
}

async fn get_entire_cache(cache_file: &Path) -> Result<Vec<CachedAccount>, CacheError> {
    let mut cache: Vec<CachedAccount> = Vec::new();
    if cache_file.exists() {
        let mut cache_file = File::open(cache_file).await.map_err(CacheError::Read)?;
        // read the file into a string
        let mut contents = String::new();
        cache_file
            .read_to_string(&mut contents)
            .await
            .map_err(CacheError::Read)?;
        cache = serde_json::from_str(&contents).map_err(CacheError::Parse)?;
    }
    Ok(cache)
}
async fn set_entire_cache(cache_file: &Path, cache: Vec<CachedAccount>) -> Result<(), CacheError> {
    trace!("saving cache: {:?}", cache);

    if !cache_file.exists() {
        let cache_file_parent = cache_file
            .parent()
            .expect("Cache file is root directory and also doesn't exist.");
        debug!(
            "Making cache file parent directory at {}",
            cache_file_parent.to_string_lossy()
        );
        fs::create_dir_all(cache_file_parent)
            .await
            .map_err(CacheError::MkDir)?;
    }
    let mut cache_file = File::create(cache_file).await.map_err(CacheError::Write)?;
    let cache = serde_json::to_string_pretty(&cache).map_err(CacheError::Parse)?;
    cache_file
        .write_all(cache.as_bytes())
        .await
        .map_err(CacheError::Write)?;

    Ok(())
}

/// Gets cached data for the given cache key.
///
/// As a convention, the cache key is usually the email of the account.
pub async fn get_account_in_cache(cache_file: &Path, cache_key: &str) -> Option<CachedAccount> {
    let cache = get_entire_cache(cache_file).await.unwrap_or_default();
    cache
        .into_iter()
        .find(|account| account.cache_key == cache_key)
}

pub async fn set_account_in_cache(
    cache_file: &Path,
    cache_key: &str,
    account: CachedAccount,
) -> Result<(), CacheError> {
    let mut cache = get_entire_cache(cache_file).await.unwrap_or_default();
    cache.retain(|account| account.cache_key != cache_key);
    cache.push(account);
    set_entire_cache(cache_file, cache).await
}

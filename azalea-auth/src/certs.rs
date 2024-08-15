use base64::Engine;
use chrono::{DateTime, Utc};
use rsa::{pkcs8::DecodePrivateKey, RsaPrivateKey};
use serde::Deserialize;
use thiserror::Error;
use tracing::trace;

#[derive(Debug, Error)]
pub enum FetchCertificatesError {
    #[error("Http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Couldn't parse pkcs8 private key: {0}")]
    Pkcs8(#[from] rsa::pkcs8::Error),
}

/// Fetch the Mojang-provided key-pair for your player, which is used for
/// cryptographically signing chat messages.
pub async fn fetch_certificates(
    minecraft_access_token: &str,
) -> Result<Certificates, FetchCertificatesError> {
    let client = reqwest::Client::new();

    let res = client
        .post("https://api.minecraftservices.com/player/certificates")
        .header("Authorization", format!("Bearer {minecraft_access_token}"))
        .send()
        .await?
        .json::<CertificatesResponse>()
        .await?;
    trace!("{:?}", res);

    // using RsaPrivateKey::from_pkcs8_pem gives an error with decoding base64 so we
    // just decode it ourselves

    // remove the first and last lines of the private key
    let private_key_pem_base64 = res
        .key_pair
        .private_key
        .lines()
        .skip(1)
        .take_while(|line| !line.starts_with('-'))
        .collect::<String>();
    let private_key_der = base64::engine::general_purpose::STANDARD
        .decode(private_key_pem_base64)
        .unwrap();

    let public_key_pem_base64 = res
        .key_pair
        .public_key
        .lines()
        .skip(1)
        .take_while(|line| !line.starts_with('-'))
        .collect::<String>();
    let public_key_der = base64::engine::general_purpose::STANDARD
        .decode(public_key_pem_base64)
        .unwrap();

    // the private key also contains the public key so it's basically a keypair
    let private_key = RsaPrivateKey::from_pkcs8_der(&private_key_der).unwrap();

    let certificates = Certificates {
        private_key,
        public_key_der,

        signature_v1: base64::engine::general_purpose::STANDARD
            .decode(&res.public_key_signature)
            .unwrap(),
        signature_v2: base64::engine::general_purpose::STANDARD
            .decode(&res.public_key_signature_v2)
            .unwrap(),

        expires_at: res.expires_at,
        refresh_after: res.refreshed_after,
    };

    Ok(certificates)
}

/// A chat signing certificate.
#[derive(Clone, Debug)]
pub struct Certificates {
    /// The RSA private key.
    pub private_key: RsaPrivateKey,
    /// The RSA public key encoded as DER.
    pub public_key_der: Vec<u8>,

    pub signature_v1: Vec<u8>,
    pub signature_v2: Vec<u8>,

    pub expires_at: DateTime<Utc>,
    pub refresh_after: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CertificatesResponse {
    #[serde(rename = "keyPair")]
    pub key_pair: KeyPairResponse,

    /// base64 string; signed data
    #[serde(rename = "publicKeySignature")]
    pub public_key_signature: String,

    /// base64 string; signed data
    #[serde(rename = "publicKeySignatureV2")]
    pub public_key_signature_v2: String,

    /// Date like `2022-04-30T00:11:32.174783069Z`
    #[serde(rename = "expiresAt")]
    pub expires_at: DateTime<Utc>,

    /// Date like `2022-04-29T16:11:32.174783069Z`
    #[serde(rename = "refreshedAfter")]
    pub refreshed_after: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct KeyPairResponse {
    /// -----BEGIN RSA PRIVATE KEY-----
    /// ...
    /// -----END RSA PRIVATE KEY-----
    #[serde(rename = "privateKey")]
    pub private_key: String,

    /// -----BEGIN RSA PUBLIC KEY-----
    /// ...
    /// -----END RSA PUBLIC KEY-----
    #[serde(rename = "publicKey")]
    pub public_key: String,
}

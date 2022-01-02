use rand::{rngs::OsRng, RngCore};
use sha1::{Digest, Sha1};

fn generate_secret_key() -> [u8; 16] {
    let mut key = [0u8; 16];
    OsRng.fill_bytes(&mut key);
    key
}

fn digest_data(server_id: &[u8], public_key: &[u8], private_key: &[u8]) -> Vec<u8> {
    let mut digest = Sha1::new();
    digest.update(server_id);
    digest.update(public_key);
    digest.update(private_key);
    digest.finalize().to_vec()
}

fn hex_digest(digest: &[u8]) -> String {
    // Note that the Sha1.hexdigest() method used by minecraft is non standard.
    // It doesn't match the digest method found in most programming languages
    // and libraries. It works by treating the sha1 output bytes as one large
    // integer in two's complement and then printing the integer in base 16,
    // placing a minus sign if the interpreted number is negative.
    num_bigint::BigInt::from_signed_bytes_be(digest).to_str_radix(16)
}

fn encrypt(public_key: &[u8], server_id: String, nonce: &[u8]) {
    let secret_key = generate_secret_key();
    let hash = hex_digest(&digest_data(server_id.as_bytes(), public_key, &secret_key));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_secret_key() {
        let key = generate_secret_key();
        assert_eq!(key.len(), 16);
    }

    #[test]
    fn test_hex_digest() {
        let digest = hex_digest(&digest_data(b"Notch", &[], &[]));
        assert_eq!(digest, "4ed1f46bbe04bc756bcb17c0c7ce3e4632f06a48");

        let digest = hex_digest(&digest_data(b"jeb_", &[], &[]));
        assert_eq!(digest, "-7c9d5b0044c130109a5d7b5fb5c317c02b4e28c1");

        let digest = hex_digest(&digest_data(b"simon", &[], &[]));
        assert_eq!(digest, "88e16a1019277b15d58faf0541e11910eb756f6");
    }
}

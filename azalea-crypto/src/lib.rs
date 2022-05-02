use aes::cipher::inout::InOutBuf;
use aes::cipher::BlockEncrypt;
use aes::{
    cipher::{
        generic_array::GenericArray, AsyncStreamCipher, BlockDecryptMut, BlockEncryptMut, KeyIvInit,
    },
    Aes128,
};
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

#[derive(Debug)]
pub struct EncryptResult {
    pub secret_key: [u8; 16],
    pub encrypted_public_key: Vec<u8>,
    pub encrypted_nonce: Vec<u8>,
}

pub fn encrypt(public_key: &[u8], nonce: &[u8]) -> Result<EncryptResult, String> {
    // On receipt of a Encryption Request from the server, the client will
    // generate a random 16-byte shared secret, to be used with the AES/CFB8
    // stream cipher.
    let secret_key = generate_secret_key();
    // let hash = hex_digest(&digest_data(server_id.as_bytes(), public_key, &secret_key));

    // this.keybytes = Crypt.encryptUsingKey(publicKey, secretKey.getEncoded());
    // this.nonce = Crypt.encryptUsingKey(publicKey, arrby);
    let encrypted_public_key: Vec<u8> =
        rsa_public_encrypt_pkcs1::encrypt(&public_key, &secret_key)?;
    let encrypted_nonce: Vec<u8> = rsa_public_encrypt_pkcs1::encrypt(&public_key, &nonce)?;

    Ok(EncryptResult {
        secret_key,
        encrypted_public_key,
        encrypted_nonce,
    })
}

pub type Aes128CfbEnc = cfb8::Encryptor<Aes128>;
pub type Aes128CfbDec = cfb8::Decryptor<Aes128>;

pub fn create_cipher(key: &[u8]) -> (Aes128CfbEnc, Aes128CfbDec) {
    (
        Aes128CfbEnc::new_from_slices(key, key).unwrap(),
        Aes128CfbDec::new_from_slices(key, key).unwrap(),
    )
}

// wow this is terrible
pub fn encrypt_packet(cipher: &mut Aes128CfbEnc, packet: &mut [u8]) {
    let (chunks, rest) = InOutBuf::from(packet).into_chunks();
    assert!(rest.is_empty());
    cipher.encrypt_blocks_inout_mut(chunks);
}
pub fn decrypt_packet(cipher: &mut Aes128CfbDec, packet: &mut [u8]) {
    let (chunks, rest) = InOutBuf::from(packet).into_chunks();
    assert!(rest.is_empty());
    cipher.decrypt_blocks_inout_mut(chunks);
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

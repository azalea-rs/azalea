mod signing;

use aes::cipher::inout::InOutBuf;
use aes::{
    cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit},
    Aes128,
};
use rand::{rngs::OsRng, RngCore};
use sha1::{Digest, Sha1};
pub use signing::*;

fn generate_secret_key() -> [u8; 16] {
    let mut key = [0u8; 16];
    OsRng.fill_bytes(&mut key);
    key
}

pub fn digest_data(server_id: &[u8], public_key: &[u8], private_key: &[u8]) -> Vec<u8> {
    let mut digest = Sha1::new();
    digest.update(server_id);
    digest.update(private_key);
    digest.update(public_key);
    digest.finalize().to_vec()
}

pub fn hex_digest(digest: &[u8]) -> String {
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
    let encrypted_public_key: Vec<u8> = rsa_public_encrypt_pkcs1::encrypt(public_key, &secret_key)?;
    let encrypted_nonce: Vec<u8> = rsa_public_encrypt_pkcs1::encrypt(public_key, nonce)?;

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

    #[test]
    fn encode_packet_twice() {
        let mut packet = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09];
        let (mut enc_cipher, _dec_cipher) = create_cipher(b"1234567890123456");
        encrypt_packet(&mut enc_cipher, &mut packet);
        assert_eq!(packet, vec![117, 151, 183, 45, 229, 232, 43, 181, 121, 16]);
        let mut packet = vec![0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13];
        encrypt_packet(&mut enc_cipher, &mut packet);
        assert_eq!(
            packet,
            vec![185, 223, 129, 153, 173, 140, 133, 239, 59, 168]
        );
    }

    #[test]
    fn encode_packet_long() {
        let mut packet = (0..=255).collect::<Vec<u8>>();
        let (mut enc_cipher, _dec_cipher) = create_cipher(b"1234567890123456");
        encrypt_packet(&mut enc_cipher, &mut packet);
        assert_eq!(
            packet,
            vec![
                117, 151, 183, 45, 229, 232, 43, 181, 121, 16, 185, 223, 129, 153, 173, 140, 133,
                239, 59, 168, 148, 39, 97, 19, 22, 219, 78, 70, 116, 143, 21, 223, 155, 99, 201,
                62, 133, 77, 244, 152, 254, 36, 135, 147, 45, 25, 66, 236, 2, 12, 101, 39, 140, 62,
                7, 57, 19, 101, 217, 91, 142, 243, 0, 3, 100, 142, 160, 21, 219, 145, 151, 37, 11,
                30, 190, 176, 26, 90, 143, 63, 255, 188, 254, 40, 41, 92, 163, 197, 8, 8, 111, 175,
                49, 234, 82, 34, 5, 96, 162, 7, 217, 42, 77, 38, 127, 213, 207, 251, 34, 173, 34,
                132, 23, 12, 118, 59, 51, 216, 173, 199, 137, 95, 132, 222, 243, 195, 81, 60, 205,
                52, 65, 209, 125, 137, 5, 52, 219, 165, 248, 35, 173, 57, 200, 182, 162, 148, 70,
                62, 102, 21, 220, 158, 71, 98, 47, 231, 196, 58, 8, 70, 160, 177, 159, 50, 20, 187,
                31, 249, 68, 26, 142, 171, 239, 193, 10, 174, 14, 80, 238, 114, 124, 185, 253, 246,
                47, 67, 37, 69, 70, 9, 69, 135, 13, 195, 253, 8, 241, 175, 170, 75, 231, 7, 92, 18,
                38, 132, 65, 146, 202, 130, 238, 224, 30, 113, 168, 241, 159, 131, 238, 67, 1, 244,
                74, 172, 86, 95, 192, 236, 198, 188, 81, 67, 49, 230, 166, 52, 224, 238, 11, 252,
                0, 179, 56, 209, 231, 62, 146, 106, 18, 217, 138, 89, 110, 240, 255, 192
            ]
        );
    }

    #[test]
    fn encode_decode_packet() {
        let mut packet = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09];
        let (mut enc_cipher, mut dec_cipher) = create_cipher(b"1234567890123456");
        encrypt_packet(&mut enc_cipher, &mut packet);
        decrypt_packet(&mut dec_cipher, &mut packet);
        assert_eq!(
            packet,
            vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
        );
    }
}

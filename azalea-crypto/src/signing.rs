#[derive(Debug, Clone)]
pub struct SaltSignaturePair {
    pub salt: u64,
    pub signature: Vec<u8>,
}

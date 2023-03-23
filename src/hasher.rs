use tiny_keccak;
use tiny_keccak::Hasher as H;

pub trait Hasher {
    const LENGTH: usize;

    fn digest(&self, data: &[u8]) -> [u8; 32];
}

pub struct FixedHasherKeccak {}

impl FixedHasherKeccak {
    pub fn new() -> Self {
        Self {}
    }
}

impl Hasher for FixedHasherKeccak {
    const LENGTH: usize = 32;

    fn digest(&self, data: &[u8]) -> [u8; 32] {
        let mut hasher = tiny_keccak::Keccak::v256();
        let mut result: [u8; 32] = [0; 32];
        hasher.update(data);
        hasher.finalize(&mut result);
        result
    }
}

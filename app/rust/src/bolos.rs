//! Rust interfaces to Ledger SDK APIs.
use rand::{CryptoRng, RngCore};

extern "C" {
    fn cx_rng(buffer: *mut u8, len: u32);
}

#[cfg(test)]
fn external_rng(dest: &mut [u8]) {
    // TODO:
}

#[cfg(not(test))]
fn external_rng(dest: &mut [u8]) {
    // TODO:
}

pub struct Trng;

impl RngCore for Trng {
    fn next_u32(&mut self) -> u32 {
        let mut out = [0; 4];
        self.fill_bytes(&mut out);
        u32::from_le_bytes(out)
    }

    fn next_u64(&mut self) -> u64 {
        let mut out = [0; 8];
        self.fill_bytes(&mut out);
        u64::from_le_bytes(out)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        external_rng(dest);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        Ok(self.fill_bytes(dest))
    }
}

impl CryptoRng for Trng {}

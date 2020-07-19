#![no_std]
#![no_builtins]
#![allow(dead_code, unused_imports)]

#[cfg(test)]
#[macro_use]
extern crate hex_literal;

mod bolos;

extern crate core;

fn debug(_msg: &str) {}

use core::convert::TryInto;
use core::mem;
#[cfg(not(test))]
use core::panic::PanicInfo;
use schnorrkel::{PublicKey, SecretKey};

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// #[no_mangle]
// pub extern "C" fn get_sr25519_pk(sk_ed25519_expanded_ptr: *const u8, pk_ptr: *mut u8) {
//     let pkd: &mut [u8; 32] = unsafe { mem::transmute::<*const u8, &mut [u8; 32]>(pk_ptr) };
//     let sk_ed25519_expanded: &[u8; 64] = unsafe { mem::transmute::<*const u8, &[u8; 64]>(sk_ed25519_expanded_ptr) };
//
//     let secret: SecretKey = SecretKey::from_ed25519_bytes(&sk_ed25519_expanded[..]).unwrap();
//
//     // 192 bytes
//     let public: PublicKey = secret.to_public();
//     pkd.copy_from_slice(&public.to_bytes())
// }

#[no_mangle]
pub extern "C" fn get_sr25519_pk(sk_ed25519_expanded_ptr: *const u8, pk_ptr: *mut u8) {
    let sk_ed25519_expanded: &[u8; 64] =
        unsafe { mem::transmute::<*const u8, &[u8; 64]>(sk_ed25519_expanded_ptr) };

    let secret: SecretKey = SecretKey::from_ed25519_bytes(&sk_ed25519_expanded[..]).unwrap();

    // 192 bytes
    let public: PublicKey = secret.to_public();

    let pkd: &mut [u8; 32] = unsafe { mem::transmute::<*const u8, &mut [u8; 32]>(pk_ptr) };
    pkd.copy_from_slice(&public.to_bytes())
}

#[cfg(test)]
mod tests {
    use crate::*;
    use schnorrkel::{Keypair, PublicKey, SecretKey, Signature};

    use log::debug;

    fn init_logging() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn get_public_key_c() {
        init_logging();

        let mut sk_ed25519_expanded = [0u8; 64];
        let mut pk = [0u8; 32];

        get_sr25519_pk(sk_ed25519_expanded.as_ptr(), pk.as_mut_ptr());

        debug!("{:?}", pk);
    }
}

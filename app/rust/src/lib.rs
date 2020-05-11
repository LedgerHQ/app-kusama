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
use core::mem;
use core::convert::TryInto;
use crate::bolos::Trng;
use schnorrkel::{SecretKey, PublicKey};

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn get_sr25519_pk(pk_ptr: *mut u8) {
    let pkd: &mut [u8; 32] = unsafe { mem::transmute::<*const u8, &mut [u8; 32]>(pk_ptr) };

    let trng = Trng;
    let secret: SecretKey = SecretKey::generate_with(trng);
    let public: PublicKey = secret.to_public();

    pkd.copy_from_slice(&public.to_bytes())
}

#[cfg(test)]
mod tests {
    use crate::*;
    use crate::bolos::Trng;
    use schnorrkel::{Keypair, Signature, SecretKey, PublicKey};

    use log::debug;

    fn init_logging() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn get_public_key() {
        init_logging();

        let trng = Trng;
        let secret: SecretKey = SecretKey::generate_with(trng);
        let public: PublicKey = secret.to_public();

        debug!("Signing test");
        debug!("{:?}", secret);
        debug!("{:?}", public);

        assert!(public == public)
    }

    #[test]
    fn get_public_key_c() {
        init_logging();

        let mut pk = [0u8; 32];

        get_sr25519_pk(pk.as_mut_ptr());

        debug!("{:?}", pk);
    }
}

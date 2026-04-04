use wolfssl_sys::{
    ed25519_export_private_key, ed25519_export_public_key, ed25519_free, ed25519_generate_key,
    ed25519_import_key, ed25519_import_public, ed25519_key, ed25519_key_free, ed25519_key_init,
    ed25519_new, ed25519_sign_msg, ed25519_verify_msg,
};

use crate::wolfcrypt;
use crate::wolfcrypt::error::*;
use crate::wolfcrypt::rng;

const ED25519_KEY_SIZE: usize = wolfssl_sys::ED25519_KEY_SIZE as usize;
const ED25519_PUB_KEY_SIZE: usize = wolfssl_sys::ED25519_PUB_KEY_SIZE as usize;
const ED25519_SIG_SIZE: usize = wolfssl_sys::ED25519_SIG_SIZE as usize;

#[derive(Debug)]
pub struct Ed25519 {
    key: *mut ed25519_key,
    rng: rng::Rng,
}

impl wolfcrypt::Sign for Ed25519 {
    fn new() -> Self {
        let key = unsafe { ed25519_new() };

        if key.is_null() {
            panic!("libc memory allocation error")
        }

        let ret = unsafe { ed25519_key_init(key) };

        let rng = rng::Rng::new();

        match ret {
            0 => Self { key, rng },
            _ => panic!("key initialize error"),
        }
    }

    fn free(&mut self) -> Result<()> {
        unsafe {
            ed25519_key_free(self.key);
            ed25519_free(self.key);
        }
        self.rng.free()
    }

    fn size_of_key() -> usize {
        ED25519_KEY_SIZE
    }

    fn dyn_size_of_key(&self) -> usize {
        Self::size_of_key()
    }

    fn size_of_pubkey() -> usize {
        ED25519_PUB_KEY_SIZE
    }

    fn dyn_size_of_pubkey(&self) -> usize {
        Self::size_of_pubkey()
    }

    fn size_of_sig() -> usize {
        ED25519_SIG_SIZE
    }
    fn dyn_size_of_sig(&self) -> usize {
        Self::size_of_sig()
    }

    fn as_str() -> &'static str {
        "ed25519"
    }

    fn dyn_as_str(&self) -> &'static str {
        Self::as_str()
    }

    fn import(&mut self, privkey: &mut Vec<u8>, pubkey: &mut Vec<u8>) -> Result<()> {
        if privkey.len() != ED25519_KEY_SIZE {
            return Err(Error::from(Errorkind::KeybufferError));
        }

        if pubkey.len() != ED25519_PUB_KEY_SIZE {
            return Err(Error::from(Errorkind::KeybufferError));
        }

        let ret = unsafe {
            ed25519_import_key(
                privkey.as_ptr(),
                ED25519_KEY_SIZE as u32,
                pubkey.as_ptr(),
                ED25519_PUB_KEY_SIZE as u32,
                self.key,
            )
        };

        match ret {
            0 => Ok(()),
            _ => Err(Error::from_native(ret)),
        }
    }

    fn import_pubkey(&mut self, pubkey: &mut Vec<u8>) -> Result<()> {
        if pubkey.len() != ED25519_PUB_KEY_SIZE {
            return Err(Error::from(Errorkind::KeybufferError));
        }

        let ret = unsafe {
            ed25519_import_public(pubkey.as_ptr(), ED25519_PUB_KEY_SIZE as u32, self.key)
        };

        match ret {
            0 => Ok(()),
            _ => Err(Error::from_native(ret)),
        }
    }

    fn sign(&mut self, msg: &mut Vec<u8>) -> Result<Vec<u8>> {
        let msg_size = msg.len() as u32;
        let mut signature: [u8; ED25519_SIG_SIZE] = [0u8; ED25519_SIG_SIZE];
        let mut out_size: u32 = ED25519_SIG_SIZE as u32;
        let ret = unsafe {
            ed25519_sign_msg(
                msg.as_ptr(),
                msg_size,
                signature.as_mut_ptr(),
                &mut out_size,
                self.key,
            )
        };

        match ret {
            0 => {
                if out_size as usize != ED25519_SIG_SIZE {
                    return Err(Error::from(Errorkind::SigbufferError));
                }
                Ok(signature.to_vec())
            }
            _ => Err(Error::from_native(ret)),
        }
    }

    fn verify(&mut self, signature: &mut Vec<u8>, msg: &mut Vec<u8>) -> Result<i32> {
        if signature.len() != ED25519_SIG_SIZE {
            return Err(Error::from(Errorkind::SigbufferError));
        }

        let msg_size = msg.len() as u32;
        let mut result: i32 = 0;
        let ret = unsafe {
            ed25519_verify_msg(
                signature.as_ptr(),
                ED25519_SIG_SIZE as u32,
                msg.as_ptr(),
                msg_size,
                &mut result,
                self.key,
            )
        };

        match ret {
            0 => Ok(result),
            _ => Err(Error::from_native(ret)),
        }
    }

    fn generate(&mut self) -> Result<()> {
        let ret =
            unsafe { ed25519_generate_key(self.rng.rng_ptr(), ED25519_KEY_SIZE as i32, self.key) };

        match ret {
            0 => Ok(()),
            _ => Err(Error::from_native(ret)),
        }
    }

    fn export_private(&mut self) -> Result<Vec<u8>> {
        let mut privkey: [u8; ED25519_KEY_SIZE] = [0; ED25519_KEY_SIZE];
        let mut out_size: u32 = ED25519_KEY_SIZE as u32;
        let ret =
            unsafe { ed25519_export_private_key(self.key, privkey.as_mut_ptr(), &mut out_size) };

        match ret {
            0 => {
                if out_size as usize != ED25519_KEY_SIZE {
                    return Err(Error::from(Errorkind::KeybufferError));
                }

                Ok(privkey.to_vec())
            }
            _ => Err(Error::from_native(ret)),
        }
    }

    fn export_public(&mut self) -> Result<Vec<u8>> {
        let mut pubkey: [u8; ED25519_PUB_KEY_SIZE] = [0; ED25519_PUB_KEY_SIZE];
        let mut out_size: u32 = ED25519_PUB_KEY_SIZE as u32;
        let ret =
            unsafe { ed25519_export_public_key(self.key, pubkey.as_mut_ptr(), &mut out_size) };

        match ret {
            0 => {
                if out_size as usize != ED25519_PUB_KEY_SIZE {
                    return Err(Error::from(Errorkind::KeybufferError));
                }

                Ok(pubkey.to_vec())
            }
            _ => Err(Error::from_native(ret)),
        }
    }
}

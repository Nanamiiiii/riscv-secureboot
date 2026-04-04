use wolfssl_sys::{
    WC_SHA3_384_DIGEST_SIZE, sha3_384_final, sha3_384_init, sha3_384_update, sha3_ctx_free,
    sha3_ctx_new, wc_Sha3,
};

use crate::wolfcrypt;
use crate::wolfcrypt::error::*;

pub const SHA3_384_DIGEST_SIZE: usize = WC_SHA3_384_DIGEST_SIZE as usize;

#[derive(Debug)]
pub struct Sha3_384 {
    ctx: *mut wc_Sha3,
}

impl wolfcrypt::Hash for Sha3_384 {
    fn new() -> Self {
        let ctx = unsafe { sha3_ctx_new() };
        let ret = unsafe { sha3_384_init(ctx) };
        match ret {
            0 => Self { ctx },
            _ => panic!("Failed to init hash ctx, {}", ret),
        }
    }

    fn size_of_digest() -> usize {
        SHA3_384_DIGEST_SIZE
    }

    fn as_str() -> &'static str {
        "sha3-384"
    }

    fn update(&mut self, data: *const u8, len: i32) -> Result<()> {
        let ret = unsafe { sha3_384_update(self.ctx, data, len) };
        match ret {
            0 => Ok(()),
            _ => Err(Error::from_native(ret)),
        }
    }

    fn finalize(&mut self) -> Result<Vec<u8>> {
        let mut digest: [u8; SHA3_384_DIGEST_SIZE] = [0; SHA3_384_DIGEST_SIZE];
        let ret = unsafe { sha3_384_final(self.ctx, digest.as_mut_ptr()) };
        match ret {
            0 => Ok(digest.to_vec()),
            _ => Err(Error::from_native(ret)),
        }
    }

    fn reset(&mut self) -> Result<()> {
        let ret = unsafe { sha3_384_init(self.ctx) };
        match ret {
            0 => Ok(()),
            _ => Err(Error::from_native(ret)),
        }
    }

    fn free(&mut self) {
        unsafe { sha3_ctx_free(self.ctx) }
    }
}

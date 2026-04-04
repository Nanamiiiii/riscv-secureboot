use wolfssl_sys::{WC_RNG, rng_free, rng_init, rng_new};

use crate::wolfcrypt::error::{Error, Result};

#[derive(Debug)]
pub struct Rng {
    wc_rng: *mut WC_RNG,
}

impl Rng {
    pub fn new() -> Self {
        let rng = unsafe { rng_new() };

        if rng.is_null() {
            panic!("libc memory allocation error")
        }

        let ret = unsafe { rng_init(rng) };

        match ret {
            0 => Self { wc_rng: rng },
            _ => panic!("failed to initialize wolfCrypt RNG, code: {}", ret,),
        }
    }

    pub fn rng_ptr(&self) -> *mut WC_RNG {
        self.wc_rng
    }

    pub fn free(&mut self) -> Result<()> {
        let ret = unsafe { rng_free(self.wc_rng) };

        match ret {
            0 => Ok(()),
            _ => Err(Error::from_native(ret)),
        }
    }
}

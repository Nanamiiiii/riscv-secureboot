use wolfssl::wolfcrypt::{self, error::*};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Signalgo {
    Ed25519,
}

#[allow(dead_code)]
impl Signalgo {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "ed25519" => Some(Self::Ed25519),
            _ => None,
        }
    }

    pub fn to_name(self) -> &'static str {
        match self {
            Self::Ed25519 => "ed25519",
        }
    }

    pub fn list_available() -> Vec<&'static str> {
        vec!["ed25519"]
    }

    pub fn size(self) -> usize {
        match self {
            Self::Ed25519 => 64,
        }
    }
}

#[derive(Debug)]
pub struct Signature<T, U> {
    pub algo: U,
    signature: Vec<u8>,
    keyhash: Keyhash<T>,
}

#[allow(dead_code)]
impl<T: wolfcrypt::Hash, U: wolfcrypt::Sign> Signature<T, U> {
    pub fn new() -> Self {
        Self {
            algo: U::new(),
            signature: Vec::new(),
            keyhash: Keyhash::new(),
        }
    }

    pub fn free(&mut self) {
        self.keyhash.free();
        self.algo.free().unwrap();
    }

    pub fn sign(&mut self, msg: &mut Vec<u8>) -> Result<()> {
        self.signature = self.algo.sign(msg)?;
        Ok(())
    }

    pub fn signature(&self) -> Vec<u8> {
        self.signature.clone()
    }

    pub fn keyhash(&mut self) -> Vec<u8> {
        self.keyhash.get()
    }

    pub fn import_key(&mut self, key: &mut Vec<u8>) -> Result<()> {
        let privkey_size = U::size_of_key();
        let pubkey_size = U::size_of_pubkey();

        if key.len() != privkey_size + pubkey_size {
            return Err(Error::from(Errorkind::KeybufferError));
        }

        self.keyhash.calc(&mut key[privkey_size..].to_vec());

        self.algo.import(
            &mut key[0..privkey_size].to_vec(),
            &mut key[privkey_size..].to_vec(),
        )
    }
}

#[derive(Debug)]
pub struct Keyhash<T> {
    ctx: T,
    digest: Vec<u8>,
}

#[allow(dead_code)]
impl<T> Keyhash<T> {
    pub fn get(&self) -> Vec<u8> {
        self.digest.clone()
    }
}

#[allow(dead_code)]
impl<T: wolfcrypt::Hash> Keyhash<T> {
    pub fn new() -> Self {
        Self {
            ctx: T::new(),
            digest: Vec::new(),
        }
    }

    pub fn free(&mut self) {
        self.ctx.free()
    }

    pub fn calc(&mut self, key: &mut Vec<u8>) {
        let key_size = key.len() as i32;

        match self.ctx.update(key.as_ptr(), key_size) {
            Ok(_) => {}
            Err(err) => panic!("{}", err),
        }

        let digest = match self.ctx.finalize() {
            Ok(d) => d,
            Err(err) => panic!("{}", err),
        };

        self.digest = digest.to_vec();
    }
}

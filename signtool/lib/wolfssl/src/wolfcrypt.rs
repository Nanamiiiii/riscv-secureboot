pub mod ed25519;
pub mod error;
pub mod rng;
pub mod sha3;

pub trait Hash {
    fn new() -> Self;
    fn free(&mut self);
    fn size_of_digest() -> usize;
    fn as_str() -> &'static str;
    fn update(&mut self, data: *const u8, len: i32) -> error::Result<()>;
    fn finalize(&mut self) -> error::Result<Vec<u8>>;
    fn reset(&mut self) -> error::Result<()>;
}

pub trait Sign {
    fn new() -> Self;
    fn free(&mut self) -> error::Result<()>;
    fn size_of_key() -> usize;
    fn dyn_size_of_key(&self) -> usize;
    fn size_of_pubkey() -> usize;
    fn dyn_size_of_pubkey(&self) -> usize;
    fn size_of_sig() -> usize;
    fn dyn_size_of_sig(&self) -> usize;
    fn as_str() -> &'static str;
    fn dyn_as_str(&self) -> &'static str;
    fn import(&mut self, privkey: &mut Vec<u8>, pubkey: &mut Vec<u8>) -> error::Result<()>;
    fn import_pubkey(&mut self, pubkey: &mut Vec<u8>) -> error::Result<()>;
    fn sign(&mut self, msg: &mut Vec<u8>) -> error::Result<Vec<u8>>;
    fn verify(&mut self, signature: &mut Vec<u8>, msg: &mut Vec<u8>) -> error::Result<i32>;
    fn generate(&mut self) -> error::Result<()>;
    fn export_private(&mut self) -> error::Result<Vec<u8>>;
    fn export_public(&mut self) -> error::Result<Vec<u8>>;
}

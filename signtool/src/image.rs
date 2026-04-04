#![allow(dead_code)]

use crate::hash::{Hash, Hashalgo};
use crate::signature::{Signalgo, Signature};

use wolfssl::wolfcrypt;

use std::fs::File;
use std::io::{self, Write};
use std::mem::size_of;
use std::time::SystemTime;
use std::{error, fmt};

use colored::Colorize;

pub const IMAGE_MAGIC: u32 = 0x53454342; // Magic number: SECB

// Header field tags
pub const FIELD_HASH_ALGO: u8 = 0x10;
pub const FIELD_BLOCK_SIZE: u8 = 0x11;
pub const FIELD_DIGEST: u8 = 0x12;
pub const FIELD_KEY_HASH: u8 = 0x20;
pub const FIELD_SIGNATURE_ALGO: u8 = 0x30;
pub const FIELD_SIGNATURE: u8 = 0x31;
pub const FIELD_IMAGE_TYPE: u8 = 0x40;
pub const FIELD_LOAD_ADDRESS: u8 = 0x41;
pub const FIELD_VERSION_CODE: u8 = 0x50;
pub const FIELD_TIMESTAMP: u8 = 0x60;

// Hash algorithms
pub const HASH_SHA3_384: u8 = 0x01;
pub const HASH_SHA_256: u8 = 0x02;

// Signature algorithms
pub const SIGN_ED25519: u8 = 0x01;

pub const HEADER_PADDING: u8 = 0xFF;
pub const HEADER_SIZE: usize = 256;

#[derive(Debug)]
pub struct ImageBuilder<T, U> {
    raw_bytes: Vec<u8>,
    image_size: u32,
    load_addr: Option<usize>,
    img_type: Option<u8>,
    version_code: Option<u32>,
    header: Vec<u8>,
    hash: Hash<T>,
    signature: Signature<T, U>,
}

impl<T: wolfcrypt::Hash, U: wolfcrypt::Sign> ImageBuilder<T, U> {
    pub fn new() -> Self {
        Self {
            raw_bytes: Vec::new(),
            image_size: 0,
            load_addr: None,
            img_type: None,
            version_code: None,
            header: Vec::with_capacity(HEADER_SIZE),
            hash: Hash::new(),
            signature: Signature::new(),
        }
    }

    pub fn base_image(&mut self, data: &mut Vec<u8>) -> &mut Self {
        self.raw_bytes = data.clone();
        self.image_size = data.len() as u32;
        self
    }

    pub fn key(&mut self, key: &mut Vec<u8>) -> &mut Self {
        match self.signature.import_key(key) {
            Ok(_) => self,
            Err(err) => panic!("{}", err),
        }
    }

    pub fn enable_block(&mut self, block: u64) -> &mut Self {
        self.hash.block(block);
        self
    }

    pub fn load_to(&mut self, addr: usize) -> &mut Self {
        self.load_addr = Some(addr);
        self
    }

    pub fn set_type(&mut self, img_type: u8) -> &mut Self {
        self.img_type = Some(img_type);
        self
    }

    pub fn set_version(&mut self, ver: u32) -> &mut Self {
        self.version_code = Some(ver);
        self
    }

    pub fn get_digest(&mut self) -> Vec<u8> {
        self.hash.root_digest()
    }

    pub fn get_keyhash(&mut self) -> Vec<u8> {
        self.signature.keyhash()
    }

    pub fn get_signature(&mut self) -> Vec<u8> {
        self.signature.signature()
    }

    pub fn build(&mut self) -> Result<()> {
        // Generate image header before digest
        // All integer field are aligned as little endian (except magic)

        // Magic number
        for v in IMAGE_MAGIC.to_be_bytes() {
            self.header.push(v)
        }

        // Image size
        for v in self.image_size.to_le_bytes() {
            self.header.push(v)
        }

        if self.header.len() != size_of::<u32>() * 2 {
            return Err(Error::from(Errorkind::ImageCreationError));
        }

        // Block size
        if let Some(size) = self.hash.get_block() {
            self.header.push(FIELD_BLOCK_SIZE);
            self.header.push(HEADER_PADDING);
            self.header.push(HEADER_PADDING);
            self.header.push(HEADER_PADDING);
            for v in size.to_le_bytes() {
                self.header.push(v);
            }
        }

        // Hash algo
        self.header.push(FIELD_HASH_ALGO);
        self.header.push(HEADER_PADDING);
        self.header.push(Self::id_hash_algo());
        self.header.push(HEADER_PADDING);

        // Signature algo
        self.header.push(FIELD_SIGNATURE_ALGO);
        self.header.push(HEADER_PADDING);
        self.header.push(Self::id_sign_algo());
        self.header.push(HEADER_PADDING);

        // Image type
        if let Some(t) = self.img_type {
            self.header.push(FIELD_IMAGE_TYPE);
            self.header.push(HEADER_PADDING);
            self.header.push(t);
            self.header.push(HEADER_PADDING);
        }

        // Load address
        if let Some(addr) = self.load_addr {
            self.header.push(FIELD_LOAD_ADDRESS);
            self.header.push(HEADER_PADDING);
            self.header.push(HEADER_PADDING);
            self.header.push(HEADER_PADDING);
            for v in addr.to_le_bytes() {
                self.header.push(v);
            }
        }

        // Version code
        if let Some(ver) = self.version_code {
            self.header.push(FIELD_VERSION_CODE);
            self.header.push(HEADER_PADDING);
            self.header.push(HEADER_PADDING);
            self.header.push(HEADER_PADDING);
            for v in ver.to_le_bytes() {
                self.header.push(v);
            }
        }

        // Timestamp
        let timestamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(err) => panic!("{}", err),
        };
        self.header.push(FIELD_TIMESTAMP);
        self.header.push(HEADER_PADDING);
        self.header.push(HEADER_PADDING);
        self.header.push(HEADER_PADDING);
        for v in timestamp.to_le_bytes() {
            self.header.push(v);
        }

        self.digest().sign();
        Ok(())
    }

    fn digest(&mut self) -> &mut Self {
        self.hash.calc(&mut self.raw_bytes, Some(&mut self.header));
        self
    }

    fn sign(&mut self) -> &mut Self {
        if let Err(err) = self.signature.sign(&mut self.hash.root_digest()) {
            panic!("Failed to sign image, {}", err)
        }
        self
    }

    fn id_hash_algo() -> u8 {
        match T::as_str() {
            "sha3-384" => HASH_SHA3_384,
            "sha-256" => HASH_SHA_256,
            _ => unimplemented!(),
        }
    }

    fn id_sign_algo() -> u8 {
        match U::as_str() {
            "ed25519" => SIGN_ED25519,
            _ => unimplemented!(),
        }
    }

    pub fn create(&mut self, out_file: String) -> Result<()> {
        let mut secb_image: Vec<u8> = Vec::with_capacity(HEADER_SIZE + self.image_size as usize);

        // Generate image header
        // All integer field are aligned as little endian (except magic)
        secb_image.extend(&self.header);

        // Digest
        secb_image.push(FIELD_DIGEST);
        secb_image.push(HEADER_PADDING);
        let root = self.hash.root_digest();
        if root.len() != T::size_of_digest() {
            return Err(Error::from(Errorkind::ImageCreationError));
        }
        for v in (root.len() as u32).to_le_bytes() {
            secb_image.push(v);
        }
        for v in root.iter() {
            secb_image.push(*v)
        }
        secb_image.push(HEADER_PADDING);
        secb_image.push(HEADER_PADDING);

        // Key hash
        secb_image.push(FIELD_KEY_HASH);
        secb_image.push(HEADER_PADDING);
        let keyhash = self.signature.keyhash();
        if keyhash.len() != T::size_of_digest() {
            return Err(Error::from(Errorkind::ImageCreationError));
        }
        for v in (keyhash.len() as u32).to_le_bytes() {
            secb_image.push(v);
        }
        for v in keyhash.iter() {
            secb_image.push(*v);
        }
        secb_image.push(HEADER_PADDING);
        secb_image.push(HEADER_PADDING);

        // Signature
        secb_image.push(FIELD_SIGNATURE);
        secb_image.push(HEADER_PADDING);
        let sig = self.signature.signature();
        if sig.len() != U::size_of_sig() {
            return Err(Error::from(Errorkind::ImageCreationError));
        }
        for v in (sig.len() as u32).to_le_bytes() {
            secb_image.push(v);
        }
        for v in sig.iter() {
            secb_image.push(*v);
        }

        if secb_image.len() > HEADER_SIZE {
            return Err(Error::from(Errorkind::ImageCreationError));
        }

        let pad_size = HEADER_SIZE - secb_image.len();
        for _ in 0..pad_size {
            secb_image.push(HEADER_PADDING);
        }

        // append raw image
        secb_image.extend(&self.raw_bytes);
        if secb_image.len() != HEADER_SIZE + self.image_size as usize {
            return Err(Error::from(Errorkind::ImageCreationError));
        }

        // Write to file
        let mut file = File::create(out_file)?;
        file.write_all(&secb_image)?;
        file.flush()?;

        Ok(())
    }

    pub fn clean(&mut self) {
        self.hash.free();
        self.signature.free();
    }
}

#[derive(Debug, Default)]
pub struct Image {
    pub blob: Option<Vec<u8>>,
    pub hash_algo: Option<Hashalgo>,
    pub block_size: Option<u64>,
    pub digest: Option<Vec<u8>>,
    pub key_hash: Option<Vec<u8>>,
    pub sign_algo: Option<Signalgo>,
    pub signature: Option<Vec<u8>>,
    pub image_type: Option<u8>,
    pub load_addr: Option<usize>,
    pub version: Option<u32>,
    pub timestamp: Option<u64>,
    read_complete: bool,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Parsestate {
    Tag,
    Hashalgo,
    Blocksize,
    Digest,
    Keyhash,
    Signalgo,
    Siganture,
    Imagetype,
    Loadaddr,
    Version,
    Timestamp,
    Blob,
    End,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Parser {
    pos: usize,
    state: Parsestate,
}

impl Image {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read(&mut self, image: &mut Vec<u8>) -> Result<()> {
        if self.read_complete {
            return Err(Error::from(Errorkind::ImageAlreadyRead));
        }

        // Entire image size
        let image_size = image.len();
        if image_size <= HEADER_SIZE {
            return Err(Error::from(Errorkind::InvalidImagesize));
        }
        // Check magic
        let magic = &image[0..4];
        Self::chk_magic(match magic.try_into() {
            Ok(v) => v,
            Err(err) => return Err(Error::from(err)),
        })?;
        // Blob size
        let blob_size = u32::from_le_bytes(match image[4..8].try_into() {
            Ok(v) => v,
            Err(err) => return Err(Error::from(err)),
        });
        if image_size != blob_size as usize + HEADER_SIZE {
            return Err(Error::from(Errorkind::InvalidImagesize));
        }

        // Parser
        let mut parser = Parser {
            pos: 8,
            state: Parsestate::Tag,
        };

        while parser.state != Parsestate::End {
            // Skip padding
            if image[parser.pos] == HEADER_PADDING {
                parser.pos += 1;
                continue;
            }

            match parser.state {
                // Tag
                Parsestate::Tag => {
                    // End of Header
                    if parser.pos == HEADER_SIZE {
                        parser.state = Parsestate::Blob;
                        continue;
                    }

                    let tag = image[parser.pos];
                    parser.state = match tag {
                        FIELD_HASH_ALGO => Parsestate::Hashalgo,
                        FIELD_BLOCK_SIZE => Parsestate::Blocksize,
                        FIELD_DIGEST => Parsestate::Digest,
                        FIELD_KEY_HASH => Parsestate::Keyhash,
                        FIELD_SIGNATURE_ALGO => Parsestate::Signalgo,
                        FIELD_SIGNATURE => Parsestate::Siganture,
                        FIELD_IMAGE_TYPE => Parsestate::Imagetype,
                        FIELD_LOAD_ADDRESS => Parsestate::Loadaddr,
                        FIELD_VERSION_CODE => Parsestate::Version,
                        FIELD_TIMESTAMP => Parsestate::Timestamp,
                        _ => unimplemented!(),
                    };
                }

                // Hash algo
                Parsestate::Hashalgo => {
                    if let Some(_) = self.hash_algo {
                        return Err(Error::from(Errorkind::DuplicatedField));
                    }

                    parser.pos += 2;

                    self.hash_algo = Some(match image[parser.pos] {
                        HASH_SHA3_384 => Hashalgo::Sha3_384,
                        _ => return Err(Error::from(Errorkind::ParseImageError)),
                    });

                    parser.pos += 1;
                    parser.state = Parsestate::Tag;
                }

                // Block size field
                Parsestate::Blocksize => {
                    parser.pos += 4;

                    if parser.pos + 8 >= HEADER_SIZE {
                        return Err(Error::from(Errorkind::ParseImageError));
                    }

                    if let Some(_) = self.block_size {
                        return Err(Error::from(Errorkind::DuplicatedField));
                    }

                    self.block_size = Some(u64::from_le_bytes(
                        match image[parser.pos..parser.pos + 8].try_into() {
                            Ok(v) => v,
                            Err(err) => return Err(Error::from(err)),
                        },
                    ));

                    parser.pos += 8;
                    parser.state = Parsestate::Tag;
                }

                // Digest
                Parsestate::Digest => {
                    if let Some(_) = self.digest {
                        return Err(Error::from(Errorkind::DuplicatedField));
                    }

                    parser.pos += 2;

                    let algo = match self.hash_algo {
                        Some(algo) => algo,
                        None => return Err(Error::from(Errorkind::ParseImageError)),
                    };

                    let digest_size =
                        u32::from_le_bytes(match image[parser.pos..parser.pos + 4].try_into() {
                            Ok(v) => v,
                            Err(err) => return Err(Error::from(err)),
                        });

                    if digest_size != algo.digest_size() as u32 {
                        return Err(Error::from(Errorkind::ParseImageError));
                    }

                    parser.pos += 4;

                    self.digest =
                        Some(image[parser.pos..parser.pos + digest_size as usize].to_vec());

                    parser.pos += digest_size as usize;
                    parser.state = Parsestate::Tag;
                }

                // Key Hash
                Parsestate::Keyhash => {
                    if let Some(_) = self.key_hash {
                        return Err(Error::from(Errorkind::DuplicatedField));
                    }

                    parser.pos += 2;

                    let algo = match self.hash_algo {
                        Some(algo) => algo,
                        None => return Err(Error::from(Errorkind::ParseImageError)),
                    };

                    let digest_size =
                        u32::from_le_bytes(match image[parser.pos..parser.pos + 4].try_into() {
                            Ok(v) => v,
                            Err(err) => return Err(Error::from(err)),
                        });

                    if digest_size != algo.digest_size() as u32 {
                        return Err(Error::from(Errorkind::ParseImageError));
                    }

                    parser.pos += 4;

                    self.key_hash =
                        Some(image[parser.pos..parser.pos + digest_size as usize].to_vec());

                    parser.pos += digest_size as usize;
                    parser.state = Parsestate::Tag;
                }

                // Signature algo
                Parsestate::Signalgo => {
                    if let Some(_) = self.sign_algo {
                        return Err(Error::from(Errorkind::DuplicatedField));
                    }

                    parser.pos += 2;

                    self.sign_algo = Some(match image[parser.pos] {
                        SIGN_ED25519 => Signalgo::Ed25519,
                        _ => return Err(Error::from(Errorkind::ParseImageError)),
                    });

                    parser.pos += 1;
                    parser.state = Parsestate::Tag;
                }

                // Signature
                Parsestate::Siganture => {
                    if let Some(_) = self.signature {
                        return Err(Error::from(Errorkind::DuplicatedField));
                    }

                    parser.pos += 2;

                    let algo = match self.sign_algo {
                        Some(algo) => algo,
                        None => return Err(Error::from(Errorkind::ParseImageError)),
                    };

                    let sigsize =
                        u32::from_le_bytes(match image[parser.pos..parser.pos + 4].try_into() {
                            Ok(v) => v,
                            Err(err) => return Err(Error::from(err)),
                        });

                    if sigsize != algo.size() as u32 {
                        return Err(Error::from(Errorkind::ParseImageError));
                    }

                    parser.pos += 4;

                    self.signature =
                        Some(image[parser.pos..parser.pos + sigsize as usize].to_vec());

                    parser.pos += sigsize as usize;
                    parser.state = Parsestate::Tag;
                }

                // Image type
                Parsestate::Imagetype => {
                    if let Some(_) = self.image_type {
                        return Err(Error::from(Errorkind::DuplicatedField));
                    }

                    parser.pos += 2;

                    self.image_type = Some(image[parser.pos]);

                    parser.pos += 1;
                    parser.state = Parsestate::Tag;
                }

                // Load address
                Parsestate::Loadaddr => {
                    if let Some(_) = self.load_addr {
                        return Err(Error::from(Errorkind::DuplicatedField));
                    }

                    parser.pos += 4;

                    self.load_addr = Some(usize::from_le_bytes(
                        match image[parser.pos..parser.pos + 8].try_into() {
                            Ok(v) => v,
                            Err(err) => return Err(Error::from(err)),
                        },
                    ));

                    parser.pos += 8;
                    parser.state = Parsestate::Tag;
                }

                // Version
                Parsestate::Version => {
                    if let Some(_) = self.version {
                        return Err(Error::from(Errorkind::DuplicatedField));
                    }

                    parser.pos += 4;

                    self.version = Some(u32::from_le_bytes(
                        match image[parser.pos..parser.pos + 4].try_into() {
                            Ok(v) => v,
                            Err(err) => return Err(Error::from(err)),
                        },
                    ));

                    parser.pos += 4;
                    parser.state = Parsestate::Tag;
                }

                Parsestate::Timestamp => {
                    if let Some(_) = self.timestamp {
                        return Err(Error::from(Errorkind::DuplicatedField));
                    }

                    parser.pos += 4;

                    self.timestamp = Some(u64::from_le_bytes(
                        match image[parser.pos..parser.pos + 8].try_into() {
                            Ok(v) => v,
                            Err(err) => return Err(Error::from(err)),
                        },
                    ));

                    parser.pos += 8;
                    parser.state = Parsestate::Tag;
                }

                // Blob
                Parsestate::Blob => {
                    if let Some(_) = self.blob {
                        return Err(Error::from(Errorkind::DuplicatedField));
                    }

                    let blob = image[parser.pos..image_size].to_vec();
                    if blob.len() != blob_size as usize {
                        return Err(Error::from(Errorkind::InvalidImagesize));
                    }

                    self.blob = Some(blob);

                    parser.state = Parsestate::End;
                }

                _ => return Err(Error::from(Errorkind::ParseImageError)),
            }
        }

        self.validate()?;

        self.read_complete = true;

        Ok(())
    }

    pub fn pp(&self) -> Result<()> {
        if !self.read_complete {
            return Err(Error::from(Errorkind::ImageNotRead));
        }

        if let Some(v) = &self.blob {
            println!("{} {} (bytes)", "Blob size:".bold(), v.len());
        }

        if let Some(algo) = self.hash_algo {
            println!("{} {}", "Hash algorithm:".bold(), algo.to_name());
        }

        if let Some(size) = self.block_size {
            println!("{} {} (bytes)", "Hash block:".bold(), size);
        }

        if let Some(v) = &self.digest {
            println!("{} {}", "Digest:".bold(), hex::encode(v));
        }

        if let Some(v) = &self.key_hash {
            println!("{} {}", "Key hint:".bold(), hex::encode(v));
        }

        if let Some(algo) = self.sign_algo {
            println!("{} {}", "Signing algorithm:".bold(), algo.to_name())
        }

        if let Some(v) = &self.signature {
            println!("{} {}", "Signature:".bold(), hex::encode(v));
        }

        if let Some(t) = self.image_type {
            println!("{} 0x{:x}", "Type:".bold(), t);
        }

        if let Some(addr) = self.load_addr {
            println!("{} 0x{:x}", "Load address:".bold(), addr);
        }

        if let Some(ver) = self.version {
            println!("{} {}", "Version:".bold(), ver);
        }

        if let Some(ts) = self.timestamp {
            println!("{} {}", "Timestamp:".bold(), ts);
        }

        Ok(())
    }

    fn validate(&self) -> Result<()> {
        if let None = self.blob {
            return Err(Error::from(Errorkind::IncompleteImage));
        }

        if let None = self.hash_algo {
            return Err(Error::from(Errorkind::IncompleteImage));
        }

        if let None = self.digest {
            return Err(Error::from(Errorkind::IncompleteImage));
        }

        if let None = self.sign_algo {
            return Err(Error::from(Errorkind::IncompleteImage));
        }

        if let None = self.signature {
            return Err(Error::from(Errorkind::IncompleteImage));
        }

        if let None = self.key_hash {
            return Err(Error::from(Errorkind::IncompleteImage));
        }

        Ok(())
    }

    fn chk_magic(magic: &[u8; 4]) -> Result<()> {
        match u32::from_be_bytes(*magic) {
            IMAGE_MAGIC => Ok(()),
            _ => Err(Error::from(Errorkind::ParseImageError)),
        }
    }
}

// Error type

type Result<T> = std::result::Result<T, Error>;

pub enum Errorkind {
    FileioError,
    WolfcryptError,
    ImageCreationError,
    SigningError,
    HashingError,
    ParseImageError,
    InvalidImagesize,
    DuplicatedField,
    InvalidValue,
    ImageNotRead,
    ImageAlreadyRead,
    IncompleteImage,
}

impl Errorkind {
    pub fn desc(&self) -> &'static str {
        match self {
            Errorkind::FileioError => "file i/o error",
            Errorkind::WolfcryptError => "An error ocuured in wolfCrypt",
            Errorkind::ImageCreationError => "Failed to create image",
            Errorkind::SigningError => "Failed to sign",
            Errorkind::HashingError => "Failed to calculate digest",
            Errorkind::ParseImageError => "Failed to parse image header",
            Errorkind::InvalidImagesize => "Image size is invalid",
            Errorkind::DuplicatedField => "Detected duplicated field",
            Errorkind::InvalidValue => "Invalid field value",
            Errorkind::ImageNotRead => "Empty image",
            Errorkind::ImageAlreadyRead => "Image already read",
            Errorkind::IncompleteImage => "Some required field not found in header",
            // _ => unimplemented!(),
        }
    }
}

pub struct Error {
    _error: _Error,
}

enum _Error {
    Simple(Errorkind),
    Custom((Errorkind, Box<dyn error::Error + Send + Sync>)),
}

impl Error {
    fn new<E>(kind: Errorkind, error: E) -> Self
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Error {
            _error: _Error::Custom((kind, error.into())),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self._error {
            _Error::Simple(s) => f.write_str(s.desc()),
            _Error::Custom(c) => c.1.fmt(f),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self._error {
            _Error::Simple(_) => None,
            _Error::Custom(c) => c.1.source(),
        }
    }
}

impl From<Errorkind> for Error {
    fn from(value: Errorkind) -> Self {
        Error {
            _error: _Error::Simple(value),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::new(Errorkind::FileioError, err)
    }
}

impl From<wolfcrypt::error::Error> for Error {
    fn from(err: wolfcrypt::error::Error) -> Self {
        Error::new(Errorkind::WolfcryptError, err)
    }
}

impl From<core::array::TryFromSliceError> for Error {
    fn from(err: core::array::TryFromSliceError) -> Self {
        Error::new(Errorkind::ParseImageError, err)
    }
}

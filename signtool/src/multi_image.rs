#![allow(dead_code)]

use std::time::SystemTime;

use wolfssl::wolfcrypt;

use crate::image::{HEADER_SIZE, Image};

pub const IMAGE_MAGIC: u32 = 0x4d534543; // Magic number
pub const IMAGE_TRAIL: u32 = 0x44417a9f; // Trail

// Hash algorithms
pub const HASH_SHA3_384: u8 = 0x01;
pub const HASH_SHA_256: u8 = 0x02;

// Signature algorithms
pub const SIGN_ED25519: u8 = 0x01;

pub const PADDING: u8 = 0xFF;

pub struct Multipleheader<T, U> {
    images: Vec<Image>,
    key: Vec<u8>,
    hash: T,
    sign: U,
}

impl<T: wolfcrypt::Hash, U: wolfcrypt::Sign> Multipleheader<T, U> {
    pub fn new(keybytes: Vec<u8>) -> Self {
        let mut sig = U::new();

        let privkey_size = U::size_of_key();
        let pubkey_size = U::size_of_pubkey();

        if keybytes.len() != privkey_size + pubkey_size {
            panic!("key error")
        }

        sig.import(
            &mut keybytes[0..privkey_size].to_vec(),
            &mut keybytes[privkey_size..].to_vec(),
        )
        .unwrap();

        Self {
            images: Vec::new(),
            key: keybytes,
            hash: T::new(),
            sign: sig,
        }
    }

    pub fn add(&mut self, data: &mut Vec<u8>) -> &Self {
        let mut input = Image::new();
        if let Err(err) = input.read(data) {
            panic!("{:?}", err)
        }
        if let None = input.image_type {
            panic!("For multiple image, image type field must be provided")
        }
        self.images.push(input);
        self
    }

    pub fn build(&mut self) -> Option<Vec<u8>> {
        let image_n = self.images.len() as u32;
        if image_n == 0 {
            return None;
        }

        let header_size = Self::header_size(image_n);

        let mut multi_header = Vec::<u8>::new();
        // Magic
        for v in IMAGE_MAGIC.to_be_bytes() {
            multi_header.push(v);
        }

        // image number
        for v in image_n.to_le_bytes() {
            multi_header.push(v);
        }

        // header size
        for v in header_size.to_le_bytes() {
            multi_header.push(v)
        }

        // For each image
        let mut offset = header_size;
        for img in self.images.iter() {
            // Type
            let img_type = img.image_type.unwrap();
            multi_header.push(img_type);
            multi_header.push(PADDING);
            multi_header.push(PADDING);
            multi_header.push(PADDING);

            // Offset from top
            for v in offset.to_le_bytes() {
                multi_header.push(v);
            }

            println!("Type: 0x{:x}, Offset: {}", img_type, offset);

            offset += img.blob.as_ref().unwrap().len() + HEADER_SIZE;
        }

        let timestamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(err) => panic!("{}", err),
        };

        // Timestamp
        for v in timestamp.to_le_bytes() {
            multi_header.push(v);
        }

        // Multi-image size
        for v in offset.to_le_bytes() {
            multi_header.push(v)
        }

        // Algorithm
        match T::as_str() {
            "sha3-384" => {
                multi_header.push(HASH_SHA3_384);
                multi_header.push(PADDING);
                multi_header.push(PADDING);
                multi_header.push(PADDING);
            }
            _ => unimplemented!(),
        }

        match U::as_str() {
            "ed25519" => {
                multi_header.push(SIGN_ED25519);
                multi_header.push(PADDING);
                multi_header.push(PADDING);
                multi_header.push(PADDING);
            }
            _ => unimplemented!(),
        }

        // Signature
        if let Err(err) = self
            .hash
            .update(multi_header.as_ptr(), multi_header.len() as i32)
        {
            panic!("hash error, {}", err)
        }

        let mut header_digest = match self.hash.finalize() {
            Ok(v) => v,
            Err(err) => panic!("{}", err),
        };

        let signature = match self.sign.sign(&mut header_digest) {
            Ok(v) => v,
            Err(err) => panic!("{}", err),
        };

        for v in signature.iter() {
            multi_header.push(*v);
        }

        multi_header.push(PADDING);
        multi_header.push(PADDING);
        multi_header.push(PADDING);
        multi_header.push(PADDING);

        self.hash.free();
        self.sign.free().unwrap();

        // Trail
        for v in IMAGE_TRAIL.to_be_bytes() {
            multi_header.push(v);
        }

        if multi_header.len() != header_size {
            panic!("Failed to create multiple image header, invalid header size")
        }

        Some(multi_header)
    }

    pub fn header_size(image_n: u32) -> usize {
        48 + U::size_of_sig() + 12 * image_n as usize
    }
}

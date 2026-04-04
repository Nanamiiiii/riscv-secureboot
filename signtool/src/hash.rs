use wolfssl::wolfcrypt;

// Available algorithm
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Hashalgo {
    Sha3_384,
}

#[allow(dead_code)]
impl Hashalgo {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "sha3" => Some(Self::Sha3_384),
            _ => None,
        }
    }

    pub fn to_name(self) -> &'static str {
        match self {
            Self::Sha3_384 => "sha3",
        }
    }

    pub fn list_available() -> Vec<&'static str> {
        vec!["sha3"]
    }

    pub fn digest_size(self) -> usize {
        match self {
            Self::Sha3_384 => 48,
        }
    }
}

#[derive(Debug)]
pub struct Hash<T> {
    ctx: T,
    block: Option<u64>,
    digests: Vec<Vec<u8>>,
    root: Vec<u8>,
    hdr_digest: Option<Vec<u8>>,
}

#[allow(dead_code)]
impl<T> Hash<T> {
    pub fn block(&mut self, size: u64) -> &mut Self {
        if size == 0 {
            panic!("cannot set block to 0!")
        }
        self.block = Some(size);
        self
    }

    pub fn get_block(&self) -> Option<u64> {
        self.block
    }

    pub fn block_digests(&self) -> Vec<Vec<u8>> {
        self.digests.clone()
    }

    pub fn root_digest(&self) -> Vec<u8> {
        self.root.clone()
    }
}

#[allow(dead_code)]
impl<T: wolfcrypt::Hash> Hash<T> {
    pub fn new() -> Self {
        Self {
            ctx: T::new(),
            block: None,
            digests: Vec::new(),
            root: Vec::new(),
            hdr_digest: None,
        }
    }

    pub fn calc(&mut self, data: &mut Vec<u8>, header: Option<&mut Vec<u8>>) {
        self.calc_block(data);
        if let Some(v) = header {
            self.calc_header(v)
        }
        self.calc_root();
    }

    fn calc_block(&mut self, data: &mut Vec<u8>) {
        let length = data.len();
        let block_size = match self.block {
            Some(size) => size as usize,
            None => length,
        };
        let block_n = if length % block_size == 0 {
            length / block_size
        } else {
            length / block_size + 1
        };

        self.digests = Vec::with_capacity(block_n);

        for i in 0..block_n {
            let identifier = i.to_le_bytes();

            match self
                .ctx
                .update(identifier.as_ptr(), identifier.len() as i32)
            {
                Ok(_) => {}
                Err(err) => panic!("{}", err),
            }

            let begin = i * block_size;

            let end = if begin + block_size > length {
                length
            } else {
                begin + block_size
            };

            let block = &data[begin..end];

            match self.ctx.update(block.as_ptr(), block.len() as i32) {
                Ok(_) => {}
                Err(err) => panic!("{}", err),
            }

            let block_digest = match self.ctx.finalize() {
                Ok(d) => d,
                Err(err) => panic!("{}", err),
            };

            self.digests.push(block_digest.to_vec());

            match self.ctx.reset() {
                Ok(_) => {}
                Err(err) => panic!("{}", err),
            }
        }
    }

    fn calc_header(&mut self, header: &mut Vec<u8>) {
        if let Err(err) = self.ctx.update(header.as_ptr(), header.len() as i32) {
            panic!("{}", err)
        }

        self.hdr_digest = match self.ctx.finalize() {
            Ok(d) => Some(d),
            Err(err) => panic!("{}", err),
        };

        match self.ctx.reset() {
            Ok(_) => {}
            Err(err) => panic!("{}", err),
        }
    }

    fn calc_root(&mut self) {
        match self.block {
            Some(_) => {
                if let Some(v) = &self.hdr_digest {
                    if let Err(err) = self.ctx.update(v.as_ptr(), v.len() as i32) {
                        panic!("{}", err)
                    }
                }

                for v in self.digests.iter() {
                    match self.ctx.update(v.as_ptr(), v.len() as i32) {
                        Ok(_) => {}
                        Err(err) => panic!("{}", err),
                    }
                }

                let digest = match self.ctx.finalize() {
                    Ok(d) => d,
                    Err(err) => panic!("{}", err),
                };

                self.root = digest.to_vec();

                match self.ctx.reset() {
                    Ok(_) => {}
                    Err(err) => panic!("{}", err),
                }
            }
            None => {
                if self.digests.len() != 1 {
                    panic!("Invalid operation.")
                }

                if let Some(v) = &self.hdr_digest {
                    if let Err(err) = self.ctx.update(v.as_ptr(), v.len() as i32) {
                        panic!("{}", err)
                    }
                }

                let blob_digest = match self.digests.get(0) {
                    Some(v) => v.clone(),
                    None => panic!("calculate block digests before!"),
                };

                if let Err(err) = self
                    .ctx
                    .update(blob_digest.as_ptr(), blob_digest.len() as i32)
                {
                    panic!("{}", err)
                }

                let digest = match self.ctx.finalize() {
                    Ok(d) => d,
                    Err(err) => panic!("{}", err),
                };

                self.root = digest.to_vec();

                match self.ctx.reset() {
                    Ok(_) => {}
                    Err(err) => panic!("{}", err),
                }
            }
        };
    }

    pub fn free(&mut self) {
        self.ctx.free();
    }
}

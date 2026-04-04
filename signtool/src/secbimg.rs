use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process::exit;

use clap::{Parser, Subcommand};
use colored::Colorize;
use dialoguer::{Confirm, Input, theme::ColorfulTheme};
use hex::ToHex;

use wolfssl::wolfcrypt::{self, ed25519, sha3};

mod hash;
mod image;
mod multi_image;
mod signature;
use hash::{Hash, Hashalgo};
use image::ImageBuilder;
use multi_image::Multipleheader;
use signature::Signalgo;

#[derive(Debug, Parser)]
#[clap(
    name = "secbimg",
    version,
    author,
    about,
    arg_required_else_help = true
)]
struct Args {
    #[clap(subcommand)]
    subcommand: Subcommands,
}

#[derive(Debug, Subcommand)]
enum Subcommands {
    Create {
        #[clap(short = 'i', long, help = "Raw image file to sign")]
        image: Option<String>,

        #[clap(short, long = "load-to", help = "Address to load the image to")]
        load_addr: Option<String>,

        #[clap(short = 't', long = "img-type", help = "Type of the image")]
        img_type: Option<u8>,

        #[clap(short, long, help = "Version code of the image")]
        version: Option<u32>,

        #[clap(short = 'a', long = "hash", help = "Hash algorithm")]
        hash_algo: Option<String>,

        #[clap(short = 'b', long = "block", help = "Size of hash block")]
        hash_block: Option<u64>,

        #[clap(short, long = "signature", help = "Signature algorithm")]
        signature_algo: Option<String>,

        #[clap(short, long = "key", help = "Key file for signing")]
        key_file: Option<String>,

        #[clap(short, long, help = "Output image name")]
        output: Option<String>,

        #[clap(long, help = "Interactive mode", default_value = "false")]
        interactive: bool,
    },
    Inspect {
        #[clap(short, long, help = "Image filename to inspect")]
        image: Option<String>,
    },
    Union {
        #[clap(short, long, help = "Images to union")]
        images: Vec<String>,

        #[clap(short, long = "key", help = "Key file for signing")]
        key_file: String,

        #[clap(short, long, help = "Output image name")]
        output: String,
    },
    Digest {
        #[clap(short, long, help = "Image filename to inspect")]
        image: Option<String>,

        #[clap(short = 'b', long = "block", help = "Size of hash block")]
        hash_block: Option<u64>,
    },
}

#[allow(unused_variables)]
fn main() {
    let args = Args::parse();
    match args.subcommand {
        Subcommands::Create {
            image,
            load_addr,
            img_type,
            version,
            hash_algo,
            hash_block,
            signature_algo,
            key_file,
            output,
            interactive,
        } => {
            // Create image
            // interactive mode
            if interactive {
                println!("Running in interactive mode");

                // Image file
                let path_str = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Input image: ")
                    .interact_text()
                    .unwrap();

                let image_path = Path::new(&path_str);

                if image_path.exists() && image_path.is_file() {
                    println!("Found input file: {}", &path_str)
                } else {
                    println!("{} no such file - {}", "error:".red().bold(), &path_str);
                    exit(1)
                }

                // Output image file
                let output = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Output image: ")
                    .with_initial_text(path_str.clone() + "_signed.bin")
                    .interact_text()
                    .unwrap();

                // Hash algorithm
                let hash_algo_name = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Hash algorithm: ")
                    .interact_text()
                    .unwrap();

                let hash_algo = match Hashalgo::from_name(&hash_algo_name) {
                    Some(algo) => algo,
                    None => {
                        println!(
                            "{} invalid hash algo name `{}`",
                            "error:".red().bold(),
                            hash_algo_name
                        );
                        print!("Available algorithms: ");
                        io::stdout().flush().unwrap();
                        for v in Hashalgo::list_available() {
                            print!("{} ", v);
                        }
                        println!();
                        exit(1)
                    }
                };

                // Signatunre algorithm
                let sign_algo_name = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Signing algorithm: ")
                    .interact_text()
                    .unwrap();

                let signature_algo = match Signalgo::from_name(&sign_algo_name) {
                    Some(algo) => algo,
                    None => {
                        println!(
                            "{} invalid sign algo name `{}`",
                            "error:".red().bold(),
                            sign_algo_name
                        );
                        print!("Available algorithms: ");
                        io::stdout().flush().unwrap();
                        for v in Signalgo::list_available() {
                            print!("{} ", v);
                        }
                        println!();
                        exit(1)
                    }
                };

                // Target image
                let mut target_image = match (hash_algo, signature_algo) {
                    (Hashalgo::Sha3_384, Signalgo::Ed25519) => {
                        ImageBuilder::<sha3::Sha3_384, ed25519::Ed25519>::new()
                    } // _ => unimplemented!(),
                };

                // Load base image
                let mut file = match File::open(path_str.as_str()) {
                    Ok(f) => f,
                    Err(err) => {
                        panic!("Failed to open file, {:?}", err)
                    }
                };

                let mut buf = Vec::new();
                if let Err(err) = file.read_to_end(&mut buf) {
                    panic!("Failed to read file, {:?}", err)
                }
                target_image.base_image(&mut buf);

                // Key file
                let key_file = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Signing key file: ")
                    .interact_text()
                    .unwrap();

                let mut file = match File::open(key_file.as_str()) {
                    Ok(f) => f,
                    Err(err) => {
                        panic!("Failed to open key file, {:?}", err)
                    }
                };

                let mut keybuf = Vec::new();
                if let Err(err) = file.read_to_end(&mut keybuf) {
                    panic!("Failed to read key file, {:?}", err)
                }
                target_image.key(&mut keybuf);

                // Blocked hash
                if Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Use blocked hash?")
                    .interact()
                    .unwrap()
                {
                    let block_size = Input::<u64>::with_theme(&ColorfulTheme::default())
                        .with_prompt("Size of hash block (bytes): ")
                        .interact_text()
                        .unwrap();

                    target_image.enable_block(block_size);
                }

                // Load address
                let load_addr = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Load address (hex): ")
                    .allow_empty(true)
                    .interact_text()
                    .unwrap();

                if !load_addr.is_empty() {
                    target_image.load_to(match usize::from_str_radix(&load_addr, 16) {
                        Ok(v) => v,
                        Err(_) => {
                            println!(
                                "{} invalid load address `{}`",
                                "error:".red().bold(),
                                load_addr
                            );
                            exit(1)
                        }
                    });
                }

                // Image type
                if Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Specify image type identifier?")
                    .interact()
                    .unwrap()
                {
                    let img_type = Input::<u8>::with_theme(&ColorfulTheme::default())
                        .with_prompt("Image type identifier (hex byte): ")
                        .interact_text()
                        .unwrap();

                    target_image.set_type(img_type);
                }

                // Version
                if Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Set image version identifier?")
                    .interact()
                    .unwrap()
                {
                    let img_ver = Input::<u32>::with_theme(&ColorfulTheme::default())
                        .with_prompt("Version identifier (4bytes): ")
                        .interact_text()
                        .unwrap();

                    target_image.set_version(img_ver);
                }

                // Create image file
                if let Err(err) = target_image.build() {
                    panic!("{}", err)
                }

                match target_image.create(output) {
                    Ok(_) => {
                        println!("{}", "Image creation successful!".green().bold());
                        println!(
                            "{} {}",
                            "Digest    :".bold(),
                            target_image.get_digest().encode_hex::<String>()
                        );
                        println!(
                            "{} {}",
                            "Key hash  :".bold(),
                            target_image.get_keyhash().encode_hex::<String>()
                        );
                        println!(
                            "{} {}",
                            "Signature :".bold(),
                            target_image.get_signature().encode_hex::<String>()
                        );
                    }
                    Err(err) => {
                        println!(
                            "{} failed to create signed image file",
                            "error:".red().bold()
                        );
                        println!("{:?}", err);
                        exit(1)
                    }
                }

                // Clean
                target_image.clean();
            } else {
                // Use cli arguments
                // Image file
                let path_str = match image {
                    Some(s) => s,
                    None => lack_of_arg("--image"),
                };
                let image_path = Path::new(&path_str);
                if image_path.exists() && image_path.is_file() {
                    println!("{} {}", "Input image:".bold(), &path_str)
                } else {
                    println!("{}: no such file - {}", "error".red().bold(), &path_str);
                    exit(1)
                }

                // Output image
                let output = match output {
                    Some(s) => {
                        println!("{} {}", "Output image:".bold(), s);
                        s
                    }
                    None => lack_of_arg("--output"),
                };

                // Hash algorithm
                let hash_algo = match hash_algo {
                    Some(s) => match Hashalgo::from_name(&s) {
                        Some(algo) => {
                            println!("{} {}", "Hash algorithm:".bold(), s);
                            algo
                        }
                        None => {
                            println!("{} invalid hash algo name `{}`", "error:".red().bold(), s);
                            print!("Available algorithms: ");
                            io::stdout().flush().unwrap();
                            for v in Hashalgo::list_available() {
                                print!("{} ", v);
                            }
                            println!();
                            exit(1)
                        }
                    },
                    None => lack_of_arg("--hash"),
                };

                // Signature algorithm
                let signature_algo = match signature_algo {
                    Some(s) => match Signalgo::from_name(&s) {
                        Some(algo) => {
                            println!("{} {}", "Signature algorithm:".bold(), s);
                            algo
                        }
                        None => {
                            println!("{} invalid sign algo name `{}`", "error:".red().bold(), s);
                            print!("Available algorithms: ");
                            io::stdout().flush().unwrap();
                            for v in Signalgo::list_available() {
                                print!("{} ", v);
                            }
                            println!();
                            exit(1)
                        }
                    },
                    None => lack_of_arg("--signature"),
                };

                // Target image
                let mut target_image = match (hash_algo, signature_algo) {
                    (Hashalgo::Sha3_384, Signalgo::Ed25519) => {
                        ImageBuilder::<sha3::Sha3_384, ed25519::Ed25519>::new()
                    } // _ => unimplemented!(),
                };

                // Load base image
                let mut file = match File::open(path_str.as_str()) {
                    Ok(f) => f,
                    Err(err) => {
                        panic!("Failed to open file, {:?}", err)
                    }
                };

                let mut buf = Vec::new();
                if let Err(err) = file.read_to_end(&mut buf) {
                    panic!("Failed to read file, {:?}", err)
                }
                target_image.base_image(&mut buf);

                // Key file
                let key_file = match key_file {
                    Some(s) => {
                        println!("{} {}", "Key file:".bold(), s);
                        s
                    }
                    None => lack_of_arg("--key"),
                };

                let mut file = match File::open(key_file.as_str()) {
                    Ok(f) => f,
                    Err(err) => {
                        panic!("Failed to open key file, {:?}", err)
                    }
                };

                let mut keybuf = Vec::new();
                if let Err(err) = file.read_to_end(&mut keybuf) {
                    panic!("Failed to read key file, {:?}", err)
                }
                target_image.key(&mut keybuf);

                // Blocked hash
                match hash_block {
                    Some(size) => {
                        target_image.enable_block(size);
                        println!("{} enabled, {}byte(s)", "Blocked hash:".bold(), size)
                    }
                    None => {
                        println!("{} disabled", "Blocked hash:".bold())
                    }
                }

                // Load address
                match load_addr {
                    Some(addr) => {
                        target_image.load_to(match usize::from_str_radix(&addr, 16) {
                            Ok(v) => {
                                print!("{} enabled, ", "Image load:".bold());
                                println!("{} 0x{}", "Load address:".bold(), addr);
                                v
                            }
                            Err(_) => {
                                println!(
                                    "{} invalid load address `{}`",
                                    "error:".red().bold(),
                                    addr
                                );
                                exit(1)
                            }
                        });
                    }
                    None => {
                        println!("{} disabled", "Image load:".bold())
                    }
                }

                // Image type
                if let Some(t) = img_type {
                    target_image.set_type(t);
                    println!("{} {}", "Image type:".bold(), t)
                }

                // Version
                if let Some(ver) = version {
                    target_image.set_version(ver);
                    println!("{} {}", "Version:".bold(), ver)
                }

                println!();

                // Create image file
                println!("Building image...");
                if let Err(err) = target_image.build() {
                    panic!("{}", err)
                }
                match target_image.create(output) {
                    Ok(_) => {
                        println!("{}", "Image creation successful!".green().bold());
                        println!(
                            "{} {}",
                            "Digest    :".bold(),
                            target_image.get_digest().encode_hex::<String>()
                        );
                        println!(
                            "{} {}",
                            "Key hash  :".bold(),
                            target_image.get_keyhash().encode_hex::<String>()
                        );
                        println!(
                            "{} {}",
                            "Signature :".bold(),
                            target_image.get_signature().encode_hex::<String>()
                        );
                    }
                    Err(err) => {
                        println!(
                            "{} failed to create signed image file",
                            "error:".red().bold()
                        );
                        println!("{:?}", err);
                        exit(1)
                    }
                }

                // Clean
                target_image.clean();
            }
        }
        Subcommands::Inspect { image } => {
            // Inspect image
            let file_path = match image {
                Some(path) => path,
                None => Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Input image: ")
                    .interact_text()
                    .unwrap(),
            };

            let mut file = match File::open(file_path.as_str()) {
                Ok(f) => f,
                Err(err) => {
                    panic!("Failed to open file, {:?}", err)
                }
            };

            let mut buf = Vec::new();
            if let Err(err) = file.read_to_end(&mut buf) {
                panic!("Failed to read file, {:?}", err)
            }

            let mut image_reader = image::Image::new();
            if let Err(err) = image_reader.read(&mut buf) {
                panic!("Image read error, {:?}", err)
            }

            image_reader.pp().unwrap();
        }
        Subcommands::Union {
            images,
            key_file,
            output,
        } => {
            // Union images
            let mut raw_imgs = Vec::<Vec<u8>>::new();
            for path in images.iter() {
                let mut file = match File::open(path.as_str()) {
                    Ok(f) => f,
                    Err(err) => {
                        panic!("Failed to open file, {:?}", err)
                    }
                };

                let mut buf = Vec::new();
                match file.read_to_end(&mut buf) {
                    Ok(_) => {}
                    Err(err) => {
                        panic!("Failed to read file, {:?}", err)
                    }
                }

                raw_imgs.push(buf);

                println!("Loaded: {}", path);
            }

            let mut file = match File::open(key_file.as_str()) {
                Ok(f) => f,
                Err(err) => panic!("Failed to open file, {:?}", err),
            };

            let mut keybytes = Vec::new();
            if let Err(err) = file.read_to_end(&mut keybytes) {
                panic!("Failed to read file, {:?}", err)
            }

            let mut multi_header = Multipleheader::<
                wolfcrypt::sha3::Sha3_384,
                wolfcrypt::ed25519::Ed25519,
            >::new(keybytes);
            for img in raw_imgs.iter_mut() {
                multi_header.add(img);
            }

            let mut output_buf = match multi_header.build() {
                Some(v) => v,
                None => panic!("Image not provided."),
            };

            for img in raw_imgs.iter() {
                output_buf.extend(img);
            }

            let mut file = File::create(output.as_str()).unwrap();
            file.write_all(&output_buf).unwrap();
            file.flush().unwrap();

            println!(
                "{} Image saved as {}",
                "Successful!".green().bold(),
                output.as_str()
            );
        }
        Subcommands::Digest { image, hash_block } => {
            let file_path = match image {
                Some(path) => path,
                None => Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Input image: ")
                    .interact_text()
                    .unwrap(),
            };

            let block_size = match hash_block {
                Some(val) => val,
                None => Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Block size (bytes): ")
                    .interact_text()
                    .unwrap(),
            };

            let mut file = match File::open(file_path.as_str()) {
                Ok(f) => f,
                Err(err) => {
                    panic!("Failed to open file, {:?}", err)
                }
            };

            let mut buf = Vec::new();
            match file.read_to_end(&mut buf) {
                Ok(_) => {}
                Err(err) => {
                    panic!("Failed to read file, {:?}", err)
                }
            }

            let mut image_hash = Hash::<sha3::Sha3_384>::new();
            image_hash.block(block_size).calc(&mut buf, None);
            let digest = image_hash.root_digest();

            println!("{}", "Successful!".green().bold());
            println!("{} {}", "Digest of".bold(), file_path.as_str());
            println!("{} SHA3-384", "Algorithm:".bold());
            println!("{} {}", "Block size (bytes):".bold(), block_size);
            print!("{}", "Root Digest: ".bold());
            for b in digest.iter() {
                print!("{:x}", b)
            }
            println!()
        }
    }
}

// Output templates
#[inline]
fn lack_of_arg(arg: &str) -> ! {
    println!("{} `{}` must specified", "error:".red().bold(), arg);
    exit(1)
}

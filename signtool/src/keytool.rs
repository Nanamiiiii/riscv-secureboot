use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::exit;

use clap::{Parser, Subcommand};
use colored::Colorize;
use dialoguer::{Confirm, theme::ColorfulTheme};

use wolfssl::wolfcrypt::{Sign, ed25519};

mod signature;

// dts conponent
const DTS_HEAD: &'static str = "\
/*
 * Device Tree Source File to store secureboot signing key
 */
/ {
    secureboot {
";
const DTS_OPT_ALGO: &'static str = "\t\talgo = ";
const DTS_OPT_LEN: &'static str = "\t\tpubkey-len = ";
const DTS_OPT_KEY: &'static str = "\t\tpubkey = ";
const DTS_TAIL: &'static str = "\
\t};
};
";

#[derive(Debug, Parser)]
#[clap(
    name = "keytool",
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
        #[clap(short, long, help = "Signing algorithm")]
        algorithm: String,

        #[clap(short, long, help = "Name of the key")]
        keyname: String,

        #[clap(short, long, help = "Flag for pubkey dtsi creation")]
        dts: bool,
    },
}

fn main() {
    let args = Args::parse();
    match args.subcommand {
        Subcommands::Create {
            algorithm,
            keyname,
            dts,
        } => {
            let mut algo_type = get_algo_type(&algorithm);
            let output_filename =
                String::new() + algorithm.as_str() + "_" + keyname.as_str() + ".key";
            let output_pub_filename =
                String::new() + algorithm.as_str() + "_pub_" + keyname.as_str() + ".key";
            let output_path = Path::new(&output_filename);
            if output_path.exists() {
                if output_path.is_file() {
                    if !Confirm::with_theme(&ColorfulTheme::default())
                        .with_prompt(format!(
                            "Key file {} already exists. Are you sure overwrite?",
                            &output_filename
                        ))
                        .interact()
                        .unwrap()
                    {
                        println!("{}", "Canceled.".red().bold());
                        exit(1)
                    }
                } else if output_path.is_dir() {
                    println!(
                        "{} directory named {} exists",
                        "error:".red().bold(),
                        &output_filename
                    );
                    exit(1)
                }
            }

            generate_key(&mut algo_type, output_pub_filename, output_filename);

            if dts {
                let dts_filename =
                    String::new() + algorithm.as_str() + "_" + keyname.as_str() + ".dts";
                let dts_path = Path::new(&dts_filename);
                if dts_path.exists() {
                    if dts_path.is_file() {
                        if !Confirm::with_theme(&ColorfulTheme::default())
                            .with_prompt(format!(
                                "DTS file {} already exists. Are you sure overwrite?",
                                &dts_filename
                            ))
                            .interact()
                            .unwrap()
                        {
                            println!("{}", "Canceled.".red().bold());
                            exit(1)
                        }
                    } else if dts_path.is_dir() {
                        println!(
                            "{} directory named {} exists",
                            "error:".red().bold(),
                            &dts_filename
                        );
                        exit(1)
                    }
                }

                create_dts(&mut algo_type, dts_filename);
            }
        }
    }
}

fn get_algo_type(algo: &str) -> impl Sign {
    match algo {
        "ed25519" => ed25519::Ed25519::new(),
        _ => unimplemented!(),
    }
}

fn generate_key(algo: &mut impl Sign, pub_output: String, comb_output: String) {
    if let Err(err) = algo.generate() {
        panic!("{:?}", err)
    }

    let privkey = match algo.export_private() {
        Ok(v) => v,
        Err(err) => panic!("{:?}", err),
    };

    let pubkey = match algo.export_public() {
        Ok(v) => v,
        Err(err) => panic!("{:?}", err),
    };

    let mut pub_file = File::create(pub_output).unwrap();
    let mut comb_file = File::create(comb_output).unwrap();
    pub_file.write_all(&pubkey).unwrap();
    pub_file.flush().unwrap();
    comb_file.write_all(&privkey).unwrap();
    comb_file.write_all(&pubkey).unwrap();
    comb_file.flush().unwrap();
}

fn create_dts(algo: &mut impl Sign, output: String) {
    let pubkey = match algo.export_public() {
        Ok(v) => v,
        Err(err) => panic!("{:?}", err),
    };

    let algo_name = algo.dyn_as_str();
    let key_size = algo.dyn_size_of_pubkey();

    let mut file = File::create(output).unwrap();
    write!(file, "{}", DTS_HEAD).unwrap();
    writeln!(file, "{}\"{}\";", DTS_OPT_ALGO, algo_name).unwrap();
    writeln!(file, "{}<{}>;", DTS_OPT_LEN, key_size).unwrap();
    write!(file, "{}<", DTS_OPT_KEY).unwrap();
    for (i, v) in pubkey.iter().enumerate() {
        if i % 4 == 0 {
            write!(file, "0x").unwrap();
        }
        write!(file, "{:02x}", v).unwrap();
        if (i + 1) % 4 == 0 && i != key_size - 1 {
            write!(file, " ").unwrap();
        }
    }
    for _ in 0..(3 - ((key_size - 1) % 4)) {
        write!(file, "00").unwrap();
    }
    writeln!(file, ">;").unwrap();
    writeln!(file, "{}", DTS_TAIL).unwrap();

    file.flush().unwrap();
}

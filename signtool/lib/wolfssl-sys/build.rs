extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wolfssl/");

    cc::Build::new()
        .file("wolfssl/src/hash.c")
        .file("wolfssl/src/signature.c")
        .file("wolfssl/src/rng.c")
        .file("wolfssl/wolfssl/wolfcrypt/src/asn.c")
        .file("wolfssl/wolfssl/wolfcrypt/src/aes.c")
        .file("wolfssl/wolfssl/wolfcrypt/src/ecc.c")
        .file("wolfssl/wolfssl/wolfcrypt/src/coding.c")
        .file("wolfssl/wolfssl/wolfcrypt/src/chacha.c")
        .file("wolfssl/wolfssl/wolfcrypt/src/ed25519.c")
        .file("wolfssl/wolfssl/wolfcrypt/src/ed448.c")
        .file("wolfssl/wolfssl/wolfcrypt/src/fe_operations.c")
        .file("wolfssl/wolfssl/wolfcrypt/src/ge_operations.c")
        .file("wolfssl/wolfssl/wolfcrypt/src/fe_448.c")
        .file("wolfssl/wolfssl/wolfcrypt/src/ge_448.c")
        .file("wolfssl/wolfssl/wolfcrypt/src/hash.c")
        .file("wolfssl/wolfssl/wolfcrypt/src/logging.c")
        .file("wolfssl/wolfssl/wolfcrypt/src/memory.c")
        .file("wolfssl/wolfssl/wolfcrypt/src/random.c")
        .file("wolfssl/wolfssl/wolfcrypt/src/rsa.c")
        .file("wolfssl/wolfssl/wolfcrypt/src/sp_int.c")
        .file("wolfssl/wolfssl/wolfcrypt/src/sp_c32.c")
        .file("wolfssl/wolfssl/wolfcrypt/src/sp_c64.c")
        .file("wolfssl/wolfssl/wolfcrypt/src/sha3.c")
        .file("wolfssl/wolfssl/wolfcrypt/src/sha256.c")
        .file("wolfssl/wolfssl/wolfcrypt/src/sha512.c")
        .file("wolfssl/wolfssl/wolfcrypt/src/tfm.c")
        .file("wolfssl/wolfssl/wolfcrypt/src/wc_port.c")
        .file("wolfssl/wolfssl/wolfcrypt/src/wolfmath.c")
        .define("WOLFSSL_USER_SETTINGS", None)
        .include("wolfssl/include")
        .include("wolfssl/wolfssl")
        .compile("libwolfssl.a");

    let bindings = bindgen::builder()
        .header("wolfssl/include/hash.h")
        .header("wolfssl/include/signature.h")
        .header("wolfssl/include/rng.h")
        .clang_arg("-Iwolfssl/include")
        .clang_arg("-Iwolfssl/wolfssl")
        .clang_arg("-DWOLFSSL_USER_SETTINGS")
        .allowlist_function("sha3_384_.*")
        .allowlist_function("sha3_ctx_.*")
        .allowlist_function("ed25519_.*")
        .allowlist_function("rng_.*")
        .allowlist_var("ED25519_.*")
        .allowlist_var("WC_SHA3_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Failed to create bindings.");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("wolfssl_bindings.rs"))
        .expect("Couldn't write bindings!");

    let error_bindings = bindgen::builder()
        .header("wolfssl/wolfssl/wolfssl/wolfcrypt/error-crypt.h")
        .clang_arg("-Iwolfssl/include")
        .clang_arg("-Iwolfssl/wolfssl")
        .clang_arg("-DWOLFSSL_USER_SETTINGS")
        .allowlist_file("wolfssl/wolfssl/wolfssl/wolfcrypt/error-crypt.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Failed to create bindings.");

    let error_out = PathBuf::from(env::var("OUT_DIR").unwrap());
    error_bindings
        .write_to_file(error_out.join("wolfcrypt_errors.rs"))
        .expect("Couldn't write bindings!");
}

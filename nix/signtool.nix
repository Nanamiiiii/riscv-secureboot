{
  lib,
  rustPlatform,
  clang,
  llvmPackages,
  pkg-config,
  rust-bindgen,
  libiconv,
  darwin,
  stdenv,
  wolfssl-src,
  src,
}:
rustPlatform.buildRustPackage {
  pname = "image-signing-tool";
  version = "0.1.0";

  inherit src;

  cargoLock.lockFile = "${src}/Cargo.lock";

  buildInputs = lib.optionals stdenv.isDarwin [
    libiconv
    darwin.apple_sdk.frameworks.Security
    darwin.apple_sdk.frameworks.SystemConfiguration
  ];

  nativeBuildInputs = [
    clang
    llvmPackages.libclang
    pkg-config
    rust-bindgen
  ];

  LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";

  prePatch = ''
    echo "Injecting wolfssl source into lib/wolfssl-sys/wolfssl..."
    rm -rf lib/wolfssl-sys/wolfssl/wolfssl
    cp -r ${wolfssl-src} lib/wolfssl-sys/wolfssl/wolfssl
    chmod -R +w lib/wolfssl-sys/wolfssl/wolfssl
  '';
}

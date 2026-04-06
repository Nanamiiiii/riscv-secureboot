{
  description = "Rust project with C bindings (via bindgen / cc crate)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
      treefmt-nix,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rustfmt"
            "clippy"
            "llvm-tools-preview"
            "rust-analyzer"
          ];
        };

        wolfssl-src = pkgs.callPackage ./nix/wolfssl-src.nix { };

        signtool = pkgs.callPackage ./nix/signtool.nix {
          rustPlatform = pkgs.makeRustPlatform {
            cargo = rustToolchain;
            rustc = rustToolchain;
          };
          inherit wolfssl-src;
          src = ./signtool/.;
        };

        opensbi-riscv64 = pkgs.pkgsCross.riscv64.callPackage ./nix/opensbi-cross.nix { };

        qemu-secureboot-riscv64 = pkgs.callPackage ./nix/qemu-secureboot.nix { inherit pkgs; };
      in
      {
        packages = {
          inherit signtool opensbi-riscv64 qemu-secureboot-riscv64;
          default = pkgs.symlinkJoin {
            name = "secureboot-qemu";
            paths = [
              signtool
              opensbi-riscv64
              qemu-secureboot-riscv64
            ];
          };
        };

        devShells.default = pkgs.mkShell rec {
          inherit (signtool) buildInputs nativeBuildInputs LIBCLANG_PATH;
          RUST_BACKTRACE = "1";
          PKG_CONFIG_PATH = pkgs.lib.makeSearchPathOutput "dev" "lib/pkgconfig" buildInputs;
        };

        formatter = treefmt-nix.lib.mkWrapper pkgs {
          projectRootFile = "flake.nix";
          programs.nixfmt.enable = true;
          programs.rustfmt.enable = true;
        };
      }
    );
}

{ pkgs }:
pkgs.qemu.overrideAttrs (
  final: prev: {
    version = "10.2.2";
    pname = "qemu-secureboot-riscv64";
    src = pkgs.fetchurl {
      url = "https://download.qemu.org/qemu-${final.version}.tar.xz";
      hash = "sha256-eEspb/KcFBeqcjI6vLLS6pq5dxck9Xfc14XDsE8h4XY=";
    };
    patches = (prev.patches or [ ]) ++ [
      ./patches/0002-feat-riscv-implement-secureboot-key-insertion.patch
    ];
    configureFlags =
      (pkgs.lib.filter (x: !(pkgs.lib.hasPrefix "--target-list=" x)) prev.configureFlags)
      ++ [ "--target-list=riscv64-softmmu" ];
    postInstall = pkgs.lib.pipe (prev.postInstall or "") [
      (pkgs.lib.replaceStrings [ "ln -s $out/bin/qemu-system-x86_64 $out/bin/qemu-kvm" ] [ "" ])
    ];
  }
)

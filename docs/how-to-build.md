# Build
All component are packaged using [Nix](https://nixos.org/), but you can also build manually in the ordinary way.

The experimental features of nix `flakes` and `nix-command` are required.

## All Components and Example System
```
nix build .#all-qemu
```
All artifacts will be located under `result/`.

## Bootloader & Firmware

### U-Boot
Source and additional information are located [here](https://github.com/Nanamiiiii/u-boot-secure). 

In this repository, following command builds the all bootloader binaries.
```
# For example rootfs
nix build .#u-boot-secure.qemu
# For Ubuntu
nix build .#u-boot-secure.qemu-ubuntu
```

### OpenSBI
We have no changes from original source. Just build for `generic` platform.

In this repository, following command builds the all opensbi's artifacts.
```
nix build .#opensbi-riscv64
```

## QEMU
Source is located [here](https://github.com/Nanamiiiii/qemu-secureboot.git).

In this repository, all changes are provided as a [patch](../nix/patches/0002-feat-riscv-implement-secureboot-key-insertion.patch) to v10.2.2.
```
nix build .#qemu-secureboot-riscv64
```

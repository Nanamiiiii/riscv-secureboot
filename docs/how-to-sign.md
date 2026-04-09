# Signing Tool
## Build 
Generate two binaries: `keytool` and `secbimg`.

### Nix
```bash
nix build .#signtool
```

### Cargo
Tested on rust 1.94.1.

```bash
cd signtool
cargo build --release
```

## Generate Keys
```bash
keytool create -a ed25519 -k NAME -d
```

This generates following items:
- `ed25519_NAME.key`
  - Priv+Pub Key Binary
  - Used to sign
- `ed25519_pub_NAME.key`
  - Public Key Binary
  - Used for QEMU
- `ed25519_NAME.dts`
  - To embed public key into devicetree
  
## Sign
Simply sign the binary
```bash
secbimg create \
    -i binary-to-sign.bin \
    -o signed.bin \
    -a sha3 -s ed25519 -k ed25519_NAME.key
```
- Options
  - `-i`
    - Path to image to sign
  - `-o`
    - Path to signed image
  - `-a`
    - Hash algorithm (only `sha3`)
  - `-s`
    - Signature algorithm (only `ed25519`)
  - `-k`
    - Path to key file

To enable parallelized hash calculation, specify the size of hash block.
```bash
secbimg create \
    -i binary-to-sign.bin \
    -o signed.bin \
    -a sha3 -s ed25519 -k ed25519_NAME.key \
    -b 8192
```
- Options
  - `-b`
    - The size of hash blocks (bytes)
 
For the bootloader and firmware, load addresses and image types can be included.  
Our U-Boot uses this information for Secure Boot image format.
```bash
secbimg create \
    -i firmware.bin \
    -o firmware-signed.bin \
    -l 80100000 -t 16 \
    -a sha3 -s ed25519 -k ed25519_NAME.key
```
- Options
  - `-l`
    - Load Address (hex w/o prefix)
  - `-t`
    - Image Type (decimal)
    - `0x10 (16)`: Firmware, `0x20 (32)`: Loadable Image, `0x30 (48)`: Flattened Device Tree

Like a FIT-image, multiple images can be combined.
```bash
secbimg union \
     -i firmware-signed.bin \
     -i loader-signed.bin \
     -k ed25519_NAME.key -o signed-loader.bin
```

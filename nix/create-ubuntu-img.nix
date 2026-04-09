{
  mktemp,
  signtool,
  signingKeys,
  util-linux,
  wget,
  writeShellScriptBin,
  xz,
}:
let
  version = "24.04.4";
  image = "ubuntu-${version}-preinstalled-server-riscv64.img";
  url = "https://cdimage.ubuntu.com/releases/${version}/release/${image}.xz";
in
writeShellScriptBin "create-ubuntu-img" ''
  if [ ! -f "${image}" ]; then
    if [ ! -f "${image}.xz" ]; then
      ${wget}/bin/wget ${url}
    fi
    ${xz}/bin/unxz ${image}.xz
  fi

  WORK_DIR="$(${mktemp}/bin/mktemp -d)"
  MNT_DIR="$WORK_DIR"/mnt
  mkdir -p "$MNT_DIR"
  LOOP_DEV=$(sudo ${util-linux}/bin/losetup --find --show --partscan ${image})
  trap 'sudo ${util-linux}/bin/umount "$MNT_DIR" 2> /dev/null || true; sudo ${util-linux}/bin/losetup -d "$LOOP_DEV" 2> /dev/null || true; rm -rf "$WORK_DIR"' EXIT
  sudo ${util-linux}/bin/mount "''${LOOP_DEV}p1" "$MNT_DIR"

  sudo ${signtool}/bin/secbimg create \
    -i "$MNT_DIR"/boot/vmlinuz \
    -a sha3 -s ed25519 -k ${signingKeys}/keys/ed25519_nix.key \
    -o "$MNT_DIR"/boot/vmlinuz-signed 

  sudo ${signtool}/bin/secbimg create \
    -i "$MNT_DIR"/boot/initrd.img \
    -a sha3 -s ed25519 -k ${signingKeys}/keys/ed25519_nix.key \
    -o "$MNT_DIR"/boot/initrd.img-signed 

  sudo ${util-linux}/bin/umount "$MNT_DIR"
  rm -rf "$WORK_DIR"
''

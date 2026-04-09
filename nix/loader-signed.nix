{
  stdenv,
  u-boot-secure,
  opensbi,
  signtool,
  signingKeys,
}:
stdenv.mkDerivation {
  name = "signed-loader-image";

  nativeBuildInputs = [ signtool ];

  unpackPhase = ":";

  buildPhase = ''
    mkdir -p $out/images
    cp ${u-boot-secure}/share/uboot/u-boot-spl.bin $out/images/
    secbimg create \
      -i ${opensbi}/share/opensbi/lp64/generic/firmware/fw_dynamic.bin \
      -a sha3 -s ed25519 -k ${signingKeys}/keys/ed25519_nix.key \
      -o $out/images/fw_dynamic-signed.bin -l 80100000 -t 16
    secbimg create \
      -i ${u-boot-secure}/share/uboot/u-boot-nodtb.bin \
      -a sha3 -s ed25519 -k ${signingKeys}/keys/ed25519_nix.key \
      -o $out/images/u-boot-signed.bin -l 81200000 -t 32
    secbimg union \
      -i $out/images/fw_dynamic-signed.bin \
      -i $out/images/u-boot-signed.bin \
      -k ${signingKeys}/keys/ed25519_nix.key -o $out/images/signed-loader.bin
  '';

  dontUnpack = true;
  dontConfigure = true;
  dontInstall = true;
}

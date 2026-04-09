{ stdenv, signtool }:
stdenv.mkDerivation {
  name = "secureboot-keys";

  nativeBuildInputs = [ signtool ];

  unpackPhase = ":";

  buildPhase = ''
    mkdir -p $out/keys
    keytool create -a ed25519 -k nix -d
    cp ed25519_nix.key ed25519_pub_nix.key ed25519_nix.dts $out/keys
  '';

  dontUnpack = true;
  dontConfigure = true;
  dontInstall = true;
}

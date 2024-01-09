{ rustPlatform, lib, wayland, libxkbcommon, makeWrapper }:
let
  libs = [ wayland libxkbcommon ];
in
rustPlatform.buildRustPackage rec {
  pname = "stroop-gui";
  version = "0.1.0";
  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;
  buildAndTestSubdir = "stroop-gui";

  buildInputs = libs;
  nativeBuildInputs = [ makeWrapper ];

  postInstall = ''
    mv $out/bin/${pname} $out/bin/${pname}-unwrapped
    makeWrapper $out/bin/${pname}-unwrapped $out/bin/${pname} --set LD_LIBRARY_PATH ${lib.makeLibraryPath libs}
  '';
}

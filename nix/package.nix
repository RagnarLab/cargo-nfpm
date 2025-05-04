{ rustPlatform
, lib
, pkg-config
, nfpm
}:
let
  cargoToml = builtins.fromTOML (builtins.readFile ../Cargo.toml);
in
rustPlatform.buildRustPackage rec {
  pname = cargoToml.package.name;
  version = cargoToml.package.version;

  src = builtins.path {
    path = ../.;
  };

  nativeBuildInputs = [
    pkg-config
  ];
  buildInputs = [
    nfpm
  ];

  doCheck = false;

  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
  CARGO_NFPM_BIN = lib.makeBinPath [ nfpm ];

  cargoLock.lockFile = ../Cargo.lock;

  meta = {
    description = cargoToml.package.description;
    homepage = cargoToml.package.homepage;
    license = with lib.licenses; [ mit asl20 ];
    mainProgram = pname;
  };
}

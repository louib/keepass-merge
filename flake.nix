{
  description = "CLI tool to merge KDBX (keepass) databases";

  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }: (
    flake-utils.lib.eachDefaultSystem (
      system: (
        let
          projectName = "keepass-merge";
          pkgs = import nixpkgs {
            inherit system;
          };

          cargoPackages = with pkgs; [
            cargo
            rustc
            rustfmt
          ];
        in {
          devShells = {
            default = pkgs.mkShell {
              buildInputs = cargoPackages;

              shellHook = ''
                export RUSTFLAGS='-C target-cpu=native'
              '';
            };
          };
          packages = {
            default = pkgs.rustPlatform.buildRustPackage rec {
              pname = projectName;
              version = "main";

              src = ./.;

              cargoLock = {
                lockFile = ./Cargo.lock;
                outputHashes = {
                  # This hash need to be updated everytime you bump the version of the keepass-rs
                  # library.
                  "keepass-0.0.0-placeholder-version" = "sha256-/lFH5k8fzfJUpA7Mn/cLFsfH0N8Z4j96ocLBDlqWf08=";
                };
              };

              meta = with pkgs.lib; {
                description = "CLI tool to merge KDBX (keepass) databases";
                homepage = "https://github.com/louib/${projectName}";
                license = licenses.gpl3;
                # maintainers = [];
              };
            };
          };
        }
      )
    )
  );
}

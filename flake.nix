{
  description = "cpexpander flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};

      buildInputs = with pkgs; [
        rustc
        rustfmt
        rust-analyzer
        cargo
      ];
    in {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = buildInputs;
      };

      packages.${system}.default = pkgs.rustPlatform.buildRustPackages {
        name = "cpexpander";
        cargoLock.lockFile = ./Cargo.lock;
        src = pkgs.lib.cleanSource ./.;
      };
    };
}

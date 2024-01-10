{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = { flake-parts, ... } @ inputs:
  let
    libs = pkgs: with pkgs; [
      wayland
      libxkbcommon
    ];
  in
    flake-parts.lib.mkFlake { inherit inputs; }
    {
      systems = [ "x86_64-linux" "aarch64-linux" "i686-linux" ];

      perSystem = { pkgs, ... }: with pkgs; {
        packages.default = callPackage ./package.nix {};
        devShells.default = mkShell rec {
          packages = [ rustc cargo clippy rustfmt rust-analyzer ] ++ libs pkgs;
          LD_LIBRARY_PATH = lib.makeLibraryPath (libs pkgs);
        };
      };
    };
}

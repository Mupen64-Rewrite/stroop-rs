{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = { flake-parts, ... } @ inputs:
    flake-parts.lib.mkFlake { inherit inputs; }
    {
      systems = [ "x86_64-linux" "aarch64-linux" "i686-linux" ];

      perSystem = { pkgs, ... }: {
        packages.default = pkgs.callPackage ./package.nix {};
        devShells.default = pkgs.mkShell {};
      };
      # languages.rust = {
      #   enable = true;
      #   channel = "stable";
      # };

      # packages = with pkgs; [
      #   wayland
      #   libxkbcommon
      #   libGL
      # ];
    };
}

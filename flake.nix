{
  description = "NET7212 dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    { nixpkgs, fenix, ... }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-darwin"
      ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
    in
    {
      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          inherit (pkgs.lib) optionals;
          inherit (pkgs.stdenv.hostPlatform) isLinux isDarwin;

          rust = fenix.packages.${system}.stable.toolchain;
        in
        {
          default = pkgs.mkShell {
            packages =
              with pkgs;
              [
                # C toolchain
                gcc
                clang
                clang-tools
                gnumake
                bats
                check

                # rust
                rust
              ]
              ++ optionals isLinux [
                gdb
                strace
                ltrace
                valgrind
              ]
              ++ optionals isDarwin [
                lldb
              ];
          };
        }
      );
    };
}

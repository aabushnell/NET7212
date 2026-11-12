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
          inherit (pkgs.lib) optionals optionalString;
          inherit (pkgs.stdenv.hostPlatform) isLinux isDarwin;

          rust = fenix.packages.${system}.stable.toolchain;
        in
        {
          default = pkgs.mkShell {
            packages =
              with pkgs;
              [
                pkg-config
                bear
                hyperfine

                # C toolchain
                gcc
                clang
                clang-tools
                gnumake
                bats
                check

                # rust
                rust

                # extra
                zstd
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

            shellHook = ''
              ${optionalString isDarwin ''
                export RUSTFLAGS="-Clinker=/usr/bin/ld"
                export LIBCLANG_PATH="${pkgs.llvmPackages.libclang.lib}/lib"
                export BINDGEN_EXTRA_CLANG_ARGS="$(pkg-config --cflags libzstd)"
              ''}
            '';
          };
        }
      );
    };
}

{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixpkgs-unstable";
    systems.url = "github:nix-systems/default"; # can run on all systems
  };

  outputs = { self, nixpkgs, systems, ... }:
  let
    eachSystem = fn: nixpkgs.lib.genAttrs (import systems) (system: fn system (import nixpkgs {
      inherit system;
    }));

    runtimeDeps = pkgs: with pkgs; [
      # for winit (https://github.com/rust-windowing/winit/issues/3244)
      wayland
      libxkbcommon
      # for the wgpu vulkan backend
      vulkan-loader
    ];
  in
  {
    packages = eachSystem (system: pkgs: rec {
      default = fractals;
      fractals = pkgs.rustPlatform.buildRustPackage rec {
        name = "fractals";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;

        buildInputs = runtimeDeps pkgs;

        # make sure runtime dependencies get picked up
        # inspired by https://github.com/NixOS/nixpkgs/blob/52faf482a3889b7619003c0daec593a1912fddc1/pkgs/by-name/al/alacritty/package.nix
        dontPatchELF = true;
        postInstall = ''patchelf --add-rpath "${pkgs.lib.makeLibraryPath buildInputs}" $out/bin/${name}'';
      };
    });

    devShells = eachSystem (system: pkgs: {
      default = pkgs.mkShell {
        # make sure runtime dependencies get picked up
        # buildInputs doesnt work, see https://github.com/rust-windowing/winit/issues/3244
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (runtimeDeps pkgs);

        nativeBuildInputs = with pkgs; [
          cargo
          rustc
          cargo-edit # provides `cargo upgrade` for dependencies
        ];

        # fix rust-analyzer in vscode
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

        # enable logging
        RUST_LOG = "fractals=debug";
      };
    });
  };
}
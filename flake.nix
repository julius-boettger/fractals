{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixpkgs-unstable";
    systems.url = "github:nix-systems/default"; # can run on all systems
    rust-overlay = { url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs"; };
  };

  outputs = { self, nixpkgs, systems, rust-overlay, ... }:
  let
    eachSystem = fn: nixpkgs.lib.genAttrs (import systems) (system: fn system (import nixpkgs {
      inherit system;
      overlays = [ (import rust-overlay) ];
    }));

    # for cross-compiling to windows
    cross-target = "x86_64-w64-mingw32";

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
          (rust-bin.stable.latest.default.override {
            # fix rust-analyzer in vscode
            extensions = [ "rust-src" ];
            # cross-compile to windows with
            # cargo build --release --target x86_64-pc-windows-gnu
            targets = [ "x86_64-pc-windows-gnu" ];
          })

          cargo-edit # provides `cargo upgrade` for dependencies
          cargo-flamegraph # provides `cargo flamegraph` for profiling
                           # best used with CARGO_PROFILE_RELEASE_DEBUG=true
        ];

        # enable logging
        RUST_LOG = "error,fractals=trace";
        # display backtrace
        RUST_BACKTRACE = 1;

        # for cross-compiling to windows
        # tell cargo to use linker from other nix dev shell
        CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER = "${pkgs.writeShellScriptBin "cross-linker" ''
          nix develop .#cross-linker -c ${cross-target}-cc "$@"
        ''}/bin/cross-linker";
      };

      # for cross-compiling to windows.
      # using a second dev shell was the easiest way i found to make the required
      # pthreads library accessible to the required mingw linker running under wine
      cross-linker = let
        cross-pkgs = import nixpkgs {
          inherit system;
          crossSystem.config = cross-target;
        };
      in cross-pkgs.mkShell {
        buildInputs = [
          cross-pkgs.windows.pthreads
        ];
      };
    });
  };
}
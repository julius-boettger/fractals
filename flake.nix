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
      vulkan-loader # for wgpu vulkan backend
      # for winit (https://github.com/rust-windowing/winit/issues/3244)
      libxkbcommon # keyboard input
      wayland
      xorg.libX11 xorg.libXi xorg.libXcursor
    ];
  in
  {
    packages = eachSystem (system: pkgs: rec {
      default = fractals;
      fractals = pkgs.rustPlatform.buildRustPackage rec {
        name = "fractals";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;

        # make sure runtime dependencies get picked up
        # inspired by https://github.com/NixOS/nixpkgs/blob/52faf482a3889b7619003c0daec593a1912fddc1/pkgs/by-name/al/alacritty/package.nix
        dontPatchELF = true;
        postInstall = ''patchelf --add-rpath "${pkgs.lib.makeLibraryPath (runtimeDeps pkgs)}" $out/bin/${name}'';
      };
    });

    devShells = eachSystem (system: pkgs: {
      default = pkgs.mkShell {
        # make sure runtime dependencies get picked up
        # buildInputs doesnt work, see https://github.com/rust-windowing/winit/issues/3244
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (runtimeDeps pkgs);
        # enable logging
        RUST_LOG = "error,fractals=trace";
        # display backtrace
        RUST_BACKTRACE = 1;

        # use rust with local rustup toolchain from rustup
        shellHook = ''
          export CARGO_HOME="$PWD/.rust/.cargo"
          export RUSTUP_HOME="$PWD/.rust/.rustup"
          export PATH="$PATH:$CARGO_HOME/bin"
          export PATH="$PATH:$RUSTUP_HOME/toolchains/stable-x86_64-unknown-linux-gnu/bin"
        '';

        nativeBuildInputs = with pkgs; [
          rustup

          cargo-edit # provides `cargo upgrade` for dependencies
          cargo-flamegraph # provides `cargo flamegraph` for profiling
                           # best used with CARGO_PROFILE_RELEASE_DEBUG=true

          # attempt: cross compile for linux musl
          # see https://github.com/cross-rs/cross/issues/1383
          cargo-cross
          docker rootlesskit
          (pkgs.writeShellScriptBin "build-linux-musl" ''
            export XARGO_HOME="$PWD/.rust/.xargo"
            export DOCKER_DATA_ROOT="$PWD/.docker"
            export DOCKER_HOST=unix://$XDG_RUNTIME_DIR/docker.sock

            dockerd-rootless &> /dev/null &
            DOCKERD_PID=$!

            # wait until docker daemon has started
            sleep 0.25

            cross build --target x86_64-unknown-linux-musl "$@"

            kill $DOCKERD_PID
          '')
        ];
      };
    });
  };
}
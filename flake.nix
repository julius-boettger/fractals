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
      inherit system overlays;
    }));

    overlays = [ rust-overlay.overlays.default ];

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

        nativeBuildInputs = with pkgs; [
          (rust-bin.stable.latest.default.override {
            # fix rust-analyzer in vscode
            extensions = [ "rust-src" ];
            # necessary for first command of release-windows, see comment below
            targets = [ "x86_64-pc-windows-gnu" ];
          })

          # convenient command to create a linux 
          (pkgs.writeShellScriptBin "release-linux" ''
            rm -rf fractals-linux-x86_64
            cargo build --target x86_64-unknown-linux-gnu --release "$@" || exit 1
            cp target/x86_64-unknown-linux-gnu/release/fractals fractals-linux-x86_64 || exit 1
            # should work for most linux distros
            patchelf --set-interpreter /lib64/ld-linux-x86-64.so.2 fractals-linux-x86_64 || exit 1
          '')

          # convenient command to cross-compile to windows in second nix dev shell.
          # first command (attempt build in current shell) is necessary to compile
          # some crates that require the x86_64-pc-windows-gnu toolchain,
          # which I was not able to set up in the second nix dev shell.
          (pkgs.writeShellScriptBin "release-windows" ''
            rm -rf fractals-windows-x86_64.exe
                                           cargo build --target x86_64-pc-windows-gnu --release "$@"
            nix develop .#cross-windows -c cargo build --target x86_64-pc-windows-gnu --release "$@" || exit 1
            cp target/x86_64-pc-windows-gnu/release/fractals.exe fractals-windows-x86_64.exe || exit 1
          '')

          # convenient command to build supported release binaries
          (pkgs.writeShellScriptBin "release" ''
            release-linux || exit 1
            release-windows || exit 1
          '')

          cargo-edit # provides `cargo upgrade` for dependencies
          cargo-flamegraph # provides `cargo flamegraph` for profiling
                           # best used with CARGO_PROFILE_RELEASE_DEBUG=true
        ];
      };

      # for cross-compiling to windows using mingw compiler with wine
      cross-windows = let
        cross-pkgs = import nixpkgs {
          inherit system overlays;
          crossSystem.config = "x86_64-w64-mingw32";
        };
      # callPackage is necessary (https://github.com/NixOS/nixpkgs/issues/49526)
      in cross-pkgs.callPackage (
        { mkShell, rust-bin, windows, stdenv }:
        mkShell {
          nativeBuildInputs = [ rust-bin.stable.latest.minimal ];
          # necessary for cargo to work
          buildInputs = [ windows.pthreads ];
          CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER = "${stdenv.cc.targetPrefix}cc";
        }
      ) {};
    });
  };
}
{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    systems.url = "github:nix-systems/default"; # can run on all systems
  };

  outputs = { self, nixpkgs, systems, ... }:
  let
    eachSystem = fn: nixpkgs.lib.genAttrs (import systems) (system: fn system (import nixpkgs {
      inherit system;
    }));
  in
  {
    devShells = eachSystem (system: pkgs: {
      default = pkgs.mkShell {
        # runtime dependencies
        LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
          # for winit (https://github.com/rust-windowing/winit/issues/3244)
          wayland
          libxkbcommon
          vulkan-loader
        ];

        nativeBuildInputs = with pkgs; [
          cargo
          cargo-edit # provides `cargo upgrade` for dependencies
        ];

        # fix rust-analyzer in vscode
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
    });
  };
}
{
  description = "A terminal-based Spotify visualizer written in Rust for Kitty";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "spotify-visualizer";
          version = "5.0.0";
          src = ./.;

          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          buildInputs = with pkgs; [
            curl
          ];

          postInstall = ''
            mkdir -p $out/share/doc/spotify-visualizer
          '';

          meta = with pkgs.lib; {
            description = "A terminal-based Spotify visualizer written in Rust for Kitty";
            homepage = "https://github.com/kadsendino/spotify-visualizer";
            license = licenses.mit;
            maintainers = [ ];
            platforms = platforms.linux;
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rust-bin.stable.latest.default
            rust-analyzer
            pkg-config
            curl
            kitty
            playerctl
          ];
        };
      }
    );
}

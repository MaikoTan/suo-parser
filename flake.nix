{
  description = "suo-parser development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs =
    inputs@{ self, nixpkgs, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];

      perSystem =
        { pkgs, ... }:
        {
          devShells.default = pkgs.mkShell {
            packages = with pkgs; [
              # Rust toolchain manager (provides cargo/rustc proxies)
              rustup

              # Rust tooling
              cargo-audit
              cargo-edit
              cargo-insta
              wasm-pack

              # TOML formatting (used by the binding package)
              taplo

              # Node tooling (for the napi-rs binding and its JS tests)
              nodejs
              yarn
            ];

            shellHook = ''
              # Install the nightly toolchain (matching CI) with rustfmt + clippy
              rustup toolchain install nightly --profile minimal --component rustfmt,clippy
              # Add the WASM target used by the parser build
              rustup target add wasm32-unknown-unknown --toolchain nightly
              # Make the nightly toolchain the default so `cargo`/`rustc` work directly
              rustup default nightly
              echo "suo-parser dev shell ready: $(rustc --version) / $(cargo --version)"
            '';
          };
        };
    };
}

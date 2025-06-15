{
  description = "PrEval development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            git
            pre-commit
            nodejs_22
            glow
          ];

          shellHook = ''
            echo "PrEval development environment loaded!"
            echo "Rust version: $(rustc --version)"
            echo "Node.js version: $(node --version)"
            echo "Available commands:"
            echo "  cargo run -- <evaluator> - Run PrEval with evaluator"
            echo "  cargo test - Run tests"
            echo "  cargo fmt --all - Format code"
            echo "  cargo clippy --workspace --all-targets - Run linter"
            echo "  pre-commit install - Install git hooks"
            echo "  npx log4brains preview - Preview ADR documentation"
            echo "  npx log4brains build - Build ADR documentation"
            echo "  glow docs/adr/ - Browse ADRs in terminal"
          '';

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        };
      });
}
# List available commands
default:
    @just --list

# Run in debug mode with logs
run:
    RUST_LOG=debug cargo run

# Build release binary
build:
    cargo build --release

# Format and Lint (Strict)
check:
    cargo fmt --all -- --check
    cargo clippy -- -D warnings

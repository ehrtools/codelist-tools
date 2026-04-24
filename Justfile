# Default recipe: list all available recipes when you run `just` with no args
default:
    @just --list

# List all available recipes
list:
    @just --list

# Ensure Rust code is formatted
fmt:
    cargo fmt

# Check if Rust code is formatted (used in CI)
fmt-check:
    cargo fmt --all -- --check

# Run Clippy with warnings treated as errors
clippy:
    cargo clippy -- -D warnings

# Scan Cargo.toml files for unused dependencies
machete:
    cargo machete

# Run Prettier on Markdown files
prettier:
    npx prettier --write "**/*.md"

# Check Prettier formatting
prettier-check:
    npx prettier --check "**/*.md"

# CI task: check formatting, linting, unused deps, and all tests
ci: fmt-check clippy machete prettier-check test-python test-rust

# Requires an active Python venv (run `source bindings/python/.venv/bin/activate`
# first). `maturin develop` builds the extension and installs it into the
# active venv so the tests exercise the current Rust code, not a stale wheel.
#
# Run python tests
test-python:
    maturin develop --manifest-path bindings/python/Cargo.toml
    sh bindings/python/tests/run.sh

# Run rust tests
test-rust:
    cargo test --all-features

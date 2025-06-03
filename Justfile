# Ensure Rust code is formatted
fmt:
    cargo fmt

# Check if Rust code is formatted (used in CI)
fmt-check:
    cargo fmt -- --check

# Run Clippy with warnings treated as errors
clippy:
    cargo clippy -- -D warnings

# Run Prettier on Markdown files
prettier:
    npx prettier --write "**/*.md"

# Check Prettier formatting
prettier-check:
    npx prettier --check "**/*.md"

# CI task: check formatting and linting and all tests
ci: fmt-check clippy prettier-check test-python test-rust

# Run python tests
test-python:
    echo "Build python library"
    maturin build --manifest-path bindings/python/Cargo.toml
    echo "Run python tests"
    sh bindings/python/tests/run.sh

# Run rust tests
test-rust:
    cargo test --all-features
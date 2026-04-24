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

# Run python tests.
# Requires an active Python venv (run `source bindings/python/.venv/bin/activate`
# first). `maturin develop` builds the extension and installs it into the
# active venv so the tests exercise the current Rust code, not a stale wheel.
test-python:
    maturin develop --manifest-path bindings/python/Cargo.toml
    sh bindings/python/tests/run.sh

# Run rust tests
test-rust:
    cargo test --all-features
## Codelist Base Library

A Rust library providing core functionality for medical coding systems (SNOMED
CT, ICD-10, OPCS-4). This library serves as the foundation for a _future_ suite
of medical coding tools (such as a validator).

### Features

- Core `CodeList` struct for managing collections of medical codes
- Efficient storage and retrieval using HashSet
- Support for code and term pairs
- Serialization support via serde

### Usage

Add to your Cargo.toml:

```toml
[dependencies]
codelist-rs = { path = "../codelist-rs" }
```

### Development Sandbox

The `examples/sandbox.rs` file is your playground for experimenting with the
library. It's designed for:

- Quick experiments with new features
- Testing how the API feels in practice
- Understanding how components work together

Use it when you want to:

- Try out a new method before implementing it properly
- Debug a specific behavior
- Learn how the library works

But please don't use it for:

- Production code
- Unit tests - these should go in the individual files at the bottom where the
  code you are testing is.
- Documentation

Running the sandbox:

```bash
cargo run --example sandbox
```

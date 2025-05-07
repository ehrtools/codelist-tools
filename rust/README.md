## Overview

This folder contains the small rust projects that we are using to form the basis of our tool. The projects are:

- `codelist-rs`: A base library for working with codelists
- `codelist-validator-rs`: A library for validating codelists

These relatively lightweight projects will be wrapped up in one tool when invoked from Python or R, but are kept 
separate for ease of development and testing.

## Development

To run a particular project, run the following command from the project's directory:

```bash
cargo run --bin <project_name>
```

For example, to run the `codelist-rs` project, run the following command:

```bash
cargo run -p codelist-rs
```

## Testing
Each project has its own set of tests. To run the tests for a particular project, run tests using the following command:

```bash
cd <project_name>
cargo test
```

or to run the tests for all projects, run the following command:

```bash
cargo test
```
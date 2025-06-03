## Python Bindings

This folder contains the Python bindings for the Rust projects in the `rust`
folder. The bindings are generated using `PyO3`, which allows us to create
Python modules from Rust code. The bindings are generated using the `maturin`
tool, which is a tool for building and publishing Rust packages as Python
wheels. This allows us to `pip install` our python package which under the hood
is calling the Rust code.

## Structure

The folder contains the following files:

- `Cargo.toml`: The Rust package manifest file. This file contains the metadata
  for the package, including the dependencies and build settings.
- `pyproject.toml`: The Python package manifest file. This tells `maturin` how
  to build the package and what dependencies are required.
- `src/lib.rs`: The Rust code for the package. This file contains the Rust code
  that is exposed to Python. It is here that we bring to the surface the
  specific modules and functions that we want to expose and be callable from
  Python.
- `src/codelist.rs`: The Rust code for the `codelist` module. This file contains
  the codelist struct and the functions that operate on it. It uses an `inner`
  to interact with the `codelist-rs` library, and is a Python wrapper around the
  `codelist-rs` and `codelist-validator-rs` libraries. This is where we will add
  the specific API calls that we want to expose to Python.

One thing that you will notice is that the `src/lib.rs` file is very small - we
are only exposing the `Codelist` struct and its methods. `codelist.rs` is where
the bulk of the work is done. This is because we want to specify the exact API
that we want to be available.

## Development

To build the package, run the following command from the root of the folder:

```bash
maturin develop
```

This will build the package and install it in your local Python environment. You
can then import the package in Python and use it as you would any other Python
package. You should use a virtual environment for ease of development. If you
run `pip list`, you should see the package listed as `codelist` with the version
number.

You can also run tests using the following command. Ensure the most up to date
version of the package with the most recent changes has been built before
running the tests.

```bash
python -m unittest discover
```

## Codelist Tools

This is a Rust library for working with medical codelists (SNOMED, ICD10, OPCS).
It provides fast and memory-efficient operations on medical codelists. There is
a Python and R interface that researchers can use.

### Structure of Project

The project is divided into two main components:

1. **rust**: The libraries written in Rust.
2. **python**: Python bindings for the Rust library.

#### Rust Library

The Rust library is a collection of modules for working with medical codelists:

- **codelists-rs**: Base library for working with codelists. This has basic
  structs and functions for working with codelists.
- **codelist-validator-rs**: Library for validating codelists. This is under
  development.

## Formatting

### Cargo Format

We are using `cargo fmt` to format the Rust code in this project. This ensures
that the code is consistently formatted and adheres to the Rust style
guidelines. A check is run on every pull request to ensure that the code is
formatted correctly, via preflight checks. If you want to format the code, you
can run the following command from the root of the project:

```bash
cargo fmt
```

### Prettier for Markdown

We use [Prettier](https://prettier.io/) to ensure consistent formatting of
Markdown files, including this README. Prettier helps maintain readability and
clean diffs by enforcing a standard style for line length, wrapping, and
spacing.

### Why use Prettier?

- Keeps formatting consistent across contributors
- Prevents unnecessary whitespace-only changes in diffs
- Makes Markdown files easier to read and maintain

### How to install Prettier

If you're using `npm` or `yarn`, you can install Prettier locally in the
project:

```bash
npm install --save-dev prettier
```

Or install it globally:

```bash
npm install -g prettier
```

### Editor Integration

Once installed, you can enable Prettier in your IDE for automatic formatting.

#### RustRover

1. Go to **Settings** → `Tools` → `Actions on Save`.
2. Enable **Prettier** and configure it to include all `.md` files.
3. After this setup, you can right-click the README file and select **Reformat
   File** to apply Prettier formatting.

#### VS Code

In VS Code, it's even easier to configure Prettier to run on save:

1. Install the Prettier extension.
2. Open **Settings** and search for `Format On Save`.
3. Enable **Editor: Format On Save**.
4. Optionally, add a `.prettierrc` file to customize the formatting rules.

## Development Utilities

We use a [`justfile`](https://github.com/casey/just) to define common tasks for
development and CI.

### How to Install `just`

Install `just` using
[the instructions here](https://github.com/casey/just#installation), or with a
package manager:

```bash
# macOS (Homebrew)
brew install just

# Debian or Ubuntu
sudo apt install just

# Arch Linux
pacman -S just
```

### How to Use It

To run a task, use:

```bash
just <recipe>
```

For example:

```bash
just ci
```

This will run the CI tasks defined in the `justfile`, which include running
tests, formatting checks, and linting. You can also run individual tasks like

```bash
just fmt
```

These tasks help ensure consistent code style and formatting before committing
or opening a pull request.

To see all available tasks, run:

```bash
just --list
```

Make sure you run `just ci` before opening a pull request to ensure that all
tasks pass. This will help catch any issues early and ensure that the code is
formatted correctly and passes all tests.

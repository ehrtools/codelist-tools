**Bindings** This directory contains language-specific bindings for interacting
with the core functionalities of the Codelist Tools project. These bindings
serve as adapters that enable the use of the underlying Python-based code list
tools in different environments, programming languages, or frameworks.

**Purpose** The purpose of this folder is to:

Provide interfaces or wrappers to allow other languages and systems to call the
core Python functions.

Facilitate interoperability in multi-language pipelines (e.g. R & Python).

Ensure modularity and reuse of core logic without duplicating code.

**Structure** Each subfolder within bindings/ corresponds to a different
language or binding target.

**Available Bindings**

- Python: Uses Py03 package to provide access to underlying Rust
- R (r/): Uses the extendR package to provide access to underlying Rust

(Additional bindings will be listed here as they are developed.)

**Usage** To use a binding, navigate to the appropriate subfolder and follow the
instructions in its own README.md or documentation.

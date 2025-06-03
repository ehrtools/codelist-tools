# codelist-builder-rs

A Rust library for building medical codelist files from source data. This is a
complementary library to `codelist-rs`, which provides the core functionality in
terms of metadata and holding the codelist data.

## Features

### TODO:

- Load SNOMEDCT from Builder files into a reference dataset. We should do this
  initially for the UK SNOMED CT release, but the library should be generic
  enough to handle any SNOMED CT release, which might have different data files.
- Load ICD-10 from Builder files into a reference dataset
- Load OPCS-4 from Builder files into a reference dataset
- Create a way of building a codelist from a set of Builder database (probably
  Surreal)
- Expose logging functionality to the user, so they can see what is happening
  and can recreate the codelist from the Builder database given the
  instructions.
- Create ways of searching for relevant codes:
  - Search by term
  - Search by child code
  - Search by vector search with terms as embeddings

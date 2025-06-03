# codelist-validator

This is a rust package with Python bindings for validating codelists (SNOMED,
ICD10, OPCS)

## Aims

`codelists-validator` is a lightweight library for validating and manipulating
medical coding standards including SNOMED CT, ICD-10, and OPCS-4. Written in
Rust with Python bindings (via PyO3), it provides fast and memory-efficient
operations on medical codelists while maintaining a Pythonic interface.

## Features

- Validate medical codes against standard formats:
  - SNOMED CT codes
  - ICD-10 codes (including support for X-codes)
  - OPCS-4 codes
- Load and manipulate codelists from CSV files
- Support for custom code and term column names
- ICD-10 specific features:
  - Optional automatic X-code generation
  - 3-digit code truncation option
- Comprehensive error handling for invalid codes and data formats

## Installation for python

```bash
pip install codelist-validator
```

## Usage

### Basic Usage

```python
from codelist_validator import Codelist

# Load an ICD-10 codelist
codelist = Codelist(
    path="path/to/codelist.csv",
    codelist_type="ICD10",
    code_column="code",
    term_column="term"
)

# Load with X-codes generation
codelist_with_x = Codelist(
    path="path/to/codelist.csv",
    codelist_type="ICD10",
    add_x_codes=True
)

# Load with 3-digit truncation
truncated_codelist = Codelist(
    path="path/to/codelist.csv",
    codelist_type="ICD10",
    icd10_3_digit_only=True
)
```

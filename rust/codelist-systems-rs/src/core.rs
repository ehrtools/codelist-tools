//! The core trait every coding system implements.

use codelist_rs::types::{Code, CodeSystemId, NormalizedCode};

use crate::errors::{SystemError, ValidationError};

/// A coding system.
///
/// Implementors are typically zero-sized marker types (e.g. `Icd10`,
/// `Snomed`). The trait is object-unsafe on purpose — dispatch happens
/// by pattern-matching on the enum tag stored inside `CodeList`.
pub trait CodingSystem {
    /// Stable identifier for the system.
    const ID: CodeSystemId;

    /// Produce a canonical form of the code (e.g. upper-case, strip
    /// surrounding whitespace).
    fn normalize(code: &Code) -> Result<NormalizedCode, SystemError>;

    /// Check that a normalised code satisfies the system's syntax rules.
    fn validate_syntax(code: &NormalizedCode) -> Result<(), ValidationError>;
}

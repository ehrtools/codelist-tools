//! Coding systems (ICD10, SNOMED, OPCS, CTV3) as data.
//!
//! Each system is a zero-sized marker type implementing
//! [`core::CodingSystem`]. Optional capabilities (truncation, hierarchy,
//! mapping) are expressed as subtraits in [`capabilities`].

pub mod errors;

pub use crate::errors::{SystemError, ValidationError};

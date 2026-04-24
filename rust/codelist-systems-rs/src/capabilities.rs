//! Optional capabilities that some coding systems provide.
//!
//! Each trait is a supertrait of [`CodingSystem`] so capability methods
//! have access to the system's `ID` and validation rules. A system opts
//! into a capability by implementing the corresponding trait; absence
//! of an impl means the capability doesn't apply to that system (e.g.
//! SNOMED codes are not `X`-extensible).

use codelist_rs::types::NormalizedCode;

use crate::core::CodingSystem;

/// A coding system whose codes can be truncated to a shorter, still-valid
/// code. The target depth is baked into the impl (ICD10 truncates to 3
/// characters); callers never pass one.
pub trait Truncatable: CodingSystem {
    /// Whether `code` is longer than the system's canonical truncation
    /// depth and therefore a candidate for [`truncate`](Self::truncate).
    fn is_truncatable(code: &NormalizedCode) -> bool;

    /// Produce the truncated form. Callers should first check
    /// [`is_truncatable`](Self::is_truncatable); calling `truncate` on a
    /// code that isn't truncatable returns the input unchanged.
    fn truncate(code: &NormalizedCode) -> NormalizedCode;
}

/// A coding system where appending `X` to a code is a valid transformation
/// (currently ICD10 only).
pub trait XExtensible: CodingSystem {
    /// Returns `true` if appending `X` to `code` is a valid system
    /// transformation. For ICD10, this is true when `code` has exactly
    /// 3 characters.
    fn is_x_addable(code: &NormalizedCode) -> bool;

    /// Returns the code with `X` appended. Callers should first check
    /// [`is_x_addable`](Self::is_x_addable).
    fn add_x(code: &NormalizedCode) -> NormalizedCode;
}

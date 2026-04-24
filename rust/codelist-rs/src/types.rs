//! Public types for the codelist crate.
//!
//! Includes:
//! - [`CodeListType`] — the runtime tag identifying which coding system a
//!   [`crate::codelist::CodeList`] belongs to.
//! - Newtype wrappers such as [`Code`] that give compile-time type safety to
//!   values that would otherwise just be `String`.
//!
//! The newtypes do NOT implement `Deref<Target = str>`. Callers convert
//! explicitly with `as_str()` (or via `String::from(x)` when they need an
//! owned `String`). This keeps "is this a `Term` or a `Code`?" answerable
//! at compile time.

use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::errors::CodeListError;

/// A raw clinical code as given by a caller, before any coding-system
/// specific normalisation.
///
/// Construction is infallible — `Code` carries no content invariants
/// beyond "it is a `String` that a caller intended as a code". Content
/// validation happens later, against the relevant `CodingSystem` (see the
/// `codelist-systems-rs` crate) or at the point it's added to a `CodeList`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Code(String);

impl Code {
    /// Borrow the inner string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for Code {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for Code {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<Code> for String {
    fn from(c: Code) -> Self {
        c.0
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A code after a `CodingSystem` has normalised it (e.g. uppercased,
/// trimmed). Semantically distinct from [`Code`]: the system guarantees
/// this value has been through `CodingSystem::normalize`. Construct via
/// `From<String>` inside system impls; elsewhere, obtain one by calling
/// the system's normalize function.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NormalizedCode(String);

impl NormalizedCode {
    /// Borrow the inner string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for NormalizedCode {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for NormalizedCode {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<NormalizedCode> for String {
    fn from(n: NormalizedCode) -> Self {
        n.0
    }
}

impl fmt::Display for NormalizedCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A human-readable label associated with a [`Code`] (e.g. "Gonorrhoea").
/// Kept separate from `Code` so a function signature like
/// `fn add(code: Code, term: Term)` is unambiguous at the call site —
/// the two cannot be accidentally transposed.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Term(String);

impl Term {
    /// Borrow the inner string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for Term {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for Term {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<Term> for String {
    fn from(t: Term) -> Self {
        t.0
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Identifier for a coding system (ICD10, SNOMED, ...). Uses `&'static str`
/// so each system can expose a `const ID: CodeSystemId` on its
/// `CodingSystem` impl.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CodeSystemId(pub &'static str);

impl fmt::Display for CodeSystemId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

/// Someone who contributed to a codelist (e.g. a name or handle).
/// Distinct from an arbitrary `String` so contributor collections cannot
/// accidentally mix with other string-typed metadata fields.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Contributor(String);

impl Contributor {
    /// Borrow the inner string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for Contributor {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for Contributor {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<Contributor> for String {
    fn from(c: Contributor) -> Self {
        c.0
    }
}

impl fmt::Display for Contributor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Stable identifier for a codelist. Distinct from an arbitrary `String`
/// so cross-codelist references (e.g. merge records) are typed rather
/// than stringly-typed.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CodeListId(String);

impl CodeListId {
    /// Borrow the inner string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for CodeListId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for CodeListId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<CodeListId> for String {
    fn from(id: CodeListId) -> Self {
        id.0
    }
}

impl fmt::Display for CodeListId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Enum to represent the different types of codelists
///
/// # Variants
/// * `ICD10` - The ICD10 codelist
/// * `SNOMED` - The SNOMED codelist
/// * `OPCS` - The OPCS codelist
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum CodeListType {
    ICD10,
    SNOMED,
    OPCS,
    CTV3,
}

impl CodeListType {
    /// Is the `CodeListType` able to be truncated
    // TODO: Make it a trait?
    // Right now truncation only applies to ICD10 code lists, but ICD11 is coming.
    pub fn is_truncatable(&self) -> bool {
        matches!(self, CodeListType::ICD10)
    }

    /// Is the `CodeListType` able to have X added
    // TODO: Make it a trait?
    // Right now adding X only applies to ICD10 code lists, but ICD11 is coming.
    pub fn is_x_addable(&self) -> bool {
        matches!(self, CodeListType::ICD10)
    }
}

impl FromStr for CodeListType {
    type Err = CodeListError;
    /// Convert a string to a CodeListType
    ///
    /// # Arguments
    /// * `s` - The string to convert to a CodeListType
    ///
    /// # Returns
    /// * `Result<CodeListType, CodeListError>` - The CodeListType or a
    ///   CodeListError
    ///
    /// # Errors
    /// * `CodeListError::InvalidCodeListType` - If the string is not a valid
    ///   CodeListType
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "icd10" => Ok(CodeListType::ICD10),
            "snomed" => Ok(CodeListType::SNOMED),
            "opcs" => Ok(CodeListType::OPCS),
            "ctv3" => Ok(CodeListType::CTV3),
            invalid_string => Err(CodeListError::invalid_code_list_type(invalid_string)),
        }
    }
}

/// Implement `Display` for `CodeListType` so it automatically supports
/// `to_string()`
impl fmt::Display for CodeListType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            CodeListType::ICD10 => "ICD10",
            CodeListType::SNOMED => "SNOMED",
            CodeListType::OPCS => "OPCS",
            CodeListType::CTV3 => "CTV3",
        };
        write!(f, "{s}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_round_trips_string_borrow_and_owned() {
        let c = Code::from("A54".to_string());
        assert_eq!(c.as_str(), "A54");
        assert_eq!(String::from(c), "A54");
    }

    #[test]
    fn code_from_str_and_string_are_equivalent() {
        assert_eq!(Code::from("A54"), Code::from("A54".to_string()));
        assert_eq!(Code::from("A54").as_str(), "A54");
        assert_eq!(Code::from("A54".to_string()).as_str(), "A54");
    }

    #[test]
    fn code_serialises_transparently_as_json_string() {
        let c = Code::from("A54");
        let json = serde_json::to_string(&c).unwrap();
        assert_eq!(json, "\"A54\"");
        let back: Code = serde_json::from_str("\"A54\"").unwrap();
        assert_eq!(back, c);
    }

    #[test]
    fn code_displays_as_inner() {
        assert_eq!(Code::from("A54").to_string(), "A54");
    }

    #[test]
    fn test_from_str() {
        assert!(matches!(CodeListType::from_str("icd10"), Ok(CodeListType::ICD10)));
        assert!(matches!(CodeListType::from_str("snomed"), Ok(CodeListType::SNOMED)));
        assert!(matches!(CodeListType::from_str("opcs"), Ok(CodeListType::OPCS)));
        assert!(matches!(CodeListType::from_str("ctv3"), Ok(CodeListType::CTV3)));
        assert!(matches!(CodeListType::from_str("invalid"), 
            Err(CodeListError::InvalidCodeListType { name }) if name == "invalid"));
    }

    #[test]
    fn test_from_str_case_insensitive() {
        assert!(matches!(CodeListType::from_str("ICD10"), Ok(CodeListType::ICD10)));
        assert!(matches!(CodeListType::from_str("SNOMED"), Ok(CodeListType::SNOMED)));
        assert!(matches!(CodeListType::from_str("OPCS"), Ok(CodeListType::OPCS)));
        assert!(matches!(CodeListType::from_str("CTV3"), Ok(CodeListType::CTV3)));
    }

    #[test]
    fn test_to_string() {
        assert_eq!(CodeListType::ICD10.to_string(), "ICD10");
        assert_eq!(CodeListType::SNOMED.to_string(), "SNOMED");
        assert_eq!(CodeListType::OPCS.to_string(), "OPCS");
        assert_eq!(CodeListType::CTV3.to_string(), "CTV3");
    }

    #[test]
    fn normalized_code_round_trips() {
        let n = NormalizedCode::from("A54");
        assert_eq!(n.as_str(), "A54");
        assert_eq!(String::from(n), "A54");
    }

    #[test]
    fn normalized_code_serialises_transparently_as_json_string() {
        let n = NormalizedCode::from("A54");
        assert_eq!(serde_json::to_string(&n).unwrap(), "\"A54\"");
        let back: NormalizedCode = serde_json::from_str("\"A54\"").unwrap();
        assert_eq!(back, n);
    }

    #[test]
    fn term_round_trips() {
        let t = Term::from("Gonorrhoea");
        assert_eq!(t.as_str(), "Gonorrhoea");
        assert_eq!(String::from(t), "Gonorrhoea");
    }

    #[test]
    fn term_serialises_transparently_as_json_string() {
        let t = Term::from("Gonorrhoea");
        assert_eq!(serde_json::to_string(&t).unwrap(), "\"Gonorrhoea\"");
        let back: Term = serde_json::from_str("\"Gonorrhoea\"").unwrap();
        assert_eq!(back, t);
    }

    #[test]
    fn code_system_id_displays() {
        assert_eq!(CodeSystemId("ICD10").to_string(), "ICD10");
    }

    #[test]
    fn contributor_round_trips() {
        let c = Contributor::from("Caroline Morton");
        assert_eq!(c.as_str(), "Caroline Morton");
        assert_eq!(String::from(c), "Caroline Morton");
    }

    #[test]
    fn contributor_serialises_transparently_as_json_string() {
        let c = Contributor::from("Caroline Morton");
        assert_eq!(serde_json::to_string(&c).unwrap(), "\"Caroline Morton\"");
        let back: Contributor = serde_json::from_str("\"Caroline Morton\"").unwrap();
        assert_eq!(back, c);
    }

    #[test]
    fn codelist_id_round_trips() {
        let id = CodeListId::from("my-codelist-123");
        assert_eq!(id.as_str(), "my-codelist-123");
        assert_eq!(String::from(id), "my-codelist-123");
    }

    #[test]
    fn codelist_id_serialises_transparently_as_json_string() {
        let id = CodeListId::from("my-codelist-123");
        assert_eq!(serde_json::to_string(&id).unwrap(), "\"my-codelist-123\"");
        let back: CodeListId = serde_json::from_str("\"my-codelist-123\"").unwrap();
        assert_eq!(back, id);
    }
}

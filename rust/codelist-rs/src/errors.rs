use std::fmt;

#[derive(Debug, PartialEq)]
pub enum CodeListError {
    // InvalidSNOMEDCodeError,
    // InvalidICD10CodeError,
    // InvalidOPCSCodesError,
    // RepeatedCodeError,
    // InvalidDataShapeError,
    // InvalidProcessingRequest,
    // InvalidCodeListError,
    InvalidCodeListType(String),
}

impl fmt::Display for CodeListError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCodeListType(invalid_type) => write!(f, "Invalid codelist type provided: {}", invalid_type),
            // TODO
        }
    }
}
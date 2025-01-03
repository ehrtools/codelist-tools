use codelist_rs::codelist::CodeList;

pub trait OPCSValidator {
    fn validate_code(&self, code: &str) -> bool; // for 1 code
    fn validate_all_code(&self);
}

// Implement the `OPCSValidator` trait for `CodeList`
impl OPCSValidator for CodeList {
    fn validate_code(&self, code: &str) -> bool {
        //TODO
        true
    }
    fn validate_all_code(&self) {

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]

}
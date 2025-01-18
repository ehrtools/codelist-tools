pub struct ValidationAndReview {
    pub reviewed: Option<bool>,
    pub reviewer: Option<String>,
    pub review_date: Option<String>,
    pub status: Option<String>,
    pub validation_notes: Option<String>,
}

// new
// update reviewed (default is false)
// Add reviewer (default is none)
// Add review date
// add status
// update status
// add validation notes
// update validation notes

#[cfg(test)]
mod tests {
    use super::*;
}

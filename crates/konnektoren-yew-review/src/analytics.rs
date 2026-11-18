use konnektoren_yew_web_analytics::Parameters;

/// Analytics event sent after the API accepted a review.
pub const REVIEW_SUBMITTED_EVENT: &str = "challenge_review_submitted";
/// Analytics event sent when a review could not be submitted.
pub const REVIEW_FAILED_EVENT: &str = "challenge_review_failed";

/// Event parameters for a review. The comment text is never sent, only whether one was given.
pub fn review_parameters(challenge_id: &str, rating: u8, has_comment: bool) -> Parameters {
    let mut parameters = Parameters::new();
    parameters.insert("challenge_id".into(), challenge_id.into());
    parameters.insert("rating".into(), rating.into());
    parameters.insert("has_comment".into(), i32::from(has_comment).into());
    parameters
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn review_parameters_never_contain_the_comment_text() {
        let parameters = review_parameters("music:c-major", 4, true);
        assert_eq!(parameters["challenge_id"], "music:c-major");
        assert_eq!(parameters["rating"], 4);
        assert_eq!(parameters["has_comment"], 1);
        assert_eq!(parameters.len(), 3);
    }

    #[test]
    fn review_parameters_mark_missing_comments() {
        assert_eq!(review_parameters("id", 1, false)["has_comment"], 0);
    }
}

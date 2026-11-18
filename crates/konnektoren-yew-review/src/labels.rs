/// User-facing texts of the review components, translated by the host app.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReviewLabels {
    pub title: String,
    pub comment_placeholder: String,
    pub submit: String,
    pub submitting: String,
    pub thank_you: String,
    pub loading: String,
}

impl Default for ReviewLabels {
    fn default() -> Self {
        Self {
            title: "Rate this Challenge".into(),
            comment_placeholder: "Leave a comment".into(),
            submit: "Submit".into(),
            submitting: "Submitting...".into(),
            thank_you: "Thank you for your review!".into(),
            loading: "Loading...".into(),
        }
    }
}

impl ReviewLabels {
    /// Translate every default (English) label with the host's translator.
    pub fn translated(t: impl Fn(&str) -> String) -> Self {
        let default = Self::default();
        Self {
            title: t(&default.title),
            comment_placeholder: t(&default.comment_placeholder),
            submit: t(&default.submit),
            submitting: t(&default.submitting),
            thank_you: t(&default.thank_you),
            loading: t(&default.loading),
        }
    }
}

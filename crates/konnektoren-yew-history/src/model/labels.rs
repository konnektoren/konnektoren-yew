/// Texts of a history page, as translation keys.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HistoryLabels {
    /// Page heading and list name, e.g. "Exercise history".
    pub title: &'static str,
    /// One sentence below the heading; empty for none.
    pub intro: &'static str,
    /// e.g. "Search exercises".
    pub search_label: &'static str,
    /// e.g. "Exercise name".
    pub search_placeholder: &'static str,
    /// After the count, e.g. "Attempts".
    pub noun: &'static str,
    /// Nothing recorded yet, e.g. "No challenges completed yet."
    pub empty: &'static str,
    /// Nothing matches, e.g. "No attempts match these filters."
    pub no_match: &'static str,
}

impl Default for HistoryLabels {
    fn default() -> Self {
        Self {
            title: "History",
            intro: "",
            search_label: "Search",
            search_placeholder: "Name",
            noun: "Entries",
            empty: "Nothing here yet.",
            no_match: "Nothing matches these filters.",
        }
    }
}

/// What an app message changed; see [`History::update`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HistoryChange {
    /// Nothing changed. (The page renders once per batch of messages anyway; `None`
    /// and `Render` only differ in intent.)
    None,
    /// Render again.
    Render,
    /// The listed records changed, e.g. through [`History::admits`]: render from the
    /// first page.
    Refilter,
}

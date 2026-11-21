//! Fuzzy FAQ search: a word index over questions, answers and tags, matched
//! with normalized Levenshtein similarity.

use konnektoren_rs::faq::{Faq, FaqData};
use konnektoren_rs::i18n::Language;
use std::cell::OnceCell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use strsim::normalized_levenshtein;

/// Minimum similarity for a query word to match an index word, and for a
/// word to be offered as suggestion.
const SIMILARITY_THRESHOLD: f64 = 0.7;

/// Word index over a [`FaqData`].
///
/// Indexes the English question and answer, their translation for the given
/// language and the tags. Results keep the YAML order for equal scores.
#[derive(Debug, Clone)]
pub struct FaqSearch {
    data: Arc<FaqData>,
    /// Lowercase word → indices into `data.faqs`, sorted and deduplicated.
    index: BTreeMap<String, Vec<usize>>,
}

impl FaqSearch {
    pub fn new(data: Arc<FaqData>, language: &Language) -> Self {
        let mut index: BTreeMap<String, Vec<usize>> = BTreeMap::new();

        for (position, faq) in data.faqs.iter().enumerate() {
            let texts = [
                faq.question.as_str(),
                faq.answer.as_str(),
                faq.get_localized_question(language),
                faq.get_localized_answer(language),
            ];
            let words = texts
                .into_iter()
                .flat_map(tokenize)
                .chain(faq.tags.iter().flat_map(|tag| tokenize(tag.as_str())));

            for word in words {
                let positions = index.entry(word).or_default();
                if positions.last() != Some(&position) {
                    positions.push(position);
                }
            }
        }

        Self { data, index }
    }

    /// All FAQs in YAML order.
    pub fn all(&self) -> Vec<Faq> {
        self.data.faqs.clone()
    }

    /// FAQs matching any word of `query`, best match first.
    ///
    /// An empty query returns no results; use [`FaqSearch::all`] to list everything.
    pub fn search(&self, query: &str) -> Vec<Faq> {
        let mut scores: HashMap<usize, f64> = HashMap::new();

        for query_word in tokenize(query) {
            for (word, positions) in &self.index {
                let similarity = similarity(&query_word, word);
                if similarity > SIMILARITY_THRESHOLD {
                    for position in positions {
                        *scores.entry(*position).or_default() += similarity;
                    }
                }
            }
        }

        let mut ranked: Vec<(usize, f64)> = scores.into_iter().collect();
        ranked.sort_by(|(a_position, a_score), (b_position, b_score)| {
            b_score
                .partial_cmp(a_score)
                .unwrap_or(Ordering::Equal)
                .then(a_position.cmp(b_position))
        });

        ranked
            .into_iter()
            .map(|(position, _)| self.data.faqs[position].clone())
            .collect()
    }

    /// The index word closest to `query`, for a "Did you mean" hint.
    pub fn suggestion(&self, query: &str) -> Option<String> {
        let query = query.trim().to_lowercase();
        if query.is_empty() {
            return None;
        }

        let mut best: Option<(&String, f64)> = None;
        for word in self.index.keys() {
            let similarity = normalized_levenshtein(&query, word);
            if !word.contains(&query) && similarity <= SIMILARITY_THRESHOLD {
                continue;
            }
            if best.is_none_or(|(_, best_similarity)| similarity > best_similarity) {
                best = Some((word, similarity));
            }
        }
        best.map(|(word, _)| word.clone())
    }
}

/// A [`FaqSearch`] that builds its index on first use.
///
/// Listing everything with [`LazyFaqSearch::all`] does not build the index,
/// so a page that is only viewed never pays for it.
#[derive(Debug)]
pub struct LazyFaqSearch {
    data: Arc<FaqData>,
    language: Language,
    search: OnceCell<FaqSearch>,
}

impl LazyFaqSearch {
    pub fn new(data: Arc<FaqData>, language: Language) -> Self {
        Self {
            data,
            language,
            search: OnceCell::new(),
        }
    }

    /// All FAQs in YAML order, without building the index.
    pub fn all(&self) -> Vec<Faq> {
        self.data.faqs.clone()
    }

    /// See [`FaqSearch::search`]; builds the index on the first call.
    pub fn search(&self, query: &str) -> Vec<Faq> {
        self.index().search(query)
    }

    /// See [`FaqSearch::suggestion`]; builds the index on the first call.
    pub fn suggestion(&self, query: &str) -> Option<String> {
        self.index().suggestion(query)
    }

    /// Whether the index has been built yet.
    pub fn is_indexed(&self) -> bool {
        self.search.get().is_some()
    }

    fn index(&self) -> &FaqSearch {
        self.search
            .get_or_init(|| FaqSearch::new(self.data.clone(), &self.language))
    }
}

/// Lowercase words, split at everything that is not a letter or digit
/// (punctuation, emoji, whitespace).
fn tokenize(text: &str) -> impl Iterator<Item = String> + '_ {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_lowercase)
}

fn similarity(a: &str, b: &str) -> f64 {
    if a == b {
        1.0
    } else {
        normalized_levenshtein(a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const YAML: &str = r#"
faqs:
  - id: general-what-is-konnektoren
    timestamp: "2025-01-29T19:19:02+01:00"
    question: "🤔 What is Konnektoren?"
    answer: "Konnektoren helps you practice German connectors."
    tags: ["general", "about"]
    i18n:
      "🤔 What is Konnektoren?":
        de: "🤔 Was ist Konnektoren?"
      "Konnektoren helps you practice German connectors.":
        de: "Konnektoren hilft dir, deutsche Konnektoren zu üben."
  - id: general-theme-switch
    timestamp: "2026-07-03T00:00:00+01:00"
    question: "🌗 Can I switch between light and dark mode?"
    answer: "Yes, head to Settings and pick your theme."
    tags: ["settings", "dark-mode"]
    i18n:
      "🌗 Can I switch between light and dark mode?":
        de: "🌗 Kann ich zwischen hellem und dunklem Modus wechseln?"
  - id: privacy-data
    timestamp: "2026-07-03T00:00:00+01:00"
    question: "Where is my data stored?"
    answer: "Your progress stays in your browser. Settings too."
    tags: ["privacy"]
"#;

    fn data() -> Arc<FaqData> {
        Arc::new(serde_yaml::from_str(YAML).unwrap())
    }

    fn ids(faqs: &[Faq]) -> Vec<&str> {
        faqs.iter().map(|faq| faq.id.as_str()).collect()
    }

    fn search(language: &str) -> FaqSearch {
        FaqSearch::new(data(), &Language::from_code(language))
    }

    #[test]
    fn tokenize_strips_punctuation_and_emoji() {
        let words: Vec<_> = tokenize("🤔 What is Konnektoren?").collect();
        assert_eq!(words, ["what", "is", "konnektoren"]);
    }

    #[test]
    fn exact_word() {
        assert_eq!(
            ids(&search("en").search("Konnektoren")),
            ["general-what-is-konnektoren"]
        );
    }

    #[test]
    fn typo() {
        assert_eq!(
            ids(&search("en").search("konektoren")),
            ["general-what-is-konnektoren"]
        );
    }

    #[test]
    fn tag_word() {
        assert_eq!(ids(&search("en").search("privacy")), ["privacy-data"]);
        assert_eq!(ids(&search("en").search("dark")), ["general-theme-switch"]);
    }

    #[test]
    fn localized_text() {
        assert_eq!(
            ids(&search("de").search("dunklem")),
            ["general-theme-switch"]
        );
        assert!(search("en").search("dunklem").is_empty());
    }

    #[test]
    fn ranks_by_score_then_yaml_order() {
        // "settings" is in both answers; "theme" only in the second FAQ.
        assert_eq!(
            ids(&search("en").search("settings theme")),
            ["general-theme-switch", "privacy-data"]
        );
        assert_eq!(
            ids(&search("en").search("settings")),
            ["general-theme-switch", "privacy-data"]
        );
    }

    #[test]
    fn empty_query() {
        assert!(search("en").search("  ").is_empty());
        assert_eq!(search("en").suggestion(""), None);
    }

    #[test]
    fn all_keeps_yaml_order() {
        assert_eq!(
            ids(&search("en").all()),
            [
                "general-what-is-konnektoren",
                "general-theme-switch",
                "privacy-data"
            ]
        );
    }

    #[test]
    fn suggestion() {
        assert_eq!(
            search("en").suggestion("konektoren").as_deref(),
            Some("konnektoren")
        );
        assert_eq!(search("en").suggestion("xyzxyz"), None);
    }

    #[test]
    fn lazy_builds_index_on_first_search() {
        let lazy = LazyFaqSearch::new(data(), Language::from_code("en"));
        assert_eq!(lazy.all().len(), 3);
        assert!(!lazy.is_indexed());
        assert_eq!(ids(&lazy.search("privacy")), ["privacy-data"]);
        assert!(lazy.is_indexed());
    }
}

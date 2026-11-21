use super::FaqList;
use crate::asset::{FAQ_ASSET_PATH, use_faq_data};
use crate::search::LazyFaqSearch;
use konnektoren_rs::faq::FaqData;
use konnektoren_rs::i18n::Language;
use web_sys::HtmlInputElement;
use yew::prelude::*;

/// Translated texts of [`FaqSearchPanel`]; the defaults are English.
#[derive(Clone, PartialEq, Debug)]
pub struct FaqSearchLabels {
    pub placeholder: AttrValue,
    pub search: AttrValue,
    pub did_you_mean: AttrValue,
    pub all_faqs: AttrValue,
    pub matching_faqs: AttrValue,
    /// Results count as "`found` 3 `relevant_faqs`".
    pub found: AttrValue,
    pub relevant_faqs: AttrValue,
}

impl Default for FaqSearchLabels {
    fn default() -> Self {
        Self {
            placeholder: "Type to search FAQs...".into(),
            search: "Search".into(),
            did_you_mean: "Did you mean".into(),
            all_faqs: "All FAQs".into(),
            matching_faqs: "Matching FAQs".into(),
            found: "Found".into(),
            relevant_faqs: "relevant FAQs".into(),
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct FaqSearchPanelProps {
    /// FAQ file relative to the asset root.
    #[prop_or_else(|| FAQ_ASSET_PATH.into())]
    pub path: AttrValue,
    #[prop_or_else(|| "en".to_string())]
    pub language: String,
    #[prop_or_default]
    pub highlighted_id: Option<String>,
    /// Query to start with, e.g. from `?query=` in the URL.
    #[prop_or_default]
    pub initial_query: AttrValue,
    #[prop_or_default]
    pub labels: FaqSearchLabels,
    /// A FAQ id or tag was clicked in the results.
    #[prop_or_default]
    pub on_select: Callback<String>,
    /// Enter or the search button submitted the query, e.g. to ask an AI
    /// assistant. Without it there is no search button.
    #[prop_or_default]
    pub on_submit: Option<Callback<String>>,
    /// Rendered between the search field and the results, e.g. an AI answer.
    #[prop_or_default]
    pub children: Html,
}

/// Search field with "Did you mean" suggestion and the matching FAQs.
///
/// Loads the FAQs with [`use_faq_data`](crate::use_faq_data) and builds the
/// search index on the first non-empty query. Without the `csr` feature
/// (SSR) the field is disabled and all FAQs are listed.
#[function_component(FaqSearchPanel)]
pub fn faq_search_panel(props: &FaqSearchPanelProps) -> Html {
    let faq_data = use_faq_data(props.path.clone()).data();
    let search_query = use_state(|| props.initial_query.to_string());
    let search = use_memo(
        (faq_data, props.language.clone()),
        |(faq_data, language)| LazyFaqSearch::new(faq_data.clone(), Language::from_code(language)),
    );

    let (results, suggestion) = if search_query.is_empty() {
        (search.all(), None)
    } else {
        (
            search.search(&search_query),
            search.suggestion(&search_query),
        )
    };
    let suggestion = suggestion.filter(|suggestion| *suggestion != *search_query);

    let on_input = {
        let search_query = search_query.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            search_query.set(input.value());
        })
    };

    let submit = {
        let search_query = search_query.clone();
        let on_submit = props.on_submit.clone();
        Callback::from(move |()| {
            let query = search_query.trim();
            if let Some(on_submit) = on_submit.as_ref().filter(|_| !query.is_empty()) {
                on_submit.emit(query.to_string());
            }
        })
    };

    let on_keypress = {
        let submit = submit.clone();
        Callback::from(move |event: KeyboardEvent| {
            if event.key() == "Enter" {
                submit.emit(());
            }
        })
    };

    let on_suggestion_select = {
        let search_query = search_query.clone();
        let suggestion = suggestion.clone().unwrap_or_default();
        Callback::from(move |_: MouseEvent| search_query.set(suggestion.clone()))
    };

    let on_results_click = {
        let on_select = props.on_select.clone();
        Callback::from(move |event: MouseEvent| {
            #[cfg(feature = "csr")]
            if let Some(hash) = crate::selection::hash_from_click(&event) {
                event.stop_propagation();
                on_select.emit(hash);
            }
            #[cfg(not(feature = "csr"))]
            let _ = (event, &on_select);
        })
    };

    let labels = &props.labels;
    let show_button = props.on_submit.is_some() && !search_query.is_empty();

    html! {
        <div class="faq-search">
            <div class="faq-search__container">
                <div class="faq-search__field">
                    <input
                        class="faq-search__input"
                        type="text"
                        value={(*search_query).clone()}
                        disabled={cfg!(not(feature = "csr"))}
                        oninput={on_input}
                        onkeypress={on_keypress}
                        placeholder={labels.placeholder.clone()}
                        aria-label={labels.search.clone()}
                    />
                    if show_button {
                        <button
                            type="button"
                            class="faq-search__button"
                            onclick={submit.reform(|_: MouseEvent| ())}
                            aria-label={labels.search.clone()}
                            title={labels.search.clone()}
                        >
                            <i class="fa-solid fa-magnifying-glass"></i>
                        </button>
                    }
                </div>
                if let Some(suggestion) = suggestion {
                    <div class="faq-search__suggestion" onclick={on_suggestion_select}>
                        {format!("{}: {}", labels.did_you_mean, suggestion)}
                    </div>
                }
            </div>

            {props.children.clone()}

            <div class="faq-search__results" onclick={on_results_click}>
                <h2 class="faq-search__results-title">{
                    if search_query.is_empty() {
                        labels.all_faqs.clone()
                    } else {
                        labels.matching_faqs.clone()
                    }
                }</h2>
                if !search_query.is_empty() {
                    <p class="faq-search__results-count">{
                        format!("{} {} {}", labels.found, results.len(), labels.relevant_faqs)
                    }</p>
                }
                <FaqList
                    faq_data={FaqData { faqs: results }}
                    language={props.language.clone()}
                    highlighted_id={props.highlighted_id.clone()}
                />
            </div>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    fn props(initial_query: &str, on_submit: Option<Callback<String>>) -> FaqSearchPanelProps {
        FaqSearchPanelProps {
            path: FAQ_ASSET_PATH.into(),
            language: "en".to_string(),
            highlighted_id: None,
            initial_query: initial_query.to_string().into(),
            labels: FaqSearchLabels::default(),
            on_select: Callback::noop(),
            on_submit,
            children: Html::default(),
        }
    }

    yew_preview::create_preview!(
        FaqSearchPanel,
        props("", None),
        ("with query", props("theme", None)),
        ("typo suggestion", props("konektoren", None)),
        ("search button", props("dark mode", Some(Callback::noop()))),
    );
}

use super::{ChallengeActions, ChallengeActionsComponent};
use crate::components::ProgressBar;
#[cfg(feature = "effects")]
use crate::effects::ReadText;
use crate::i18n::use_i18n;
use konnektoren_core::challenges::{ChallengeResult, Vocabulary, VocabularyItem};
use konnektoren_core::commands::{ChallengeCommand, Command};
use konnektoren_core::events::Event;
use yew::prelude::*;

// Testable utility functions
pub fn calculate_total_pages(total_items: usize, items_per_page: usize) -> usize {
    if total_items == 0 {
        0
    } else {
        (total_items + items_per_page - 1) / items_per_page // Ceiling division
    }
}

pub fn calculate_items_on_page(
    current_page: usize,
    total_items: usize,
    items_per_page: usize,
) -> usize {
    let start_index = current_page * items_per_page;
    if start_index >= total_items {
        0
    } else {
        let remaining_items = total_items - start_index;
        remaining_items.min(items_per_page)
    }
}

pub fn has_next_page(current_page: usize, total_items: usize, items_per_page: usize) -> bool {
    let start_index_of_next_page = (current_page + 1) * items_per_page;
    start_index_of_next_page < total_items
}

pub fn has_previous_page(current_page: usize) -> bool {
    current_page > 0
}

#[derive(Properties, PartialEq, Default)]
pub struct VocabularyComponentProps {
    pub challenge: Vocabulary,
    #[prop_or_default]
    pub on_event: Option<Callback<Event>>,
    #[prop_or_default]
    pub on_command: Option<Callback<Command>>,
}

#[derive(Properties, PartialEq)]
pub struct VocabularyCardProps {
    pub item: VocabularyItem,
    pub lang: String,
    pub on_read: Callback<()>,
}

#[function_component(VocabularyCard)]
fn vocabulary_card(props: &VocabularyCardProps) -> Html {
    let i18n = use_i18n();
    let read_triggered = use_state(|| false);

    let handle_read = {
        let on_read = props.on_read.clone();
        let read_triggered = read_triggered.clone();
        Callback::from(move |_| {
            read_triggered.set(true);
            on_read.emit(());
        })
    };

    let read_text_component = {
        #[cfg(feature = "effects")]
        {
            if *read_triggered {
                html! {
                    <ReadText
                        text={props.item.text.clone()}
                        lang={props.lang.clone()}
                    />
                }
            } else {
                html! {}
            }
        }
        #[cfg(not(feature = "effects"))]
        {
            html! {}
        }
    };

    // Reset read_triggered after a short delay to allow re-reading
    {
        let read_triggered = read_triggered.clone();
        use_effect_with(*read_triggered, move |triggered| {
            if *triggered {
                #[cfg(feature = "csr")]
                {
                    use gloo::timers::callback::Timeout;
                    Timeout::new(1000, move || {
                        read_triggered.set(false);
                    })
                    .forget();
                }
            }
            || ()
        });
    }

    html! {
        <div class="vocabulary-card">
            // Read button positioned in top-right corner
            <button
                class="vocabulary-card__read-button"
                onclick={handle_read}
                title={i18n.t("Read aloud")}
            >
                <i class="fa-solid fa-volume-high"></i>
            </button>

            <div class="vocabulary-card__header">
                if let Some(icon) = &props.item.icon {
                    <div class="vocabulary-card__icon">
                        if icon.starts_with("fa-") {
                            <i class={classes!(icon)}></i>
                        } else {
                            <img src={icon.clone()} alt="" class="vocabulary-card__icon-image" />
                        }
                    </div>
                }
            </div>

            <div class="vocabulary-card__content">
                <h3 class="vocabulary-card__text">{&props.item.text}</h3>

                if let Some(translation) = &props.item.translation {
                    <p class="vocabulary-card__translation">{translation}</p>
                }

                if let Some(phonetic) = &props.item.phonetic {
                    <p class="vocabulary-card__phonetic">{phonetic}</p>
                }
            </div>

            {read_text_component}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct VocabularyPageProps {
    pub items: Vec<VocabularyItem>,
    pub lang: String,
    pub current_page: usize,
}

#[function_component(VocabularyPage)]
fn vocabulary_page(props: &VocabularyPageProps) -> Html {
    let start_index = props.current_page * 10;
    let items_to_show = props
        .items
        .iter()
        .skip(start_index)
        .take(10)
        .cloned()
        .collect::<Vec<_>>();

    if items_to_show.is_empty() {
        return html! {};
    }

    html! {
        <div class="vocabulary-grid">
            {for items_to_show.iter().map(|item| {
                let item_clone = item.clone();
                let lang_clone = props.lang.clone();

                html! {
                    <VocabularyCard
                        item={item_clone}
                        lang={lang_clone}
                        on_read={Callback::from(|_| {})}
                    />
                }
            })}
        </div>
    }
}

#[function_component(VocabularyComponent)]
pub fn vocabulary_component(props: &VocabularyComponentProps) -> Html {
    let i18n = use_i18n();
    let current_page = use_state(|| 0);
    let show_help = use_state(|| false);

    const ITEMS_PER_PAGE: usize = 10;

    let total_items = props.challenge.items.len();
    let total_pages = calculate_total_pages(total_items, ITEMS_PER_PAGE);

    if total_pages == 0 {
        return html! {
            <div class="vocabulary">
                <div class="vocabulary__empty">
                    {i18n.t("No vocabulary items available.")}
                </div>
            </div>
        };
    }

    let current_page_val = *current_page;
    let items_on_current_page =
        calculate_items_on_page(current_page_val, total_items, ITEMS_PER_PAGE);
    let has_next = has_next_page(current_page_val, total_items, ITEMS_PER_PAGE);
    let has_prev = has_previous_page(current_page_val);

    let handle_action = {
        let current_page = current_page.clone();
        let show_help = show_help.clone();
        let on_command = props.on_command.clone();

        Callback::from(move |action: ChallengeActions| match action {
            ChallengeActions::Next => {
                let current_page_val = *current_page;

                if has_next_page(current_page_val, total_items, ITEMS_PER_PAGE) {
                    // Go to next page
                    current_page.set(current_page_val + 1);

                    if let Some(on_command) = on_command.as_ref() {
                        on_command.emit(Command::Challenge(ChallengeCommand::NextTask));
                    }
                } else {
                    // Finish the challenge
                    if let Some(on_command) = on_command.as_ref() {
                        on_command.emit(Command::Challenge(ChallengeCommand::Finish(Some(
                            ChallengeResult::Informative,
                        ))));
                    }
                }
            }
            ChallengeActions::Previous => {
                let current_page_val = *current_page;

                if has_previous_page(current_page_val) {
                    current_page.set(current_page_val - 1);

                    if let Some(on_command) = on_command.as_ref() {
                        on_command.emit(Command::Challenge(ChallengeCommand::PreviousTask));
                    }
                }
            }
            ChallengeActions::Help => {
                show_help.set(!*show_help);
            }
        })
    };

    html! {
        <div class="vocabulary">
            <div class="vocabulary__header">
                <h2 class="vocabulary__title">{&props.challenge.name}</h2>
                if let Some(icon) = &props.challenge.icon {
                    <div class="vocabulary__challenge-icon">
                        if icon.starts_with("fa-") {
                            <i class={classes!(icon)}></i>
                        } else {
                            <img src={icon.clone()} alt="" />
                        }
                    </div>
                }
                <p class="vocabulary__description">{&props.challenge.description}</p>
            </div>

            <ProgressBar
                value={current_page_val}
                max={total_pages}
                label={format!("{} {} {} {} ({} {})",
                    i18n.t("Page"),
                    current_page_val + 1,
                    i18n.t("of"),
                    total_pages,
                    items_on_current_page,
                    i18n.t("items")
                )}
            />

            <div class="vocabulary__content">
                <VocabularyPage
                    items={props.challenge.items.clone()}
                    lang={props.challenge.lang.clone()}
                    current_page={current_page_val}
                />
            </div>

            if *show_help {
                <div class="vocabulary__help">
                    <h3 class="vocabulary__help-title">{i18n.t("Help")}</h3>
                    <p class="vocabulary__help-text">
                        {i18n.t("Review these vocabulary words and their meanings. Click the listen button to hear the pronunciation.")}
                    </p>
                    <div class="vocabulary__help-navigation">
                        <p>{i18n.t("Use the Previous/Next buttons to navigate between vocabulary pages.")}</p>
                    </div>
                    <div class="vocabulary__help-debug">
                        <p>{format!("Debug: Page {}/{}, Items: {}, Has next: {}, Has prev: {}",
                            current_page_val + 1,
                            total_pages,
                            items_on_current_page,
                            has_next,
                            has_prev
                        )}</p>
                    </div>
                </div>
            }

            <ChallengeActionsComponent on_action={handle_action} />
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_total_pages() {
        assert_eq!(calculate_total_pages(0, 10), 0);
        assert_eq!(calculate_total_pages(1, 10), 1);
        assert_eq!(calculate_total_pages(10, 10), 1);
        assert_eq!(calculate_total_pages(11, 10), 2);
        assert_eq!(calculate_total_pages(16, 10), 2);
        assert_eq!(calculate_total_pages(20, 10), 2);
        assert_eq!(calculate_total_pages(21, 10), 3);
    }

    #[test]
    fn test_calculate_items_on_page() {
        // 16 items total, 10 per page
        assert_eq!(calculate_items_on_page(0, 16, 10), 10); // Page 0: items 0-9
        assert_eq!(calculate_items_on_page(1, 16, 10), 6); // Page 1: items 10-15
        assert_eq!(calculate_items_on_page(2, 16, 10), 0); // Page 2: no items

        // 10 items total, 10 per page
        assert_eq!(calculate_items_on_page(0, 10, 10), 10); // Page 0: items 0-9
        assert_eq!(calculate_items_on_page(1, 10, 10), 0); // Page 1: no items

        // 3 items total, 10 per page
        assert_eq!(calculate_items_on_page(0, 3, 10), 3); // Page 0: items 0-2
        assert_eq!(calculate_items_on_page(1, 3, 10), 0); // Page 1: no items
    }

    #[test]
    fn test_has_next_page() {
        // 16 items total, 10 per page
        assert_eq!(has_next_page(0, 16, 10), true); // Page 0 -> Page 1 exists
        assert_eq!(has_next_page(1, 16, 10), false); // Page 1 -> Page 2 doesn't exist

        // 10 items total, 10 per page
        assert_eq!(has_next_page(0, 10, 10), false); // Page 0 -> Page 1 doesn't exist

        // 21 items total, 10 per page
        assert_eq!(has_next_page(0, 21, 10), true); // Page 0 -> Page 1 exists
        assert_eq!(has_next_page(1, 21, 10), true); // Page 1 -> Page 2 exists
        assert_eq!(has_next_page(2, 21, 10), false); // Page 2 -> Page 3 doesn't exist
    }

    #[test]
    fn test_has_previous_page() {
        assert_eq!(has_previous_page(0), false);
        assert_eq!(has_previous_page(1), true);
        assert_eq!(has_previous_page(2), true);
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use konnektoren_core::challenges::VocabularyItem;
    use yew_preview::prelude::*;

    fn create_test_vocabulary() -> Vocabulary {
        Vocabulary {
            id: "test-vocabulary".to_string(),
            name: "Basic German Vocabulary".to_string(),
            description: "Learn essential German words".to_string(),
            icon: Some("fa-solid fa-book".to_string()),
            lang: "de".to_string(),
            items: vec![
                VocabularyItem {
                    id: 0,
                    text: "der Apfel".to_string(),
                    translation: Some("the apple".to_string()),
                    icon: Some("fa-solid fa-apple-whole".to_string()),
                    phonetic: Some("/ˈapfəl/".to_string()),
                },
                VocabularyItem {
                    id: 1,
                    text: "das Haus".to_string(),
                    translation: Some("the house".to_string()),
                    icon: Some("fa-solid fa-house".to_string()),
                    phonetic: Some("/haʊs/".to_string()),
                },
                VocabularyItem {
                    id: 2,
                    text: "die Katze".to_string(),
                    translation: Some("the cat".to_string()),
                    icon: Some("fa-solid fa-cat".to_string()),
                    phonetic: Some("/ˈkatsə/".to_string()),
                },
                VocabularyItem {
                    id: 3,
                    text: "das Auto".to_string(),
                    translation: Some("the car".to_string()),
                    icon: Some("fa-solid fa-car".to_string()),
                    phonetic: Some("/ˈaʊto/".to_string()),
                },
                VocabularyItem {
                    id: 4,
                    text: "der Hund".to_string(),
                    translation: Some("the dog".to_string()),
                    icon: Some("fa-solid fa-dog".to_string()),
                    phonetic: Some("/hʊnt/".to_string()),
                },
                VocabularyItem {
                    id: 5,
                    text: "die Blume".to_string(),
                    translation: Some("the flower".to_string()),
                    icon: Some("fa-solid fa-seedling".to_string()),
                    phonetic: Some("/ˈbluːmə/".to_string()),
                },
                VocabularyItem {
                    id: 6,
                    text: "das Buch".to_string(),
                    translation: Some("the book".to_string()),
                    icon: Some("fa-solid fa-book".to_string()),
                    phonetic: Some("/buːx/".to_string()),
                },
                VocabularyItem {
                    id: 7,
                    text: "der Tisch".to_string(),
                    translation: Some("the table".to_string()),
                    icon: Some("fa-solid fa-table".to_string()),
                    phonetic: Some("/tɪʃ/".to_string()),
                },
                VocabularyItem {
                    id: 8,
                    text: "die Sonne".to_string(),
                    translation: Some("the sun".to_string()),
                    icon: Some("fa-solid fa-sun".to_string()),
                    phonetic: Some("/ˈzɔnə/".to_string()),
                },
                VocabularyItem {
                    id: 9,
                    text: "das Wasser".to_string(),
                    translation: Some("the water".to_string()),
                    icon: Some("fa-solid fa-droplet".to_string()),
                    phonetic: Some("/ˈvasɐ/".to_string()),
                },
                VocabularyItem {
                    id: 10,
                    text: "der Baum".to_string(),
                    translation: Some("the tree".to_string()),
                    icon: Some("fa-solid fa-tree".to_string()),
                    phonetic: Some("/baʊm/".to_string()),
                },
                VocabularyItem {
                    id: 11,
                    text: "die Musik".to_string(),
                    translation: Some("the music".to_string()),
                    icon: Some("fa-solid fa-music".to_string()),
                    phonetic: Some("/muˈziːk/".to_string()),
                },
            ],
        }
    }

    fn create_small_vocabulary() -> Vocabulary {
        Vocabulary {
            id: "small-vocabulary".to_string(),
            name: "Small Vocabulary Set".to_string(),
            description: "A smaller set for testing".to_string(),
            icon: Some("fa-solid fa-book-open".to_string()),
            lang: "en".to_string(),
            items: vec![
                VocabularyItem {
                    id: 0,
                    text: "Hello".to_string(),
                    translation: Some("Hola".to_string()),
                    icon: Some("fa-solid fa-hand-wave".to_string()),
                    phonetic: Some("/həˈloʊ/".to_string()),
                },
                VocabularyItem {
                    id: 1,
                    text: "Goodbye".to_string(),
                    translation: Some("Adiós".to_string()),
                    icon: Some("fa-solid fa-hand-peace".to_string()),
                    phonetic: Some("/ɡʊdˈbaɪ/".to_string()),
                },
                VocabularyItem {
                    id: 2,
                    text: "Thank you".to_string(),
                    translation: Some("Gracias".to_string()),
                    icon: Some("fa-solid fa-heart".to_string()),
                    phonetic: Some("/θæŋk juː/".to_string()),
                },
            ],
        }
    }

    // Test with exactly 16 items (like your street vocabulary)
    fn create_sixteen_item_vocabulary() -> Vocabulary {
        let mut items = vec![];
        for i in 0..16 {
            items.push(VocabularyItem {
                id: i,
                text: format!("Item {}", i + 1),
                translation: Some(format!("Translation {}", i + 1)),
                icon: Some("fa-solid fa-star".to_string()),
                phonetic: Some(format!("/test{}/", i + 1)),
            });
        }

        Vocabulary {
            id: "sixteen-items".to_string(),
            name: "Sixteen Items Test".to_string(),
            description: "Test with exactly 16 items".to_string(),
            icon: Some("fa-solid fa-list".to_string()),
            lang: "en".to_string(),
            items,
        }
    }

    fn create_gif_vocabulary() -> Vocabulary {
        Vocabulary {
            id: "gif-vocabulary".to_string(),
            name: "Summer Vocabulary with GIFs".to_string(),
            description: "Learn summer vocabulary with animated GIFs".to_string(),
            icon: Some("https://media.giphy.com/media/26FLgGTPUDH6UGAbm/giphy.gif".to_string()),
            lang: "de".to_string(),
            items: vec![
                VocabularyItem {
                    id: 0,
                    text: "der Sommer".to_string(),
                    translation: Some("the summer".to_string()),
                    icon: Some(
                        "https://media.giphy.com/media/26FLgGTPUDH6UGAbm/giphy.gif".to_string(),
                    ),
                    phonetic: Some("/ˈzɔmɐ/".to_string()),
                },
                VocabularyItem {
                    id: 1,
                    text: "die Sonne".to_string(),
                    translation: Some("the sun".to_string()),
                    icon: Some(
                        "https://media.giphy.com/media/xT5LMHxhOfscxPfIfm/giphy.gif".to_string(),
                    ),
                    phonetic: Some("/ˈzɔnə/".to_string()),
                },
                VocabularyItem {
                    id: 2,
                    text: "das Schwimmbad".to_string(),
                    translation: Some("the swimming pool".to_string()),
                    icon: Some(
                        "https://media.giphy.com/media/l0HlBO7eyXzSZkJri/giphy.gif".to_string(),
                    ),
                    phonetic: Some("/ˈʃvɪmbaːt/".to_string()),
                },
                VocabularyItem {
                    id: 3,
                    text: "baden".to_string(),
                    translation: Some("to bathe, to swim".to_string()),
                    icon: Some(
                        "https://media.giphy.com/media/3o7TKOCXl47JSbLUGc/giphy.gif".to_string(),
                    ),
                    phonetic: Some("/ˈbaːdən/".to_string()),
                },
                VocabularyItem {
                    id: 4,
                    text: "der Strand".to_string(),
                    translation: Some("the beach".to_string()),
                    icon: Some(
                        "https://media.giphy.com/media/3o6ZtaO9BZHcOjmErm/giphy.gif".to_string(),
                    ),
                    phonetic: Some("/ʃtʁant/".to_string()),
                },
                VocabularyItem {
                    id: 5,
                    text: "das Eis".to_string(),
                    translation: Some("the ice cream".to_string()),
                    icon: Some(
                        "https://media.giphy.com/media/3o6UBlHJQT19wSgJQk/giphy.gif".to_string(),
                    ),
                    phonetic: Some("/aɪs/".to_string()),
                },
                VocabularyItem {
                    id: 6,
                    text: "die Sonnencreme".to_string(),
                    translation: Some("the sunscreen".to_string()),
                    icon: Some(
                        "https://media.giphy.com/media/l0HlwwRxfcVEr4AUg/giphy.gif".to_string(),
                    ),
                    phonetic: Some("/ˈzɔnənˌkʁeːmə/".to_string()),
                },
            ],
        }
    }

    yew_preview::create_preview!(
        VocabularyComponent,
        VocabularyComponentProps {
            challenge: create_test_vocabulary(),
            on_command: None,
            on_event: None,
        },
        (
            "Small Vocabulary",
            VocabularyComponentProps {
                challenge: create_small_vocabulary(),
                on_command: None,
                on_event: None,
            }
        ),
        (
            "Sixteen Items",
            VocabularyComponentProps {
                challenge: create_sixteen_item_vocabulary(),
                on_command: None,
                on_event: None,
            }
        ),
        (
            "Summer with GIFs",
            VocabularyComponentProps {
                challenge: create_gif_vocabulary(),
                on_command: None,
                on_event: None,
            }
        ),
        (
            "Empty Vocabulary",
            VocabularyComponentProps {
                challenge: Vocabulary {
                    id: "empty".to_string(),
                    name: "Empty Vocabulary".to_string(),
                    description: "No items".to_string(),
                    icon: None,
                    lang: "en".to_string(),
                    items: vec![],
                },
                on_command: None,
                on_event: None,
            }
        ),
        (
            "Single Item",
            VocabularyComponentProps {
                challenge: Vocabulary {
                    id: "single".to_string(),
                    name: "Single Item".to_string(),
                    description: "Only one vocabulary item".to_string(),
                    icon: Some("fa-solid fa-book".to_string()),
                    lang: "en".to_string(),
                    items: vec![VocabularyItem {
                        id: 0,
                        text: "Hello".to_string(),
                        translation: Some("Hola".to_string()),
                        icon: Some("fa-solid fa-hand-wave".to_string()),
                        phonetic: Some("/həˈloʊ/".to_string()),
                    },],
                },
                on_command: None,
                on_event: None,
            }
        )
    );
}

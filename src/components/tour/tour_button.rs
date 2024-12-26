use crate::components::Tour;
use crate::i18n::use_i18n;
use crate::providers::use_settings;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: String,
    pub data: String,
}

#[function_component(TourButton)]
pub fn tour_button(props: &Props) -> Html {
    let i18n = use_i18n();
    let show_tour = use_state(|| false);
    let settings = use_settings();

    let on_click = {
        let show_tour = show_tour.clone();
        let settings = settings.clone();
        Callback::from(move |_| {
            show_tour.set(true);
            let mut new_settings = (*settings).clone();
            new_settings.show_helpers = false;
            settings.set(new_settings);
        })
    };

    match (settings.show_helpers, *show_tour) {
        (true, false) => {
            html! {
                <div class="tour-button">
                    <button class="tour-button__btn" onclick={on_click}>
                        { i18n.t("Start Tour") }
                    </button>
                </div>
            }
        }
        (_, true) => {
            html! {
                <div class="tour-welcome">
                    <Tour
                        id={props.id.clone()}
                        data={props.data.clone()}
                    />
                </div>
            }
        }
        _ => html! { <></> },
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        TourButton,
        Props {
            id: "tour".to_string(),
            data: {
                r#"steps:
  - selector: ".tour-welcome"
    content: "Welcome to Konnektoren!"
  - selector: ".select-theme"
    content: "Select a theme"
  - selector: ".select-design"
    content: "Select a design""#
                    .to_string()
            }
        },
    );
}

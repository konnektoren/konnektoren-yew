use crate::i18n::use_i18n;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChallengeHelpProps {
    #[prop_or_default]
    pub title: Option<AttrValue>,
    #[prop_or_default]
    pub text: Option<AttrValue>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ChallengeHelpComponent)]
pub fn challenge_help_component(props: &ChallengeHelpProps) -> Html {
    let i18n = use_i18n();
    let title = props
        .title
        .clone()
        .unwrap_or_else(|| AttrValue::from(i18n.t("Help")));

    html! {
        <div class="challenge-help">
            <h3 class="challenge-help__title">{ title }</h3>
            if let Some(text) = &props.text {
                <p class="challenge-help__text">{ text }</p>
            }
            if !props.children.is_empty() {
                <div class="challenge-help__content">
                    { for props.children.iter() }
                </div>
            }
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        ChallengeHelpComponent,
        ChallengeHelpProps {
            title: None,
            text: Some(AttrValue::from(
                "Fill in the blanks by selecting the correct option."
            )),
            children: Default::default(),
        },
    );
}

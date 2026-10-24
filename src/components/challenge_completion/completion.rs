use super::ChallengeResultDetails;
use crate::components::challenge::ResultSummaryComponent;
use konnektoren_core::challenges::{Challenge, ChallengeResult};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChallengeCompletionProps {
    pub challenge: Challenge,
    pub challenge_result: ChallengeResult,
    #[prop_or_default]
    pub heading: Html,
    #[prop_or_default]
    pub introduction: Html,
    #[prop_or_default]
    pub status: Html,
    #[prop_or_default]
    pub actions: Html,
    #[prop_or_default]
    pub custom_result: Option<Html>,
    /// Disable automatic details when the app places them among other content.
    #[prop_or(true)]
    pub show_details: bool,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub labelledby: Option<AttrValue>,
}

#[function_component(ChallengeCompletion)]
pub fn challenge_completion(props: &ChallengeCompletionProps) -> Html {
    html! {
        <section class={classes!("challenge-completion", props.class.clone())}
            id={props.id.clone()} aria-labelledby={props.labelledby.clone()}>
            {props.heading.clone()}
            {props.introduction.clone()}
            <ResultSummaryComponent challenge={props.challenge.clone()} challenge_result={props.challenge_result.clone()} />
            if props.show_details {
                <div class="challenge-completion__details">
                    <ChallengeResultDetails challenge={props.challenge.clone()} challenge_result={props.challenge_result.clone()}
                        custom_result={props.custom_result.clone()} />
                </div>
            }
            {props.status.clone()}
            if props.actions != Html::default() {
                <div class="challenge-completion__actions">{props.actions.clone()}</div>
            }
            {for props.children.iter()}
        </section>
    }
}

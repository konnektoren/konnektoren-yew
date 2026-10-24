use crate::components::challenge::*;
use konnektoren_core::challenges::{Challenge, ChallengeResult, ChallengeType};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChallengeResultDetailsProps {
    pub challenge: Challenge,
    pub challenge_result: ChallengeResult,
    /// Override custom challenge results, including with empty HTML to hide them.
    #[prop_or_default]
    pub custom_result: Option<Html>,
}

#[function_component(ChallengeResultDetails)]
pub fn challenge_result_details(props: &ChallengeResultDetailsProps) -> Html {
    let result = &props.challenge_result;
    match &props.challenge.challenge_type {
        ChallengeType::MultipleChoice(challenge) => html! {
            <MultipleChoiceResultComponent challenge={challenge.clone()} challenge_result={result.clone()} />
        },
        ChallengeType::ContextualChoice(challenge) => html! {
            <ContextualChoiceResultComponent challenge={challenge.clone()} challenge_result={result.clone()} />
        },
        ChallengeType::GapFill(challenge) => html! {
            <GapFillResultComponent challenge={challenge.clone()} challenge_result={result.clone()} />
        },
        ChallengeType::SortTable(challenge) => html! {
            <SortTableResultComponent challenge={challenge.clone()} challenge_result={result.clone()} />
        },
        ChallengeType::Ordering(challenge) => html! {
            <OrderingResultComponent challenge={challenge.clone()} challenge_result={result.clone()} />
        },
        ChallengeType::Dialog(challenge) => html! {
            <DialogResultComponent challenge={challenge.clone()} challenge_result={result.clone()}
                variant={props.challenge.challenge_config.variant.clone()} />
        },
        ChallengeType::Custom(challenge) => {
            if let Some(custom_result) = &props.custom_result {
                custom_result.clone()
            } else if let ChallengeResult::Custom(result) = result {
                html! { <CustomResultComponent challenge={challenge.clone()} result={result.clone()} /> }
            } else {
                Html::default()
            }
        }
        ChallengeType::Informative(_)
        | ChallengeType::Placeholder(_)
        | ChallengeType::Vocabulary(_) => {
            html! { <InformativeResultComponent /> }
        }
    }
}

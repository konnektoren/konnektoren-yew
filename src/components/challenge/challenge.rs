use super::{
    ContextualChoiceComponent, ContextualChoiceResultComponent, CustomComponent,
    CustomPackageComponent, GapFillComponent, GapFillResultComponent, InformativeComponent,
    InformativeMarkdownComponent, MultipleChoiceCircleComponent, MultipleChoiceComponent,
    MultipleChoiceResultComponent, OrderingResultComponent, SortTableComponent,
};
use crate::components::{ChallengeInfoComponent, ChallengeTimerComponent};
use konnektoren_core::challenges::ChallengeVariant;
use konnektoren_core::commands::Command;
use konnektoren_core::events::Event;
use konnektoren_core::prelude::*;
use yew::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct ChallengeComponentProps {
    pub challenge: Challenge,
    #[prop_or_default]
    pub variant: Option<ChallengeVariant>,
    #[prop_or_default]
    pub on_event: Option<Callback<Event>>,
    #[prop_or_default]
    pub on_command: Option<Callback<Command>>,
    #[prop_or_default]
    pub language: Option<String>,
}

#[function_component(ChallengeComponent)]
pub fn challenge_component(props: &ChallengeComponentProps) -> Html {
    let challenge_result = use_state(|| Option::<ChallengeResult>::None);
    let show_challenge_info = use_state(|| false);

    let handle_event = {
        let on_event = props.on_event.clone();
        Callback::from(move |event: Event| {
            if let Some(on_event) = on_event.as_ref() {
                on_event.emit(event);
            }
        })
    };

    let handle_command = {
        let on_command = props.on_command.clone();
        Callback::from(move |command: Command| {
            if let Some(on_command) = on_command.as_ref() {
                on_command.emit(command);
            }
        })
    };

    let challenge_component = match (
        &*challenge_result,
        &props.challenge.challenge_type,
        &props.variant.clone().unwrap_or_default(),
    ) {
        (None, ChallengeType::MultipleChoice(challenge), ChallengeVariant::MultipleChoice) => {
            html! {
                <MultipleChoiceComponent challenge={challenge.clone()}
                on_event={handle_event}
               on_command={handle_command} />
            }
        }
        (
            None,
            ChallengeType::MultipleChoice(challenge),
            ChallengeVariant::MultipleChoiceCircle,
        ) => html! {
            <MultipleChoiceCircleComponent challenge={challenge.clone()}
                on_event={handle_event}
               on_command={handle_command} />
        },
        (None, ChallengeType::ContextualChoice(challenge), ChallengeVariant::ContextualChoice) => {
            html! {
                <ContextualChoiceComponent challenge={challenge.clone()}
                on_event={handle_event}
               on_command={handle_command} />
            }
        }
        (None, ChallengeType::GapFill(challenge), _) => html! {
            <GapFillComponent challenge={challenge.clone()} on_event={handle_event}
           on_command={handle_command} />
        },
        (None, ChallengeType::SortTable(challenge), ChallengeVariant::SortTable) => html! {
            <SortTableComponent challenge={challenge.clone()} on_event={handle_event}
           on_command={handle_command} />
        },
        (None, ChallengeType::Informative(challenge), ChallengeVariant::InformativeText) => html! {
            <InformativeComponent challenge={challenge.clone()} on_command={handle_command} language={props.language.clone()} />
        },
        (None, ChallengeType::Informative(challenge), ChallengeVariant::InformativeMarkdown) => {
            html! {
                <InformativeMarkdownComponent challenge={challenge.clone()} on_command={handle_command} language={props.language.clone()}  />
            }
        }
        (None, ChallengeType::Custom(challenge), ChallengeVariant::Custom) => html! {
            <CustomComponent challenge={challenge.clone()} on_event={handle_event}
           on_command={handle_command} />
        },
        (None, ChallengeType::Custom(challenge), ChallengeVariant::CustomPackage) => html! {
            <CustomPackageComponent challenge={challenge.clone()} on_event={handle_event}
           on_command={handle_command} />
        },
        _ => html! {},
    };

    let challenge_result_component = match (&*challenge_result, &props.challenge.challenge_type) {
        (Some(result), ChallengeType::MultipleChoice(challenge)) => html! {
            <MultipleChoiceResultComponent challenge={challenge.clone()} challenge_result={result.clone()} />
        },
        (Some(result), ChallengeType::ContextualChoice(challenge)) => html! {
            <ContextualChoiceResultComponent challenge={challenge.clone()} challenge_result={result.clone()} />
        },
        (Some(result), ChallengeType::GapFill(challenge)) => html! {
            <GapFillResultComponent challenge={challenge.clone()} challenge_result={result.clone()} />
        },
        (Some(result), ChallengeType::Ordering(challenge)) => html! {
            <OrderingResultComponent challenge={challenge.clone()} challenge_result={result.clone()} />
        },
        _ => html! {},
    };

    let challenge_info = {
        let show_info = *show_challenge_info;
        html! {
            <>
                <button class="challenge-info-button" onclick={
                Callback::from(move |_| show_challenge_info.set(!show_info))}>
                    {if show_info { "X" } else { "?" }}
                </button>
                <div class="challenge-info" style={if show_info { "display: block;" } else { "display: none;" }}>
                <ChallengeInfoComponent challenge_config={props.challenge.challenge_config.clone()} />
                <ChallengeTimerComponent challenge={props.challenge.clone()} running={true} />
                </div>
            </>
        }
    };

    html! {
        <div class="challenge">
            {challenge_info}
            {challenge_component}
            {challenge_result_component}
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(ChallengeComponent, ChallengeComponentProps::default(),);
}

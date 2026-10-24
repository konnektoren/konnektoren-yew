use super::*;
use konnektoren_core::challenges::{
    Challenge, ChallengeConfig, ChallengeResult, ChallengeType, Custom, MultipleChoice,
    MultipleChoiceOption, Question,
};
use yew::prelude::*;
use yew_preview::prelude::*;

fn sample() -> Challenge {
    let answer = MultipleChoiceOption {
        id: 0,
        name: "Hallo".into(),
    };
    let mut challenge = Challenge::new(
        &ChallengeType::MultipleChoice(MultipleChoice {
            id: "greetings".into(),
            name: "Greetings".into(),
            lang: "de".into(),
            options: vec![answer.clone()],
            questions: vec![Question {
                question: "Hello".into(),
                option: 0,
                ..Question::default()
            }],
        }),
        &ChallengeConfig {
            id: "greetings".into(),
            name: "Greetings".into(),
            ..ChallengeConfig::default()
        },
    );
    challenge.challenge_result = ChallengeResult::MultipleChoice(vec![answer]);
    challenge
}

fn props() -> ChallengeCompletionProps {
    let challenge = sample();
    ChallengeCompletionProps {
        challenge_result: challenge.challenge_result.clone(),
        challenge,
        heading: Html::default(),
        introduction: Html::default(),
        status: Html::default(),
        actions: Html::default(),
        custom_result: None,
        show_details: true,
        children: Children::default(),
        class: Classes::default(),
        id: None,
        labelledby: None,
    }
}

yew_preview::create_preview!(
    ChallengeCompletion,
    props(),
    (
        "with heading and actions",
        ChallengeCompletionProps {
            heading: html! { <h2>{"Exercise results"}</h2> },
            introduction: html! { <p>{"Review your answers before continuing."}</p> },
            actions: html! { <><button class="btn btn-primary">{"Next exercise"}</button><button class="btn btn-outline">{"Try again"}</button></> },
            ..props()
        }
    ),
    (
        "summary only",
        ChallengeCompletionProps {
            show_details: false,
            ..props()
        }
    ),
    ("custom review", {
        let challenge = Challenge::new(
            &ChallengeType::Custom(Custom::default()),
            &ChallengeConfig::default(),
        );
        ChallengeCompletionProps {
            challenge_result: challenge.challenge_result.clone(),
            challenge,
            custom_result: Some(html! { <p>{"An application-specific review can go here."}</p> }),
            ..props()
        }
    }),
);

yew_preview::create_preview!(
    ChallengeResultDetails,
    ChallengeResultDetailsProps {
        challenge: sample(),
        challenge_result: sample().challenge_result,
        custom_result: None,
    },
    (
        "incorrect answer",
        ChallengeResultDetailsProps {
            challenge: sample(),
            challenge_result: ChallengeResult::MultipleChoice(vec![MultipleChoiceOption {
                id: 1,
                name: "Tschüss".into()
            }]),
            custom_result: None,
        }
    ),
);

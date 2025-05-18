use super::{OptionsComponent, QuestionComponent};
use crate::components::ProgressBar;
use crate::components::challenge::multiple_choice::{
    MultipleChoiceComponentProps, create_handle_option_selection,
};
use crate::i18n::{use_i18n, use_selected_language};
#[cfg(feature = "effects")]
use crate::prelude::ReadText;
use konnektoren_core::challenges::{ChallengeResult, MultipleChoice, MultipleChoiceOption};
use rand::prelude::{SeedableRng, SliceRandom, StdRng};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use yew::prelude::*;

fn get_4_options(challenge: &MultipleChoice, question_index: usize) -> Vec<MultipleChoiceOption> {
    let question = &challenge.questions[question_index];
    let correct_id = question.option;

    // Find the correct option
    let correct_option = challenge.options.iter().find(|opt| opt.id == correct_id);

    // Collect all distractors (not the correct one)
    let mut distractors: Vec<_> = challenge
        .options
        .iter()
        .filter(|opt| opt.id != correct_id)
        .cloned()
        .collect();

    // --- Deterministic seeding ---
    // Hash challenge id and question index together for the seed
    let mut hasher = DefaultHasher::new();
    challenge.id.hash(&mut hasher);
    question_index.hash(&mut hasher);
    let seed = hasher.finish();
    let mut rng = StdRng::seed_from_u64(seed);

    // Shuffle distractors and pick 3
    distractors.shuffle(&mut rng);
    let mut selected_options = Vec::new();

    if let Some(correct) = correct_option.cloned() {
        selected_options.push(correct);
    }
    selected_options.extend(distractors.into_iter().take(3));

    // Shuffle the final 4 options so the correct answer is not always first
    selected_options.shuffle(&mut rng);

    selected_options
}

#[function_component(MultipleChoice4Component)]
pub fn multiple_choice_4_component(props: &MultipleChoiceComponentProps) -> Html {
    let i18n = use_i18n();
    let selected_language = use_selected_language();
    let lang_code = selected_language.get().code();

    let task_index = use_state(|| 0);
    let challenge_result = use_state(ChallengeResult::default);
    let show_help = use_state(|| false);

    if *task_index >= props.challenge.questions.len() {
        return html! {};
    }

    let handle_option_selection = create_handle_option_selection(
        task_index.clone(),
        props.challenge.clone(),
        challenge_result.clone(),
        props.challenge.questions.len(),
        props.on_command.clone(),
        props.on_event.clone(),
    );

    let read_text = {
        #[cfg(feature = "effects")]
        {
            html! {
                <ReadText
                    text={i18n.t(&props.challenge.questions[*task_index].question)}
                    lang={lang_code}
                />
            }
        }
        #[cfg(not(feature = "effects"))]
        {
            html! {}
        }
    };

    let options = get_4_options(&props.challenge, *task_index);

    // Prepare translated question and help
    let question = {
        let mut q = props.challenge.questions[*task_index].clone();
        q.question = i18n.t(&q.question);
        q.help = i18n.t(&q.help);
        q
    };

    html! {
        <div class="multiple-choice-4">
            <ProgressBar
                value={*task_index}
                max={props.challenge.questions.len()}
                label={format!("Question {} of {}", *task_index + 1, props.challenge.questions.len())}
            />
            <div class="multiple-choice-4__content">
                <QuestionComponent
                    question={question}
                    help={*show_help}
                />
                <OptionsComponent
                    options={options}
                    on_select={handle_option_selection}
                />
            </div>
            {read_text}
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use konnektoren_core::prelude::{ChallengeType, Game};
    use yew_preview::prelude::*;

    use konnektoren_core::challenges::multiple_choice::MultipleChoice;

    fn create_default_challenge() -> MultipleChoice {
        let game = Game::default();
        let default_challenge = game.create_challenge("konnektoren-1").unwrap();
        match &default_challenge.challenge_type {
            ChallengeType::MultipleChoice(multiple_choice) => multiple_choice.clone(),
            _ => unreachable!(),
        }
    }

    yew_preview::create_preview!(
        MultipleChoice4Component,
        MultipleChoiceComponentProps {
            challenge: create_default_challenge(),
            on_command: None,
            on_event: None,
        },
    );
}

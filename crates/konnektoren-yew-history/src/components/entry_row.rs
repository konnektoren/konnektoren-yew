use super::EntryParts;
use konnektoren_yew_core::{Pill, Time};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HistoryEntryRowProps {
    pub parts: EntryParts,
    /// Leaves out the facts, for previews.
    #[prop_or_default]
    pub compact: bool,
}

/// The standard row: icon, title, time, facts, result badge and actions.
#[function_component(HistoryEntryRow)]
pub fn history_entry_row(props: &HistoryEntryRowProps) -> Html {
    let parts = &props.parts;
    let title = parts.title.clone();
    html! {
        <div class={classes!(
            "history-entry",
            parts.selected.then_some("history-entry--selected"),
            props.compact.then_some("history-entry--compact"),
        )}>
            <div class="history-entry__main">
                if let Some(icon) = &parts.icon {
                    <span class="history-entry__icon" aria-hidden="true">{icon.clone()}</span>
                }
                <span class="history-entry__heading">
                    if let Some(on_select) = &parts.on_select {
                        <button type="button" class="history-entry__title history-entry__title--button"
                            onclick={on_select.clone()} aria-pressed={parts.selected.to_string()}
                            title={title.clone()}>{title}</button>
                    } else {
                        <span class="history-entry__title" title={title.clone()}>{title}</span>
                    }
                    if let Some(time) = &parts.time {
                        <Time class="history-entry__time" value={time.clone()} />
                    }
                </span>
            </div>
            if !props.compact && !parts.meta.is_empty() {
                <span class="history-entry__meta">
                    {for parts.meta.iter().map(|meta| html! {
                        <span class={classes!("history-entry__fact", meta.tone.modifier("history-entry__fact"))}
                            title={meta.title.clone()}>
                            if let Some(icon) = &meta.icon {
                                <span class="history-entry__fact-icon" aria-hidden="true">{icon.clone()}</span>
                            }
                            {meta.text.clone()}
                        </span>
                    })}
                </span>
            }
            if let Some(result) = &parts.result {
                <Pill class="history-entry__result" text={result.text.clone()} tone={result.tone}
                    icon={result.icon.clone()} label={result.label.clone()} />
            }
            if parts.actions != Html::default() {
                <span class="history-entry__actions">{parts.actions.clone()}</span>
            }
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use crate::model::{TimeValue, Tone};
    use crate::{EntryMeta, EntryResult};
    use chrono::{TimeZone, Utc};
    use yew_preview::prelude::*;

    fn parts() -> EntryParts {
        EntryParts {
            icon: Some(html! { "🎼" }),
            title: "Ode to Joy".into(),
            meta: vec![
                EntryMeta::rating_change(1000.0, 1012.0),
                EntryMeta::text("Mastered").with_tone(Tone::Good),
            ],
            result: Some(
                EntryResult::new("90%")
                    .with_tone(Tone::Good)
                    .with_label("Result"),
            ),
            time: Utc
                .with_ymd_and_hms(2026, 11, 22, 14, 30, 0)
                .single()
                .map(TimeValue::At),
            ..EntryParts::default()
        }
    }

    yew_preview::create_preview!(
        HistoryEntryRow,
        HistoryEntryRowProps {
            parts: parts(),
            compact: false,
        },
        (
            "Selectable, selected",
            HistoryEntryRowProps {
                parts: EntryParts {
                    selected: true,
                    on_select: Some(Callback::noop()),
                    ..parts()
                },
                compact: false,
            }
        ),
        (
            "Not passed, rating loss",
            HistoryEntryRowProps {
                parts: EntryParts {
                    title: "Silent Night".into(),
                    meta: vec![EntryMeta::rating_change(1012.0, 1003.0)],
                    result: Some(EntryResult::new("45%").with_tone(Tone::Warn)),
                    time: Some(TimeValue::Raw("2026-11-21T09:05:00.000Z".into())),
                    ..parts()
                },
                compact: false,
            }
        ),
        (
            "Compact",
            HistoryEntryRowProps {
                parts: parts(),
                compact: true,
            }
        ),
    );
}

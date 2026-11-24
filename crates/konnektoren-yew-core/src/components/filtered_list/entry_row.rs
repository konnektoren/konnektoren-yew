use crate::components::{Pill, Time};
use crate::model::EntryParts;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EntryRowProps {
    pub parts: EntryParts,
    /// Leaves out the facts, for previews.
    #[prop_or_default]
    pub compact: bool,
}

/// The standard row: icon, title, time, facts, result badge and actions.
#[function_component(EntryRow)]
pub fn entry_row(props: &EntryRowProps) -> Html {
    let parts = &props.parts;
    let title = parts.title.clone();
    html! {
        <div class={classes!(
            "entry-row",
            parts.selected.then_some("entry-row--selected"),
            props.compact.then_some("entry-row--compact"),
        )}>
            <div class="entry-row__main">
                if let Some(icon) = &parts.icon {
                    <span class="entry-row__icon" aria-hidden="true">{icon.clone()}</span>
                }
                <span class="entry-row__heading">
                    if let Some(on_select) = &parts.on_select {
                        <button type="button" class="entry-row__title entry-row__title--button"
                            onclick={on_select.clone()} aria-pressed={parts.selected.to_string()}
                            title={title.clone()}>{title}</button>
                    } else {
                        <span class="entry-row__title" title={title.clone()}>{title}</span>
                    }
                    if let Some(time) = &parts.time {
                        <Time class="entry-row__time" value={time.clone()} />
                    }
                </span>
            </div>
            if !props.compact && !parts.meta.is_empty() {
                <span class="entry-row__meta">
                    {for parts.meta.iter().map(|meta| html! {
                        <span class={classes!("entry-row__fact", meta.tone.modifier("entry-row__fact"))}
                            title={meta.title.clone()}>
                            if let Some(icon) = &meta.icon {
                                <span class="entry-row__fact-icon" aria-hidden="true">{icon.clone()}</span>
                            }
                            {meta.text.clone()}
                        </span>
                    })}
                </span>
            }
            if let Some(result) = &parts.result {
                <Pill class="entry-row__result" text={result.text.clone()} tone={result.tone}
                    icon={result.icon.clone()} label={result.label.clone()} />
            }
            if parts.actions != Html::default() {
                <span class="entry-row__actions">{parts.actions.clone()}</span>
            }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::EntryMeta;

    #[tokio::test]
    async fn renders_selectable_and_compact_rows() {
        let parts = EntryParts {
            title: "Apple".into(),
            selected: true,
            on_select: Some(Callback::noop()),
            meta: vec![EntryMeta::text("Apfel")],
            ..EntryParts::default()
        };
        let row = yew::LocalServerRenderer::<EntryRow>::with_props(EntryRowProps {
            parts: parts.clone(),
            compact: false,
        })
        .render()
        .await;
        assert!(row.contains("Apple"));
        assert!(row.contains("Apfel"));
        assert!(row.contains("entry-row--selected"));
        assert!(row.contains("aria-pressed=\"true\""));
        let compact = yew::LocalServerRenderer::<EntryRow>::with_props(EntryRowProps {
            parts,
            compact: true,
        })
        .render()
        .await;
        assert!(compact.contains("entry-row--compact"));
        assert!(!compact.contains("Apfel"));
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use crate::model::{EntryMeta, EntryResult};
    use crate::model::{TimeValue, Tone};
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
        EntryRow,
        EntryRowProps {
            parts: parts(),
            compact: false,
        },
        (
            "Selectable, selected",
            EntryRowProps {
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
            EntryRowProps {
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
            EntryRowProps {
                parts: parts(),
                compact: true,
            }
        ),
    );
}

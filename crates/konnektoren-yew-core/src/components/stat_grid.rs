use yew::prelude::*;

/// One figure in a [`StatGrid`].
#[derive(Clone, Default, PartialEq)]
pub struct Stat {
    pub icon: Option<Html>,
    /// Translated.
    pub label: AttrValue,
    pub value: AttrValue,
    /// Stands out, e.g. the rating.
    pub highlight: bool,
}

impl Stat {
    pub fn new(label: impl Into<AttrValue>, value: impl Into<AttrValue>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            ..Self::default()
        }
    }

    pub fn with_icon(mut self, icon: Html) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn highlighted(mut self) -> Self {
        self.highlight = true;
        self
    }
}

#[derive(Properties, PartialEq)]
pub struct StatGridProps {
    pub stats: Vec<Stat>,
    #[prop_or_default]
    pub class: Classes,
}

/// Figures in tiles, e.g. rating, wins and losses above a history.
#[function_component(StatGrid)]
pub fn stat_grid(props: &StatGridProps) -> Html {
    html! {
        <dl class={classes!("stat-grid", props.class.clone())}>
            {for props.stats.iter().map(|stat| html! {
                <div class={classes!("stat-grid__tile", stat.highlight.then_some("stat-grid__tile--highlight"))}>
                    <dt class="stat-grid__label">
                        if let Some(icon) = &stat.icon {
                            <span class="stat-grid__icon" aria-hidden="true">{icon.clone()}</span>
                        }
                        {stat.label.clone()}
                    </dt>
                    <dd class="stat-grid__value">{stat.value.clone()}</dd>
                </div>
            })}
        </dl>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        StatGrid,
        StatGridProps {
            stats: vec![
                Stat::new("Rating", "1012")
                    .with_icon(html! { "📈" })
                    .highlighted(),
                Stat::new("Wins", "12").with_icon(html! { "🏆" }),
                Stat::new("Losses", "7").with_icon(html! { "💔" }),
                Stat::new("Draws", "2").with_icon(html! { "🤝" }),
            ],
            class: Classes::new(),
        },
        (
            "Two figures",
            StatGridProps {
                stats: vec![
                    Stat::new("Attempts", "14").highlighted(),
                    Stat::new("Mastered", "5"),
                ],
                class: Classes::new(),
            }
        ),
    );
}

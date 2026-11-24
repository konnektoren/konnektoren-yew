//! A small example history for previews and tests: practised pieces with a result,
//! a result filter, a favourites toggle, sort orders, a summary and a detail panel.
use crate::{
    DefaultHistoryEntry, Direction, EntryContext, EntryMeta, EntryResult, FilterContext,
    FilterControl, History, HistoryChange, HistoryContext, HistoryFilter, HistoryItem,
    HistoryLabels, HistoryOption, HistorySort, SortEntry, SortKey, Tone,
};
use chrono::{DateTime, TimeZone, Utc};
use konnektoren_yew_core::{Stat, StatGrid};
use yew::prelude::*;

#[cfg(feature = "challenge")]
pub mod challenge;

/// One practised piece.
#[derive(Clone, Debug, PartialEq)]
pub struct Session {
    pub piece: &'static str,
    pub score: u32,
    /// Day in November 2026; `None` for an undated session.
    pub day: Option<u32>,
    pub favourite: bool,
}

impl HistoryItem for Session {
    fn key(&self) -> AttrValue {
        self.piece.into()
    }

    fn time(&self) -> Option<DateTime<Utc>> {
        self.day.and_then(|day| {
            Utc.with_ymd_and_hms(2026, 11, day, 9 + day % 8, 30, 0)
                .single()
        })
    }

    fn name(&self) -> AttrValue {
        self.piece.into()
    }
}

#[derive(Properties, PartialEq)]
pub struct SampleHistoryProps {
    /// `Err` shows the error state.
    pub sessions: Result<Vec<Session>, AttrValue>,
}

/// Page state: the selected piece.
pub struct SampleHistory {
    selected: Option<AttrValue>,
}

pub enum SampleMsg {
    Select(AttrValue),
    Close,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Score {
    #[default]
    All,
    Strong,
    Weak,
}

impl HistoryOption for Score {
    const ALL: &'static [Self] = &[Self::All, Self::Strong, Self::Weak];
    fn value(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Strong => "strong",
            Self::Weak => "weak",
        }
    }
    fn label(self) -> &'static str {
        match self {
            Self::All => "All results",
            Self::Strong => "80% and more",
            Self::Weak => "Below 60%",
        }
    }
}

impl HistoryFilter<SampleHistory> for Score {
    const ID: &'static str = "result";
    const LABEL: &'static str = "Result";
    const COUNTS: bool = true;
    fn admits(self, session: &Session, _: &FilterContext<'_, SampleHistory>) -> bool {
        match self {
            Self::All => true,
            Self::Strong => session.score >= 80,
            Self::Weak => session.score < 60,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Favourites {
    #[default]
    All,
    Only,
}

impl HistoryOption for Favourites {
    const ALL: &'static [Self] = &[Self::All, Self::Only];
    fn value(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Only => "only",
        }
    }
    fn label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Only => "Favourites",
        }
    }
}

impl HistoryFilter<SampleHistory> for Favourites {
    const ID: &'static str = "favourites";
    const LABEL: &'static str = "Favourites";
    const CONTROL: FilterControl = FilterControl::Toggle { icon: "fa-star" };
    const COUNTS: bool = true;
    fn admits(self, session: &Session, _: &FilterContext<'_, SampleHistory>) -> bool {
        self == Self::All || session.favourite
    }
    fn disabled(ctx: &FilterContext<'_, SampleHistory>) -> bool {
        !matches!(&ctx.props.sessions, Ok(sessions) if sessions.iter().any(|s| s.favourite))
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SampleSort {
    #[default]
    Newest,
    Oldest,
    Best,
    Piece,
}

impl HistoryOption for SampleSort {
    const ALL: &'static [Self] = &[Self::Newest, Self::Oldest, Self::Best, Self::Piece];
    fn value(self) -> &'static str {
        match self {
            Self::Newest => "newest",
            Self::Oldest => "oldest",
            Self::Best => "best",
            Self::Piece => "piece",
        }
    }
    fn label(self) -> &'static str {
        match self {
            Self::Newest => "Newest first",
            Self::Oldest => "Oldest first",
            Self::Best => "Best result first",
            Self::Piece => "Piece",
        }
    }
}

impl HistorySort<SampleHistory> for SampleSort {
    fn key(self, entry: &SortEntry<'_, Session>) -> SortKey {
        match self {
            Self::Newest | Self::Oldest => SortKey::Base,
            Self::Best => SortKey::Number(Some(entry.item.score.into())),
            Self::Piece => SortKey::Text(entry.name.to_owned().into()),
        }
    }
    fn direction(self) -> Direction {
        match self {
            Self::Oldest | Self::Piece => Direction::Ascending,
            Self::Newest | Self::Best => Direction::Descending,
        }
    }
}

impl History for SampleHistory {
    type Item = Session;
    type Filters = (Score, Favourites);
    type Sort = SampleSort;
    type Properties = SampleHistoryProps;
    type Message = SampleMsg;

    const PAGE: usize = 8;

    fn create(_: &HistoryContext<'_, Self>) -> Self {
        Self { selected: None }
    }

    fn items(props: &SampleHistoryProps) -> Result<Vec<Session>, AttrValue> {
        props.sessions.clone()
    }

    fn labels() -> HistoryLabels {
        HistoryLabels {
            title: "Practice history",
            intro: "Every piece you practised. Search, filter and sort your sessions.",
            search_label: "Search pieces",
            search_placeholder: "Piece",
            noun: "Sessions",
            empty: "No sessions yet.",
            no_match: "No sessions match these filters.",
        }
    }

    fn update(&mut self, _: &HistoryContext<'_, Self>, msg: SampleMsg) -> HistoryChange {
        self.selected = match msg {
            SampleMsg::Select(piece) => Some(piece),
            SampleMsg::Close => None,
        };
        HistoryChange::Render
    }

    fn view_summary(&self, ctx: &HistoryContext<'_, Self>, sessions: &[Session]) -> Html {
        let best = sessions.iter().map(|s| s.score).max();
        let tiles = vec![
            Stat::new(ctx.t("Sessions"), sessions.len().to_string())
                .with_icon(html! { "🎹" })
                .highlighted(),
            Stat::new(
                ctx.t("Best result"),
                best.map_or("—".into(), |b| format!("{b}%")),
            )
            .with_icon(html! { "🏆" }),
            Stat::new(
                ctx.t("Favourites"),
                sessions.iter().filter(|s| s.favourite).count().to_string(),
            )
            .with_icon(html! { "⭐" }),
        ];
        html! { <StatGrid stats={tiles} /> }
    }

    fn view_aside(&self, ctx: &HistoryContext<'_, Self>) -> Option<Html> {
        let piece = self.selected.clone()?;
        let sessions = SampleHistory::items(ctx.props()).unwrap_or_default();
        let count = sessions
            .iter()
            .filter(|s| s.piece == piece.as_str())
            .count();
        Some(html! {
            <div class="history-sample__aside">
                <h2>{piece.clone()}</h2>
                <p>{format!("{}: {count}", ctx.t("Sessions"))}</p>
                <button type="button" class="btn btn-ghost btn-sm" onclick={ctx.callback(|_: MouseEvent| SampleMsg::Close)}>
                    {ctx.t("Close")}
                </button>
            </div>
        })
    }

    fn view_empty_action(&self, ctx: &HistoryContext<'_, Self>) -> Html {
        html! { <a href="#" class="btn btn-primary btn-sm">{ctx.t("Practice")}</a> }
    }
}

impl DefaultHistoryEntry<SampleHistory> for Session {
    fn icon(&self, _: &EntryContext<'_, SampleHistory>) -> Option<Html> {
        Some(html! { "🎼" })
    }

    fn selected(&self, ctx: &EntryContext<'_, SampleHistory>) -> bool {
        ctx.history().selected.as_deref() == Some(self.piece)
    }

    fn on_select(&self, ctx: &EntryContext<'_, SampleHistory>) -> Option<Callback<MouseEvent>> {
        let piece = self.key();
        Some(ctx.callback(move |_| SampleMsg::Select(piece.clone())))
    }

    fn meta(&self, ctx: &EntryContext<'_, SampleHistory>) -> Vec<EntryMeta> {
        let mut meta = vec![EntryMeta::rating_change(
            1000.0,
            1000.0 + self.score as f64 - 60.0,
        )];
        if self.favourite {
            meta.push(
                EntryMeta::text(ctx.t("Favourite"))
                    .with_icon(html! { <i class="fa-solid fa-star"></i> })
                    .with_tone(Tone::Info),
            );
        }
        meta
    }

    fn result(&self, ctx: &EntryContext<'_, SampleHistory>) -> Option<EntryResult> {
        Some(EntryResult::score(self.score, 80, 60).with_label(ctx.t("Result")))
    }
}

const PIECES: [&str; 6] = [
    "Little Waltz",
    "Silent Night",
    "Five-Note Walk",
    "O Tannenbaum",
    "Ode to Joy",
    "Jingle Bells",
];

/// `count` sessions, one or two a day, with varied results; every fifth a favourite.
pub fn sample_sessions(count: usize) -> Vec<Session> {
    (0..count)
        .map(|index| Session {
            piece: PIECES[index * 7 % PIECES.len()],
            score: ((index * 37 + 25) % 101) as u32,
            day: Some(1 + (index / 2) as u32 % 28),
            favourite: index % 5 == 2,
        })
        .collect()
}

pub fn sample_props(count: usize) -> SampleHistoryProps {
    SampleHistoryProps {
        sessions: Ok(sample_sessions(count)),
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use crate::{HistoryPreview, HistoryPreviewProps, HistoryView};
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        HistoryView<SampleHistory>,
        sample_props(20),
        ("A few sessions", sample_props(3)),
        ("No sessions", sample_props(0)),
        (
            "Unreadable",
            SampleHistoryProps {
                sessions: Err("Could not load the history.".into()),
            }
        ),
    );

    yew_preview::create_preview!(
        HistoryPreview<SampleHistory>,
        HistoryPreviewProps {
            data: sample_props(12),
            limit: 5,
            more: html! {
                <a href="#" class="btn btn-ghost btn-sm">
                    <i class="fa-solid fa-clock-rotate-left" aria-hidden="true"></i>{"Full history"}
                </a>
            },
            title: None,
        },
        (
            "Empty",
            HistoryPreviewProps {
                data: sample_props(0),
                limit: 5,
                more: Html::default(),
                title: None,
            }
        ),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{HistoryFilters, HistoryPreview, HistoryPreviewProps, HistoryView};
    use yew::ServerRenderer;

    fn filters_context<'a>(
        props: &'a SampleHistoryProps,
        history: &'a SampleHistory,
    ) -> FilterContext<'a, SampleHistory> {
        FilterContext { props, history }
    }

    #[test]
    fn options_round_trip() {
        for option in Score::ALL {
            assert_eq!(Score::parse(option.value()), *option);
        }
        for option in SampleSort::ALL {
            assert_eq!(SampleSort::parse(option.value()), *option);
        }
        assert_eq!(SampleSort::parse("unknown"), SampleSort::Newest);
    }

    #[test]
    fn filter_tuples_combine_set_and_describe_their_fields() {
        let props = SampleHistoryProps {
            sessions: Ok(vec![
                Session {
                    piece: "A",
                    score: 90,
                    day: Some(1),
                    favourite: true,
                },
                Session {
                    piece: "B",
                    score: 90,
                    day: Some(2),
                    favourite: false,
                },
            ]),
        };
        let history = SampleHistory { selected: None };
        let ctx = filters_context(&props, &history);
        let sessions = props.sessions.clone().unwrap();
        let mut filters = <(Score, Favourites)>::default();
        assert!(!HistoryFilters::<SampleHistory>::active(&filters));
        assert!(HistoryFilters::<SampleHistory>::set(
            &mut filters,
            "favourites",
            "only"
        ));
        assert!(!HistoryFilters::<SampleHistory>::set(
            &mut filters,
            "missing",
            "x"
        ));
        assert!(HistoryFilters::<SampleHistory>::active(&filters));
        assert!(filters.admits(&sessions[0], &ctx));
        assert!(!filters.admits(&sessions[1], &ctx));
        let fields = filters.fields(&ctx);
        assert_eq!(
            fields.iter().map(|f| f.id).collect::<Vec<_>>(),
            ["result", "favourites"]
        );
        assert_eq!(fields[1].value, "only");
        assert_eq!(fields[1].control, FilterControl::Toggle { icon: "fa-star" });
        assert!(!fields[1].disabled);
    }

    #[tokio::test]
    async fn renders_a_page_with_summary_filters_count_and_paging() {
        let html = ServerRenderer::<HistoryView<SampleHistory>>::with_props(|| sample_props(20))
            .render()
            .await;
        assert!(html.contains("Practice history"));
        assert!(html.contains("stat-grid"));
        assert!(html.contains("20 / 20 Sessions"));
        assert!(html.contains("Favourites (4)"));
        assert!(html.contains("80% and more ("));
        assert_eq!(
            html.matches("class=\"filtered-list__item\"").count(),
            SampleHistory::PAGE
        );
        assert!(html.contains("Show more"));
        assert!(html.contains("aria-pressed=\"false\""));
    }

    #[tokio::test]
    async fn renders_empty_and_error_states() {
        let empty = ServerRenderer::<HistoryView<SampleHistory>>::with_props(|| sample_props(0))
            .render()
            .await;
        assert!(empty.contains("No sessions yet."));
        let error =
            ServerRenderer::<HistoryView<SampleHistory>>::with_props(|| SampleHistoryProps {
                sessions: Err("Could not load.".into()),
            })
            .render()
            .await;
        assert!(error.contains("role=\"alert\""));
        assert!(!error.contains("filtered-list__filters"));
    }

    #[tokio::test]
    async fn preview_shows_the_newest_compact_rows() {
        let html =
            ServerRenderer::<HistoryPreview<SampleHistory>>::with_props(|| HistoryPreviewProps {
                data: sample_props(12),
                limit: 3,
                more: html! { <a href="#">{"Full history"}</a> },
                title: None,
            })
            .render()
            .await;
        assert_eq!(html.matches("class=\"history-preview__item\"").count(), 3);
        assert!(html.contains("entry-row--compact"));
        assert!(html.contains("Full history"));
        // The newest session (day 6) comes first.
        assert!(html.find("2026-11-06").unwrap() < html.find("2026-11-05").unwrap());
    }
}

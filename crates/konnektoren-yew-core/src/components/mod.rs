//! Stateless components: they render their props and keep no state.
pub mod filtered_list;
mod icon;
mod pill;
mod stat_grid;
pub mod status;
mod time;

pub use filtered_list::{
    FilteredList, FilteredListProps, ListCount, ListCountProps, ListFilterField,
    ListFilterFieldProps, ListItem, ListRows, ListRowsProps, ListSearch, ListSearchProps,
    ListSortField, ListSortFieldProps,
};
pub use icon::{Icon, IconProps};
pub use pill::{Pill, PillProps};
pub use stat_grid::{Stat, StatGrid, StatGridProps};
pub use status::{
    EmptyState, EmptyStateProps, ErrorState, ErrorStateProps, LoadingState, LoadingStateProps,
    StatusMessage, StatusMessageProps,
};
pub use time::{Time, TimeProps};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ListChoice, ListFilter, ListFilterControl, TimeValue, Tone};
    use yew::prelude::*;

    #[function_component(Sample)]
    fn sample() -> Html {
        html! {
            <>
                <Pill text="80%" tone={Tone::Good} label="Result" />
                <StatGrid stats={vec![Stat::new("Wins", "12").highlighted()]} />
                <Time value={TimeValue::Raw("2026-11-21T09:05:00.000Z".into())} />
                <EmptyState message="Nothing here yet." />
                <FilteredList
                    items={vec![ListItem::new("a", html! { "Apfel" })]}
                    matching={2} total={3} active=true
                    filters={vec![ListFilter {
                        id: "fav".into(), label: "Favourites".into(),
                        control: ListFilterControl::Toggle { icon: "fa-star".into() },
                        value: "only".into(),
                        choices: vec![ListChoice::new("all", "All"), ListChoice::new("only", "Favourites (1)")],
                        disabled: true,
                    }]} />
            </>
        }
    }

    #[tokio::test]
    async fn renders_bem_blocks() {
        let html = yew::ServerRenderer::<Sample>::new().render().await;
        assert!(html.contains(r#"class="pill pill--good""#), "{html}");
        assert!(html.contains(r#"aria-label="Result: 80%""#));
        assert!(html.contains("stat-grid__tile stat-grid__tile--highlight"));
        assert!(html.contains(">2026-11-21 09:05</time>"));
        assert!(html.contains("Nothing here yet."));
        assert!(html.contains("2 / 3 Entries"));
        assert!(html.contains("filtered-list__toggle filtered-list__toggle--on"));
        assert!(html.contains(r#"aria-pressed="true""#));
        assert!(!html.contains("disabled"), "an active toggle stays enabled");
        assert!(html.contains("Clear filters"));
        assert!(html.contains("filtered-list__more"));
        assert!(html.contains("1 / 2"));
    }
}

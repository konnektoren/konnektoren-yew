use konnektoren_yew_core::Icon;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SeasonalEventCardProps {
    pub title: AttrValue,
    pub message: AttrValue,
    pub action_label: AttrValue,
    pub action_url: AttrValue,
    #[prop_or_default]
    pub icon: Option<String>,
}

#[function_component(SeasonalEventCard)]
pub fn seasonal_event_card(props: &SeasonalEventCardProps) -> Html {
    html! {
        <section class="seasonal-event" aria-label={props.title.clone()}>
            if let Some(icon) = &props.icon {
                <Icon icon={icon.clone()} alt={String::new()} class="seasonal-event__icon" />
            }
            <div class="seasonal-event__content">
                <h2 class="seasonal-event__title">{&props.title}</h2>
                <p>{&props.message}</p>
            </div>
            <a class="seasonal-event__action" href={props.action_url.clone()}>{&props.action_label}</a>
        </section>
    }
}

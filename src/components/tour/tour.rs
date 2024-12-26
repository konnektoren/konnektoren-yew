use yew::prelude::*;
use yew_tou_rs::prelude::{Tour as YewTour, TourConfig};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: String,
    pub data: String,
}

#[function_component(Tour)]
pub fn tour(props: &Props) -> Html {
    let config: TourConfig = serde_yaml::from_str(&props.data.clone()).unwrap();
    html! {
    <div>
        <YewTour steps={config.steps} id={props.id.clone()} />
    </div>
    }
}

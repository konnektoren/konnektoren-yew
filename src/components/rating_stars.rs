use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct RatingStarsProps {
    pub rating: f64,
    #[prop_or(Some(5))]
    pub max_stars: Option<usize>,
    #[prop_or_default]
    pub on_click: Option<Callback<usize>>,
}
#[function_component(RatingStarsComponent)]
pub fn rating_stars(props: &RatingStarsProps) -> Html {
    let max_stars = props.max_stars.unwrap_or(5);
    let rating = props.rating;
    let on_click = props.on_click.clone();

    let stars = (0..max_stars).map(|i| {
        let star_class = if (i+1) as f64 <= rating {
            "star full"
        } else if i as f64 + 0.5 <= rating {
            "star half"
        } else {
            "star empty"
        };

        match &on_click {
            Some(on_click) => {
                let on_click = on_click.reform(move |_: MouseEvent| i);
                html! {
                    <span id={format!("star-{}", i)} class={star_class} onclick={on_click.clone()}>{"★"}</span>
                }
            }
            None => html! {
                <span class={star_class}>{"★"}</span>
            },
        }
    });

    html! {
        <div class="rating-stars">
            { for stars }
        </div>
    }
}

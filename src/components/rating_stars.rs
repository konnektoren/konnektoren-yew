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
        let star_class = classes!(
            "rating-stars__star",
            match () {
                _ if (i + 1) as f64 <= rating => "rating-stars__star--full",
                _ if i as f64 + 0.5 <= rating => "rating-stars__star--half",
                _ => "rating-stars__star--empty",
            }
        );

        match &on_click {
            Some(on_click) => {
                let on_click = on_click.reform(move |_| i);
                html! {
                    <span class={star_class} onclick={on_click}>{"★"}</span>
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

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        RatingStarsComponent,
        RatingStarsProps {
            rating: 3.5,
            max_stars: Some(5),
            on_click: None
        },
        (
            "4.5 out of 10",
            RatingStarsProps {
                rating: 4.5,
                max_stars: Some(10),
                on_click: None
            }
        ),
        (
            "5 out of 5",
            RatingStarsProps {
                rating: 5.0,
                max_stars: Some(5),
                on_click: None
            }
        ),
        (
            "0 stars",
            RatingStarsProps {
                rating: 0.0,
                max_stars: Some(5),
                on_click: None
            }
        ),
    );
}

use crate::{RatingStarsComponent, Review};
use konnektoren_yew_core::{FilteredList, ListItem, ListLabels};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ReviewListProps {
    pub reviews: Vec<Review>,
    pub challenge_label: String,
    /// Translated list controls and empty-state messages.
    #[prop_or_default]
    pub labels: ListLabels,
}

#[function_component(ReviewList)]
pub fn review_list(props: &ReviewListProps) -> Html {
    let search = use_state(String::new);
    let query = search.to_lowercase();
    let items: Vec<ListItem> = props.reviews.iter().enumerate()
        .filter(|(_, review)| review.challenge_id.to_lowercase().contains(&query)
            || review.comment.as_deref().unwrap_or_default().to_lowercase().contains(&query))
        .map(|(index, review)| ListItem::new(index.to_string(), html! {
            <div class="review-list__item">
                <div class="review-list__rating">
                    <RatingStarsComponent rating={review.rating as f64} />
                </div>
                <div class="review-list__comment">{review.comment.as_deref().unwrap_or_default()}</div>
                <div class="review-list__challenge">{&props.challenge_label}{&review.challenge_id}</div>
            </div>
        })).collect();
    let matching = items.len();
    let on_search = {
        let search = search.clone();
        Callback::from(move |value| search.set(value))
    };
    let on_clear = {
        let search = search.clone();
        Callback::from(move |()| search.set(String::new()))
    };
    html! {
        <FilteredList class="review-list" labels={props.labels.clone()} {items} {matching}
            total={props.reviews.len()} search={(*search).clone()} active={!search.is_empty()}
            {on_search} {on_clear} />
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        ReviewList,
        ReviewListProps {
            reviews: vec![
                Review {
                    challenge_id: "articles-1".into(),
                    rating: 5,
                    comment: Some("A helpful exercise!".into())
                },
                Review {
                    challenge_id: "verbs-1".into(),
                    rating: 3,
                    comment: Some("More examples would help.".into())
                },
            ],
            challenge_label: "Challenge: ".into(),
            labels: ListLabels::default(),
        },
        (
            "Empty list",
            ReviewListProps {
                reviews: vec![],
                challenge_label: "Challenge: ".into(),
            labels: ListLabels::default(),
            }
        ),
        (
            "Without comment",
            ReviewListProps {
                reviews: vec![Review {
                    challenge_id: "vocabulary-1".into(),
                    rating: 4,
                    comment: None,
                }],
                challenge_label: "Exercise: ".into(),
                labels: ListLabels::default(),
            }
        ),
        (
            "Varied ratings and long comments",
            ReviewListProps {
                reviews: vec![
                    Review {
                        challenge_id: "grammar-1".into(),
                        rating: 1,
                        comment: Some("I would appreciate more explanations and examples before trying this exercise again.".into()),
                    },
                    Review {
                        challenge_id: "conversation-1".into(),
                        rating: 5,
                        comment: Some("Very helpful!".into()),
                    },
                ],
                challenge_label: "Exercise: ".into(),
                labels: ListLabels::default(),
            }
        ),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn renders_reviews_and_escapes_comments() {
        let html = yew::LocalServerRenderer::<ReviewList>::with_props(ReviewListProps {
            reviews: vec![Review {
                challenge_id: "lesson-1".into(),
                rating: 4,
                comment: Some("<useful>".into()),
            }],
            challenge_label: "Exercise: ".into(),
            labels: ListLabels::default(),
        })
        .render()
        .await;
        assert!(html.contains("&lt;useful&gt;"));
        assert!(html.contains("lesson-1"));
        assert_eq!(html.matches("rating-stars__star--full").count(), 4);
    }
}

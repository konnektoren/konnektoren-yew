use crate::{RatingStarsComponent, Review};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ReviewListProps {
    pub reviews: Vec<Review>,
    pub challenge_label: String,
}

#[function_component(ReviewList)]
pub fn review_list(props: &ReviewListProps) -> Html {
    html! {
        <div class="review-list">
            {for props.reviews.iter().map(|review| html! {
                <div class="review-list__item">
                    <div class="review-list__rating">
                        <RatingStarsComponent rating={review.rating as f64} />
                    </div>
                    <div class="review-list__comment">{review.comment.as_deref().unwrap_or_default()}</div>
                    <div class="review-list__challenge">
                        {&props.challenge_label}{&review.challenge_id}
                    </div>
                </div>
            })}
        </div>
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
        },
        (
            "Empty list",
            ReviewListProps {
                reviews: vec![],
                challenge_label: "Challenge: ".into(),
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
        })
        .render()
        .await;
        assert!(html.contains("&lt;useful&gt;"));
        assert!(html.contains("lesson-1"));
        assert_eq!(html.matches("rating-stars__star--full").count(), 4);
    }
}

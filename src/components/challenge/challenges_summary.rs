use crate::components::{SeoComponent, SeoConfig};
use konnektoren_core::{
    game::{Game, GamePath},
    prelude::ChallengeConfig,
};

use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ChallengesSummaryProps {
    pub game: Game,
    #[prop_or_default]
    pub config: ChallengesSummaryConfig,
}

#[derive(Clone, PartialEq, Default)]
pub struct ChallengesSummaryConfig {
    pub domain: String,
    pub site_name: String,
    pub title: String,
    pub description: String,
    pub image_url: String,
    pub keywords: String,
    pub author: String,
    pub language: String,
}

#[derive(Properties, Clone, PartialEq)]
pub struct ChallengeInfoBlockProps {
    pub challenge: ChallengeConfig,
}

#[function_component(ChallengeInfoBlock)]
pub fn challenge_info_block(props: &ChallengeInfoBlockProps) -> Html {
    let challenge = &props.challenge;

    html! {
        <div class="challenge-block">
            <div class="challenge-block__header">
                <h3 class="challenge-block__title">{&challenge.name}</h3>
                <span class="challenge-block__points">{format!("{} XP required", challenge.unlock_points)}</span>
            </div>
            <div class="challenge-block__content">
                <p class="challenge-block__description">{&challenge.description}</p>
                <div class="challenge-block__details">
                    <div class="challenge-block__detail">
                        <span class="challenge-block__detail-label">{"Type:"}</span>
                        <span class="challenge-block__detail-value">{&challenge.challenge}</span>
                    </div>
                    <div class="challenge-block__detail">
                        <span class="challenge-block__detail-label">{"Tasks:"}</span>
                        <span class="challenge-block__detail-value">{format!("{:?}", challenge.tasks)}</span>
                    </div>
                    if let Some(variant) = &challenge.variant {
                        <div class="challenge-block__detail">
                            <span class="challenge-block__detail-label">{"Variant:"}</span>
                            <span class="challenge-block__detail-value">{format!("{:?}", variant)}</span>
                        </div>
                    }
                </div>
                <div class="challenge-block__actions">
                    <a href={format!("/challenge/{}", &challenge.id)}
                       class="challenge-block__link"
                       target="_blank"
                       rel="noopener noreferrer">
                        {"View Challenge →"}
                    </a>
                </div>
            </div>
        </div>
    }
}

fn level_section(level: &GamePath) -> Html {
    html! {
        <section class="level-section">
            <div class="level-section__header">
                <h2 class="level-section__title">{&level.name}</h2>
                <div class="level-section__meta">
                    <span class="level-section__count">
                        {format!("{} Challenges", level.challenges.len())}
                    </span>
                    <span class="level-section__id">
                        {format!("ID: {}", level.id)}
                    </span>
                </div>
            </div>
            <div class="level-section__challenges">
                {for level.challenges.iter().map(|challenge| {
                    html! {
                        <ChallengeInfoBlock challenge={challenge.clone()} />
                    }
                })}
            </div>
        </section>
    }
}

#[function_component(ChallengesSummaryComp)]
pub fn challenges_summary(props: &ChallengesSummaryProps) -> Html {
    let levels = &props.game.game_paths;
    let config = &props.config;

    // Create challenges data for structured data
    let challenges_data: Vec<serde_json::Value> = levels
        .iter()
        .flat_map(|level| {
            level.challenges.iter().map(|challenge| {
                serde_json::json!({
                    "@type": "LearningResource",
                    "name": challenge.name,
                    "description": challenge.description,
                    "educationalLevel": level.name,
                    "learningResourceType": "Exercise",
                    "timeRequired": "PT10M",
                    "url": format!("{}/challenge/{}", config.domain, challenge.id)
                })
            })
        })
        .collect();

    // Create structured data
    let structured_data = serde_json::json!({
        "@context": "https://schema.org",
        "@type": ["WebPage", "ItemList", "Course"],
        "name": config.title,
        "description": config.description,
        "provider": {
            "@type": "Organization",
            "name": config.site_name,
            "url": config.domain
        },
        "about": {
            "@type": "Thing",
            "name": "German Grammar",
            "description": "Interactive exercises for learning German grammar"
        },
        "learningResourceType": "Exercise Collection",
        "teaches": ["German Grammar", "Language Learning"],
        "numberOfLessons": levels.iter().map(|l| l.challenges.len()).sum::<usize>(),
        "hasPart": challenges_data,
        "inLanguage": config.language,
        "author": {
            "@type": "Organization",
            "name": config.author
        }
    })
    .to_string();

    // Create SEO config
    let seo_config = SeoConfig::builder()
        .title(config.title.clone())
        .description(config.description.clone())
        .keywords(config.keywords.clone())
        .og_title(format!("{} - {}", config.site_name, config.title))
        .og_description(config.description.clone())
        .og_image(config.image_url.clone())
        .twitter_card("summary_large_image")
        .twitter_title(config.title.clone())
        .twitter_description(config.description.clone())
        .twitter_image(config.image_url.clone())
        .canonical_url(format!("{}/challenges", config.domain))
        .robots("index, follow")
        .author(config.author.clone())
        .language(config.language.clone())
        .structured_data(structured_data)
        .build();

    html! {
        <>
            <SeoComponent config={seo_config} />
            <div class="challenges-summary">
                <style>
                    { get_styles() }
                </style>
                <div class="challenges-summary__container">
                    <header class="challenges-summary__header">
                        <h1 class="challenges-summary__title">{&config.title}</h1>
                        <p class="challenges-summary__description">
                            {&config.description}
                        </p>
                    </header>
                    <div class="challenges-summary__content">
                        {for levels.iter().map(level_section)}
                    </div>
                </div>
            </div>
        </>
    }
}

pub fn get_styles() -> String {
    r#"
    .challenges-summary {
        font-family: system-ui, -apple-system, "Segoe UI", Roboto, sans-serif;
        color: #1a1a1a;
        line-height: 1.6;
        max-width: 1200px;
        margin: 0 auto;
        padding: 2rem;
    }

    .challenges-summary__container {
        background: #ffffff;
        border-radius: 8px;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    }

    .challenges-summary__header {
        padding: 3rem 2rem;
        background: #f8f9fa;
        border-radius: 8px 8px 0 0;
        border-bottom: 1px solid #e9ecef;
    }

    .challenges-summary__title {
        font-size: 2.5rem;
        font-weight: 700;
        margin: 0 0 1rem;
        color: #2c3e50;
    }

    .challenges-summary__description {
        font-size: 1.1rem;
        color: #495057;
        max-width: 800px;
    }

    .level-section {
        padding: 2rem;
        border-bottom: 1px solid #e9ecef;
    }

    .level-section__header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 2rem;
    }

    .level-section__title {
        font-size: 1.75rem;
        font-weight: 600;
        color: #2c3e50;
        margin: 0;
    }

    .level-section__meta {
        display: flex;
        gap: 1rem;
        font-size: 0.9rem;
        color: #6c757d;
    }

    .level-section__challenges {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
        gap: 1.5rem;
    }

    .challenge-block {
        background: #ffffff;
        border: 1px solid #dee2e6;
        border-radius: 6px;
        transition: transform 0.2s, box-shadow 0.2s;
    }

    .challenge-block:hover {
        transform: translateY(-2px);
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    }

    .challenge-block__header {
        padding: 1rem;
        border-bottom: 1px solid #e9ecef;
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .challenge-block__title {
        font-size: 1.25rem;
        font-weight: 600;
        margin: 0;
        color: #2c3e50;
    }

    .challenge-block__points {
        font-size: 0.875rem;
        color: #6c757d;
        background: #e9ecef;
        padding: 0.25rem 0.5rem;
        border-radius: 4px;
    }

    .challenge-block__content {
        padding: 1rem;
    }

    .challenge-block__description {
        margin: 0 0 1rem;
        color: #495057;
    }

    .challenge-block__details {
        margin-bottom: 1rem;
        display: grid;
        gap: 0.5rem;
    }

    .challenge-block__detail {
        display: flex;
        font-size: 0.875rem;
    }

    .challenge-block__detail-label {
        font-weight: 500;
        color: #495057;
        width: 80px;
    }

    .challenge-block__detail-value {
        color: #6c757d;
    }

    .challenge-block__actions {
        border-top: 1px solid #e9ecef;
        padding-top: 1rem;
        text-align: right;
    }

    .challenge-block__link {
        display: inline-block;
        text-decoration: none;
        color: #007bff;
        font-weight: 500;
        transition: color 0.2s;
    }

    .challenge-block__link:hover {
        color: #0056b3;
    }

    @media (max-width: 768px) {
        .challenges-summary {
            padding: 1rem;
        }

        .level-section__challenges {
            grid-template-columns: 1fr;
        }

        .challenges-summary__header {
            padding: 2rem 1rem;
        }

        .challenges-summary__title {
            font-size: 2rem;
        }
    }
    "#
    .to_string()
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        ChallengesSummaryComp,
        ChallengesSummaryProps {
            game: Game::default(),
            config: ChallengesSummaryConfig {
                site_name: "Konnektoren".to_string(),
                title: "German Grammar Challenges Overview".to_string(),
                description:
                    "A comprehensive collection of interactive exercises to master German grammar."
                        .to_string(),
                keywords: "German Grammar,Interactive Learning,Language Practice,Grammar Exercises"
                    .to_string(),
                author: "Konnektoren".to_string(),
                language: "en".to_string(),
                ..ChallengesSummaryConfig::default()
            }
        },
    );
}

use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub bluesky: Option<String>,
    #[prop_or_default]
    pub telegram: Option<String>,
    #[prop_or_default]
    pub twitter: Option<String>,
    #[prop_or_default]
    pub github: Option<String>,
    #[prop_or_default]
    pub web: Option<String>,
}

#[function_component(SocialLinks)]
pub fn social_links(props: &Props) -> Html {
    let bluesky_link = if let Some(bluesky) = &props.bluesky {
        html! {
            <a href={bluesky.clone()} target="_blank" rel="noopener">
                <i class="fab fa-bluesky"></i>
            </a>
        }
    } else {
        html! {}
    };

    let telegram_link = if let Some(telegram) = &props.telegram {
        html! {
            <a href={telegram.clone()} target="_blank" rel="noopener">
                <i class="fab fa-telegram"></i>
            </a>
        }
    } else {
        html! {}
    };

    let twitter_link = if let Some(twitter) = &props.twitter {
        html! {
            <a href={twitter.clone()} target="_blank" rel="noopener">
                <i class="fab fa-twitter"></i>
            </a>
        }
    } else {
        html! {}
    };

    let github_link = if let Some(github) = &props.github {
        html! {
            <a href={github.clone()} target="_blank" rel="noopener">
                <i class="fab fa-github"></i>
            </a>
        }
    } else {
        html! {}
    };

    let web_link = if let Some(web) = &props.web {
        html! {
            <a href={web.clone()} target="_blank" rel="noopener">
                <i class="fas fa-globe"></i>
            </a>
        }
    } else {
        html! {}
    };

    html! {
        <div class="social-links">
            {bluesky_link}
            {telegram_link}
            {twitter_link}
            {github_link}
            {web_link}
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        SocialLinks,
        Props {
            bluesky: Some("https://bluesky.com".to_string()),
            telegram: Some("https://telegram.com".to_string()),
            twitter: Some("https://twitter.com".to_string()),
            github: Some("https://github.com".to_string()),
            web: Some("https://info.konnektoren.help".to_string()),
        },
    );
}

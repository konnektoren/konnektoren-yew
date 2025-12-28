use yew::prelude::*;

#[derive(Clone, PartialEq, Properties, Default)]
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
    #[prop_or_default]
    pub youtube: Option<String>,
}

#[function_component(SocialLinks)]
pub fn social_links(props: &Props) -> Html {
    let render_link = |url: &Option<String>, icon_class: &str, label: &str| -> Html {
        match url {
            Some(url) => {
                let icon_class = icon_class.to_string();
                let label = label.to_string();

                html! {
                    <a
                        href={url.clone()}
                        target="_blank"
                        rel="noopener noreferrer"
                        class="social-links__link"
                        aria-label={label.clone()}
                        title={label}
                    >
                        <i class={icon_class}></i>
                    </a>
                }
            }
            None => html! {},
        }
    };

    html! {
        <div class="social-links">
            {render_link(&props.bluesky, "fa-brands fa-bluesky", "Bluesky")}
            {render_link(&props.telegram, "fa-brands fa-telegram", "Telegram")}
            {render_link(&props.twitter, "fa-brands fa-twitter", "Twitter")}
            {render_link(&props.github, "fa-brands fa-github", "GitHub")}
            {render_link(&props.web, "fa-solid fa-globe", "Website")}
            {render_link(&props.youtube, "fa-brands fa-youtube", "YouTube")}
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        SocialLinks,
        Props::default(), // Default preview (empty)
        (
            "All social links",
            Props {
                bluesky: Some("https://bluesky.com".to_string()),
                telegram: Some("https://telegram.com".to_string()),
                twitter: Some("https://twitter.com".to_string()),
                github: Some("https://github.com".to_string()),
                web: Some("https://info.konnektoren.help".to_string()),
                youtube: Some("https://youtube.com/@KonnektorenHelp".to_string()),
            }
        ),
        (
            "Single link (GitHub only)",
            Props {
                github: Some("https://github.com/konnektoren".to_string()),
                ..Props::default()
            }
        ),
        (
            "Three links",
            Props {
                telegram: Some("https://t.me/konnektoren".to_string()),
                twitter: Some("https://twitter.com/konnektoren".to_string()),
                github: Some("https://github.com/konnektoren".to_string()),
                ..Props::default()
            }
        )
    );
}

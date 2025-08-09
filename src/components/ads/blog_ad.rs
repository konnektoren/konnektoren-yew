use crate::i18n::use_i18n;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct BlogAdProps {
    pub blog_url: String,
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub preview_text: Option<String>,
    #[prop_or_default]
    pub preview_image: Option<String>,
}

#[function_component(BlogAdComponent)]
pub fn blog_ad_component(props: &BlogAdProps) -> Html {
    let i18n = use_i18n();

    let default_title = i18n.t("Visit our Blog");
    let default_preview = i18n.t(
        "Discover tips, tutorials, and insights about language learning and connecting cultures.",
    );

    let title = props.title.as_ref().unwrap_or(&default_title);
    let preview_text = props.preview_text.as_ref().unwrap_or(&default_preview);

    html! {
        <div class="blog-ad">
            <div class="blog-ad__header">
                <div class="blog-ad__icon">
                    {"📝"}
                </div>
                <div class="blog-ad__title">
                    { title }
                </div>
            </div>

            if let Some(ref image_url) = props.preview_image {
                <div class="blog-ad__image">
                    <img src={image_url.clone()} alt="Blog preview" />
                </div>
            }

            <div class="blog-ad__content">
                <p class="blog-ad__preview">
                    { preview_text }
                </p>
            </div>

            <div class="blog-ad__actions">
                <a
                    class="blog-ad__button"
                    href={props.blog_url.clone()}
                    target="_blank"
                    rel="noopener"
                >
                    { i18n.t("Read our Blog") }
                    <span class="blog-ad__arrow">{"→"}</span>
                </a>
            </div>

            <div class="blog-ad__note">
                { i18n.t("Free articles & tutorials") }
            </div>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        BlogAdComponent,
        BlogAdProps {
            blog_url: "https://www.konnektoren.blog/".to_string(),
            title: Some("Konnektoren Blog".to_string()),
            preview_text: Some("Explore our latest articles on language learning, cultural connections, and educational insights.".to_string()),
            preview_image: Some("https://konnektoren.help/favicon.png".to_string()),
        },
        (
            "minimal",
            BlogAdProps {
                blog_url: "https://www.konnektoren.blog/".to_string(),
                title: None,
                preview_text: None,
                preview_image: None,
            }
        )
    );
}

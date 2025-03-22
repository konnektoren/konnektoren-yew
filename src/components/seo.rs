use yew::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub struct SeoConfig {
    pub title: Option<String>,
    pub description: Option<String>,
    pub keywords: Option<String>,
    pub og_title: Option<String>,
    pub og_description: Option<String>,
    pub og_image: Option<String>,
    pub twitter_card: Option<String>,
    pub twitter_title: Option<String>,
    pub twitter_description: Option<String>,
    pub twitter_image: Option<String>,
    pub canonical_url: Option<String>,
    pub robots: Option<String>,
    pub author: Option<String>,
    pub language: Option<String>,
    pub structured_data: Option<String>, // JSON-LD
}

impl SeoConfig {
    pub fn builder() -> SeoConfigBuilder {
        SeoConfigBuilder::default()
    }
}

#[derive(Default)]
pub struct SeoConfigBuilder {
    config: SeoConfig,
}

impl SeoConfigBuilder {
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.config.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.config.description = Some(description.into());
        self
    }

    pub fn keywords(mut self, keywords: impl Into<String>) -> Self {
        self.config.keywords = Some(keywords.into());
        self
    }

    pub fn og_title(mut self, og_title: impl Into<String>) -> Self {
        self.config.og_title = Some(og_title.into());
        self
    }

    pub fn og_description(mut self, og_description: impl Into<String>) -> Self {
        self.config.og_description = Some(og_description.into());
        self
    }

    pub fn og_image(mut self, og_image: impl Into<String>) -> Self {
        self.config.og_image = Some(og_image.into());
        self
    }

    pub fn twitter_card(mut self, twitter_card: impl Into<String>) -> Self {
        self.config.twitter_card = Some(twitter_card.into());
        self
    }

    pub fn twitter_title(mut self, twitter_title: impl Into<String>) -> Self {
        self.config.twitter_title = Some(twitter_title.into());
        self
    }

    pub fn twitter_description(mut self, twitter_description: impl Into<String>) -> Self {
        self.config.twitter_description = Some(twitter_description.into());
        self
    }

    pub fn twitter_image(mut self, twitter_image: impl Into<String>) -> Self {
        self.config.twitter_image = Some(twitter_image.into());
        self
    }

    pub fn canonical_url(mut self, canonical_url: impl Into<String>) -> Self {
        self.config.canonical_url = Some(canonical_url.into());
        self
    }

    pub fn robots(mut self, robots: impl Into<String>) -> Self {
        self.config.robots = Some(robots.into());
        self
    }

    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.config.author = Some(author.into());
        self
    }

    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.config.language = Some(language.into());
        self
    }

    pub fn structured_data(mut self, data: impl Into<String>) -> Self {
        self.config.structured_data = Some(data.into());
        self
    }

    pub fn build(self) -> SeoConfig {
        self.config
    }
}

#[derive(Properties, PartialEq)]
pub struct SeoProps {
    pub config: SeoConfig,
}

#[function_component(SeoComponent)]
pub fn seo(props: &SeoProps) -> Html {
    #[cfg(feature = "csr")]
    {
        use gloo::utils::document;
        use wasm_bindgen::JsCast;
        use web_sys::{Element, HtmlMetaElement};

        use_effect_with(props.config.clone(), move |config| {
            let document = document();
            let head = document.head().expect("No <head> element found");

            // Helper function to update meta tag
            let update_meta_tag = {
                let document = document.clone();
                let head = head.clone();
                move |name: &str, content: &str| {
                    if let Some(element) = document
                        .query_selector(&format!("meta[name='{}']", name))
                        .unwrap()
                    {
                        let meta = element.dyn_into::<HtmlMetaElement>().unwrap();
                        meta.set_content(content);
                    } else {
                        let meta = document
                            .create_element("meta")
                            .unwrap()
                            .dyn_into::<HtmlMetaElement>()
                            .unwrap();
                        meta.set_attribute("name", name).unwrap();
                        meta.set_content(content);
                        head.append_child(&meta).unwrap();
                    }
                }
            };

            let update_og_meta_tag = {
                let document = document.clone();
                let head = head.clone();
                move |property: &str, content: &str| {
                    if let Some(element) = document
                        .query_selector(&format!("meta[property='og:{}']", property))
                        .unwrap()
                    {
                        let meta = element.dyn_into::<HtmlMetaElement>().unwrap();
                        meta.set_content(content);
                    } else {
                        let meta = document
                            .create_element("meta")
                            .unwrap()
                            .dyn_into::<HtmlMetaElement>()
                            .unwrap();
                        meta.set_attribute("property", &format!("og:{}", property))
                            .unwrap();
                        meta.set_content(content);
                        head.append_child(&meta).unwrap();
                    }
                }
            };

            let update_twitter_meta_tag = {
                let document = document.clone();
                let head = head.clone();
                move |name: &str, content: &str| {
                    if let Some(element) = document
                        .query_selector(&format!("meta[name='twitter:{}']", name))
                        .unwrap()
                    {
                        let meta = element.dyn_into::<HtmlMetaElement>().unwrap();
                        meta.set_content(content);
                    } else {
                        let meta = document
                            .create_element("meta")
                            .unwrap()
                            .dyn_into::<HtmlMetaElement>()
                            .unwrap();
                        meta.set_attribute("name", &format!("twitter:{}", name))
                            .unwrap();
                        meta.set_content(content);
                        head.append_child(&meta).unwrap();
                    }
                }
            };

            let update_structured_data = {
                let document = document.clone();
                let head = head.clone();
                move |content: &str| {
                    if let Some(element) = document
                        .query_selector("script[type='application/ld+json']")
                        .unwrap()
                    {
                        element.set_text_content(Some(content));
                    } else {
                        let script = document.create_element("script").unwrap();
                        script.set_attribute("type", "application/ld+json").unwrap();
                        script.set_text_content(Some(content));
                        head.append_child(&script).unwrap();
                    }
                }
            };

            // Update title if provided
            if let Some(title) = &config.title {
                document.set_title(title);
            }

            // Update basic meta tags if provided
            if let Some(description) = &config.description {
                update_meta_tag("description", description);
            }
            if let Some(keywords) = &config.keywords {
                update_meta_tag("keywords", keywords);
            }
            if let Some(author) = &config.author {
                update_meta_tag("author", author);
            }
            if let Some(robots) = &config.robots {
                update_meta_tag("robots", robots);
            }

            // Update Open Graph meta tags if provided
            if let Some(og_title) = &config.og_title {
                update_og_meta_tag("title", og_title);
            }
            if let Some(og_description) = &config.og_description {
                update_og_meta_tag("description", og_description);
            }
            if let Some(og_image) = &config.og_image {
                update_og_meta_tag("image", og_image);
            }

            // Update Twitter meta tags if provided
            if let Some(twitter_card) = &config.twitter_card {
                update_twitter_meta_tag("card", twitter_card);
            }
            if let Some(twitter_title) = &config.twitter_title {
                update_twitter_meta_tag("title", twitter_title);
            }
            if let Some(twitter_description) = &config.twitter_description {
                update_twitter_meta_tag("description", twitter_description);
            }
            if let Some(twitter_image) = &config.twitter_image {
                update_twitter_meta_tag("image", twitter_image);
            }

            // Update canonical URL if provided
            if let Some(canonical_url) = &config.canonical_url {
                if let Some(existing_canonical) =
                    document.query_selector("link[rel='canonical']").unwrap()
                {
                    existing_canonical
                        .set_attribute("href", canonical_url)
                        .unwrap();
                } else {
                    let link: Element = document.create_element("link").unwrap();
                    link.set_attribute("rel", "canonical").unwrap();
                    link.set_attribute("href", canonical_url).unwrap();
                    head.append_child(&link).unwrap();
                }
            }

            // Update language if provided
            if let Some(language) = &config.language {
                if let Some(html_element) = document.document_element() {
                    html_element.set_attribute("lang", language).unwrap();
                }
            }

            // Update structured data if provided
            if let Some(structured_data) = &config.structured_data {
                update_structured_data(structured_data);
            }

            || ()
        });
    }

    html! {}
}

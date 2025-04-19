use crate::i18n::use_i18n;
use konnektoren_platform::domain::DomainConfig;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum DomainSelectorStyle {
    Dropdown,
    Cards,
    ButtonGroup,
    Pills,
}

#[derive(Properties, PartialEq)]
pub struct DomainSelectorProps<T: DomainConfig> {
    pub domains: Vec<T>,
    pub current_domain: String,
    pub on_domain_change: Callback<T>,
    #[prop_or(DomainSelectorStyle::Dropdown)]
    pub style: DomainSelectorStyle,
    #[prop_or(false)]
    pub show_descriptions: bool,
}

#[function_component(DomainSelectorComponent)]
pub fn domain_selector_component<T: DomainConfig + 'static>(
    props: &DomainSelectorProps<T>,
) -> Html {
    // Get i18n translations
    let i18n = use_i18n();

    // Translate UI elements
    let select_language_title = i18n.t("Select language domain");

    let on_select_change = {
        let domains = props.domains.clone();
        let on_domain_change = props.on_domain_change.clone();

        Callback::from(move |e: Event| {
            #[cfg(feature = "csr")]
            {
                use web_sys::HtmlSelectElement;
                if let Some(target) = e.target_dyn_into::<HtmlSelectElement>() {
                    let selected_value = target.value();

                    // Find the selected domain and call the callback
                    if let Some(selected_domain) = domains
                        .iter()
                        .find(|domain| domain.code() == selected_value)
                    {
                        on_domain_change.emit(selected_domain.clone());
                    }
                }
            }
        })
    };

    match props.style {
        DomainSelectorStyle::Dropdown => {
            html! {
                <div class="domain-selector">
                    <select
                        class="domain-selector__select"
                        onchange={on_select_change}
                        title={select_language_title.clone()}
                    >
                        {
                            props.domains.iter().map(|domain| {
                                // Translate domain name if possible
                                let domain_name = i18n.t(&domain.name());

                                html! {
                                    <option
                                        value={domain.code().to_string()}
                                        selected={props.current_domain == domain.code()}
                                        class="domain-selector__option"
                                    >
                                        {format!("{} {} {}",
                                            domain.icon(),
                                            domain_name,
                                            domain.icon() // Using icon as flag since there's no separate flag in DomainConfig
                                        )}
                                    </option>
                                }
                            }).collect::<Html>()
                        }
                    </select>
                </div>
            }
        }
        DomainSelectorStyle::Cards => {
            html! {
                <div class="domain-selector-cards">
                    <div class="domain-selector-cards__grid">
                        {props.domains.iter().map(|domain| {
                            let domain_code = domain.code().to_string();
                            let is_active = props.current_domain == domain_code;

                            // Translate domain name and description
                            let domain_name = i18n.t(&domain.name());
                            let description = domain.description()
                                .map(|d| i18n.t(&d));

                            let onclick = {
                                let domain = domain.clone();
                                let on_domain_change = props.on_domain_change.clone();

                                Callback::from(move |_| {
                                    on_domain_change.emit(domain.clone());
                                })
                            };

                            html! {
                                <div
                                    class={classes!(
                                        "domain-selector-cards__card",
                                        is_active.then(|| "domain-selector-cards__card--active")
                                    )}
                                    onclick={onclick}
                                >
                                    <div class="domain-selector-cards__card-body">
                                        <span class="domain-selector-cards__icon">{domain.icon()}</span>
                                        <span class="domain-selector-cards__name">{domain_name}</span>
                                        {
                                            if props.show_descriptions && description.is_some() {
                                                html! {
                                                    <span class="domain-selector-cards__description">
                                                        {description.unwrap()}
                                                    </span>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        }
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>
            }
        }
        DomainSelectorStyle::ButtonGroup => {
            html! {
                <div class="domain-selector-buttons">
                    {props.domains.iter().map(|domain| {
                        let domain_code = domain.code().to_string();
                        let is_active = props.current_domain == domain_code;

                        // Translate domain name and description
                        let domain_name = i18n.t(&domain.name());
                        let title = domain.description()
                            .map(|d| i18n.t(&d))
                            .unwrap_or_default();

                        let onclick = {
                            let domain = domain.clone();
                            let on_domain_change = props.on_domain_change.clone();

                            Callback::from(move |_| {
                                on_domain_change.emit(domain.clone());
                            })
                        };

                        html! {
                            <button
                                class={classes!(
                                    "domain-selector-buttons__button",
                                    is_active.then(|| "domain-selector-buttons__button--active")
                                )}
                                onclick={onclick}
                                title={title}
                            >
                                <span class="domain-selector-buttons__icon">{domain.icon()}</span>
                                <span class="domain-selector-buttons__name">{domain_name}</span>
                            </button>
                        }
                    }).collect::<Html>()}
                </div>
            }
        }
        DomainSelectorStyle::Pills => {
            html! {
                <div class="domain-selector-pills">
                    {props.domains.iter().map(|domain| {
                        let domain_code = domain.code().to_string();
                        let is_active = props.current_domain == domain_code;

                        // Translate domain name and description
                        let domain_name = i18n.t(&domain.name());
                        let title = domain.description()
                            .map(|d| i18n.t(&d))
                            .unwrap_or_default();

                        let onclick = {
                            let domain = domain.clone();
                            let on_domain_change = props.on_domain_change.clone();

                            Callback::from(move |_| {
                                on_domain_change.emit(domain.clone());
                            })
                        };

                        html! {
                            <div
                                class={classes!(
                                    "domain-selector-pills__pill",
                                    is_active.then(|| "domain-selector-pills__pill--active")
                                )}
                                onclick={onclick}
                                title={title}
                            >
                                <span class="domain-selector-pills__icon">{domain.icon()}</span>
                                <span class="domain-selector-pills__name">{domain_name}</span>
                            </div>
                        }
                    }).collect::<Html>()}
                </div>
            }
        }
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use konnektoren_platform::domain::LanguageDomainConfig;
    use yew_preview::prelude::*;

    fn domains() -> Vec<LanguageDomainConfig> {
        vec![
            LanguageDomainConfig {
                code: "de".to_string(),
                name: "Learn German".to_string(),
                base_path: "/de".to_string(),
                locale: "de-DE".to_string(),
                icon: "🇩🇪".to_string(),
                hostname: "konnektoren.help".to_string(),
                description: Some("Learn German with Konnektoren".to_string()),
            },
            LanguageDomainConfig {
                code: "en".to_string(),
                name: "Learn English".to_string(),
                base_path: "/en".to_string(),
                locale: "en-US".to_string(),
                icon: "🇬🇧".to_string(),
                hostname: "en.konnektoren.help".to_string(),
                description: Some("Learn English with Konnektoren".to_string()),
            },
            LanguageDomainConfig {
                code: "es".to_string(),
                name: "Learn Spanish".to_string(),
                base_path: "/es".to_string(),
                locale: "es-ES".to_string(),
                icon: "🇪🇸".to_string(),
                hostname: "es.konnektoren.help".to_string(),
                description: Some("Learn Spanish with Konnektoren".to_string()),
            },
        ]
    }

    yew_preview::create_preview!(
        DomainSelectorComponent<LanguageDomainConfig>,
        DomainSelectorProps {
            domains: domains(),
            current_domain: "de".to_string(),
            on_domain_change: Callback::from(|_| ()),
            style: DomainSelectorStyle::Dropdown,
            show_descriptions: false,
        },
        (
            "Cards",
            DomainSelectorProps {
                domains: domains(),
                current_domain: "de".to_string(),
                on_domain_change: Callback::from(|_| ()),
                style: DomainSelectorStyle::Cards,
                show_descriptions: false,
            }
        ),
        (
            "Cards with Descriptions",
            DomainSelectorProps {
                domains: domains(),
                current_domain: "de".to_string(),
                on_domain_change: Callback::from(|_| ()),
                style: DomainSelectorStyle::Cards,
                show_descriptions: true,
            }
        ),
        (
            "ButtonGroup",
            DomainSelectorProps {
                domains: domains(),
                current_domain: "de".to_string(),
                on_domain_change: Callback::from(|_| ()),
                style: DomainSelectorStyle::ButtonGroup,
                show_descriptions: false,
            }
        ),
        (
            "Dropdown",
            DomainSelectorProps {
                domains: domains(),
                current_domain: "de".to_string(),
                on_domain_change: Callback::from(|_| ()),
                style: DomainSelectorStyle::Dropdown,
                show_descriptions: false,
            }
        ),
        (
            "Pills",
            DomainSelectorProps {
                domains: domains(),
                current_domain: "de".to_string(),
                on_domain_change: Callback::from(|_| ()),
                style: DomainSelectorStyle::Pills,
                show_descriptions: false,
            }
        ),
    );
}

use konnektoren_platform::domain::DomainConfig;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DomainSelectorProps<T: DomainConfig> {
    pub domains: Vec<T>,
    pub current_domain: String,
    pub on_domain_change: Callback<T>,
}

#[function_component(DomainSelectorComponent)]
pub fn domain_selector_component<T: DomainConfig + 'static>(
    props: &DomainSelectorProps<T>,
) -> Html {
    let on_select_change = {
        let domains = props.domains.clone();
        let on_domain_change = props.on_domain_change.clone();

        Callback::from(move |e: Event| {
            let target = e.target_dyn_into::<HtmlSelectElement>().unwrap();
            let selected_value = target.value();

            // Find the selected domain and call the callback
            if let Some(selected_domain) = domains
                .iter()
                .find(|domain| domain.code() == selected_value)
            {
                on_domain_change.emit(selected_domain.clone());
            }
        })
    };

    html! {
        <div class="domain-selector">
            <select
                class="domain-selector__select"
                onchange={on_select_change}
                title="Select language domain"
            >
                {
                    props.domains.iter().map(|domain| {
                        html! {
                            <option
                                value={domain.code().to_string()}
                                selected={props.current_domain == domain.code()}
                                class="domain-selector__option"
                            >
                                {format!("{} {} {}",
                                    domain.icon(),
                                    domain.name(),
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

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use konnektoren_platform::domain::LanguageDomainConfig;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        DomainSelectorComponent<LanguageDomainConfig>,
        DomainSelectorProps {
            domains: vec![
                LanguageDomainConfig {
                    code: "de".to_string(),
                    name: "Learn German".to_string(),
                    base_path: "/de".to_string(),
                    locale: "de-DE".to_string(),
                    icon: "🇩🇪".to_string(),
                    hostname: "konnektoren.help".to_string(),
                },
                LanguageDomainConfig {
                    code: "en".to_string(),
                    name: "Learn English".to_string(),
                    base_path: "/en".to_string(),
                    locale: "en-US".to_string(),
                    icon: "🇬🇧".to_string(),
                    hostname: "en.konnektoren.help".to_string(),
                },
                LanguageDomainConfig {
                    code: "es".to_string(),
                    name: "Learn Spanish".to_string(),
                    base_path: "/es".to_string(),
                    locale: "es-ES".to_string(),
                    icon: "🇪🇸".to_string(),
                    hostname: "es.konnektoren.help".to_string(),
                },
            ],
            current_domain: "de".to_string(),
            on_domain_change: Callback::from(|_| ()),
        },
    );
}

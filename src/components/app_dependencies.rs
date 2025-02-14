use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct License {
    expression: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Component {
    name: String,
    version: String,
    #[serde(default)]
    licenses: Option<Vec<License>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CycloneDX {
    components: Vec<Component>,
}

fn parse_license_expression(expr: &str) -> Vec<(String, Vec<String>)> {
    expr.split(" OR ")
        .map(|or_part| {
            let requirements: Vec<String> = or_part
                .split(" AND ")
                .map(|s| {
                    s.trim()
                        .trim_matches(|c| c == '(' || c == ')')
                        .split_whitespace()
                        .next()
                        .unwrap_or(s.trim())
                        .to_string()
                })
                .collect();

            match requirements.split_first() {
                Some((main, rest)) => (main.clone(), rest.to_vec()),
                None => (String::from("Unknown"), vec![]),
            }
        })
        .collect()
}

#[function_component(AppDependenciesComponent)]
pub fn app_dependencies() -> Html {
    let sbom: CycloneDX = serde_json::from_str(env!("CARGO_SBOM")).unwrap_or_else(|e| {
        log::error!("Failed to parse SBOM: {}", e);
        CycloneDX { components: vec![] }
    });

    let filter = use_state(String::new);
    let show_list = use_state(|| false);

    let mut license_count: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();

    // Count licenses with OR and AND handling
    for component in &sbom.components {
        if let Some(licenses) = &component.licenses {
            for license in licenses {
                let parsed_groups = parse_license_expression(&license.expression);
                for (main_license, requirements) in parsed_groups {
                    // Count main license
                    *license_count.entry(main_license).or_default() += 1;
                    // Count requirement licenses
                    for req in requirements {
                        *license_count.entry(req).or_default() += 1;
                    }
                }
            }
        }
    }

    let filtered_components = sbom
        .components
        .iter()
        .filter(|component| {
            component
                .name
                .to_lowercase()
                .contains(&filter.to_lowercase())
        })
        .collect::<Vec<_>>();

    let onfilter = {
        let filter = filter.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            filter.set(input.value());
        })
    };

    let toggle_list = {
        let show_list = show_list.clone();
        Callback::from(move |_| {
            show_list.set(!*show_list);
        })
    };

    html! {
        <div class="app-dependencies">
            <div class="app-dependencies__summary">
                <div class="app-dependencies__header">
                    <span class="app-dependencies__count">
                        {format!("📦 {} dependencies", sbom.components.len())}
                    </span>
                    <button class="app-dependencies__toggle" onclick={toggle_list}>
                        {if *show_list { "Hide List" } else { "Show List" }}
                    </button>
                </div>
                <div class="app-dependencies__licenses">
                    {for license_count.iter().map(|(license, count)| {
                        html! {
                            <span class="license-badge" title={format!("{} packages", count)}>
                                {format!("{} ({})", license, count)}
                            </span>
                        }
                    })}
                </div>
            </div>
            if *show_list {
                <div class="app-dependencies__list-container">
                    <input
                        type="text"
                        class="app-dependencies__search"
                        placeholder="Search dependencies..."
                        onchange={onfilter}
                    />
                    <div class="app-dependencies__list">
                        {for filtered_components.iter().map(|component| {
                            html! {
                                <div class="app-dependencies__item">
                                    <span class="app-dependencies__name">
                                        {&component.name} {" "} {&component.version}
                                    </span>
                                    <span class="app-dependencies__item-licenses">
                                        {if let Some(licenses) = &component.licenses {
                                            html! {
                                                for licenses.iter().map(|license| {
                                                    let parsed_groups = parse_license_expression(&license.expression);
                                                    html! {
                                                        for parsed_groups.iter().map(|(main, requirements)| {
                                                            html! {
                                                                <div class="license-group">
                                                                    <span class="license-badge license-badge--small">
                                                                        {main}
                                                                    </span>
                                                                    {if !requirements.is_empty() {
                                                                        html! {
                                                                            <span class="license-requirements">
                                                                                {for requirements.iter().map(|req| {
                                                                                    html! {
                                                                                        <span class="license-badge license-badge--small license-badge--requirement">
                                                                                            {req}
                                                                                        </span>
                                                                                    }
                                                                                })}
                                                                            </span>
                                                                        }
                                                                    } else {
                                                                        html! {}
                                                                    }}
                                                                </div>
                                                            }
                                                        })
                                                    }
                                                })
                                            }
                                        } else {
                                            html! { <span class="license-badge license-badge--small">{"Unknown"}</span> }
                                        }}
                                    </span>
                                </div>
                            }
                        })}
                    </div>
                </div>
            }
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(AppDependenciesComponent, (),);
}

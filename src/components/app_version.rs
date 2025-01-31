use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AppVersionProps {
    #[prop_or_default]
    pub show_details: bool,
    #[prop_or_else(|| env!("CARGO_PKG_VERSION").to_string())]
    pub version: String,
}

#[function_component(AppVersionComponent)]
pub fn app_version(props: &AppVersionProps) -> Html {
    let build_timestamp = {
        let timestamp = env!("VERGEN_BUILD_TIMESTAMP");
        if let Some(pos) = timestamp.rfind('.') {
            if let Some(_z_pos) = timestamp.rfind('Z') {
                format!("{}Z", &timestamp[..pos])
            } else {
                timestamp.to_string()
            }
        } else {
            timestamp.to_string()
        }
    };
    let rustc_version = env!("VERGEN_RUSTC_SEMVER");
    let target = env!("VERGEN_CARGO_TARGET_TRIPLE");
    let opt_level = env!("VERGEN_CARGO_OPT_LEVEL");

    html! {
        <div class="app-version">
            <div class="app-version__main">
                <div class="app-version__version">
                    <span class="app-version__label">{"📦 Version"}</span>
                    <span class="app-version__value">{&props.version}</span>
                </div>
                <div class="app-version__build">
                    <span class="app-version__label">{"🏗️ Built"}</span>
                    <span class="app-version__value">{build_timestamp}</span>
                </div>
            </div>
            if props.show_details {
                <div class="app-version__details">
                    <div class="app-version__item">
                        <span class="app-version__label">{"🦀 Rustc"}</span>
                        <span class="app-version__value">{rustc_version}</span>
                    </div>
                    <div class="app-version__item">
                        <span class="app-version__label">{"🎯 Target"}</span>
                        <span class="app-version__value">{target}</span>
                    </div>
                    <div class="app-version__item">
                        <span class="app-version__label">{"⚡ Opt"}</span>
                        <span class="app-version__value">{opt_level}</span>
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

    yew_preview::create_preview!(
        AppVersionComponent,
        AppVersionProps {
            show_details: true,
            version: "0.1.0".to_string()
        },
        (
            "minimal",
            AppVersionProps {
                show_details: false,
                version: "0.1.0".to_string()
            }
        )
    );
}

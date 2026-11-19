use super::catalog::CreditsView;
use crate::i18n::use_i18n;
use crate::model::CreditsFormat;
use crate::providers::credits_provider::{use_credit_asset, use_credit_server};
use konnektoren_asset_loader::SourceLayer;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Properties)]
pub struct LicenseCreditsProps {
    /// URL, embedded, filesystem or layered assets, shared with the rest of the app.
    pub source: Rc<dyn SourceLayer>,
    /// YAML path relative to the source root.
    pub manifest: AttrValue,
    /// Public URL matching the source root, used for direct license-file links.
    #[prop_or_default]
    pub base_url: AttrValue,
}

impl PartialEq for LicenseCreditsProps {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.source, &other.source)
            && self.manifest == other.manifest
            && self.base_url == other.base_url
    }
}

#[function_component(LicenseCredits)]
pub fn license_credits(props: &LicenseCreditsProps) -> Html {
    let i18n = use_i18n();
    let server = use_credit_server(props.source.clone());
    let (catalog, retry) =
        use_credit_asset::<CreditsFormat>(server.clone(), props.manifest.clone());
    html! {
        <div class="license-credits">
            {match catalog {
                None => html! { <p role="status">{i18n.t("Loading credits…")}</p> },
                Some(Err(_)) => html! { <div role="alert"><p>{i18n.t("Could not load credits.")}</p><button class="btn btn-outline btn-sm" onclick={retry}>{i18n.t("Retry")}</button></div> },
                Some(Ok(catalog)) => html! { <CreditsView {catalog} server={server.clone()} base_url={props.base_url.clone()} /> },
            }}
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use konnektoren_asset_loader::EmbeddedSource;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        LicenseCredits,
        LicenseCreditsProps {
            source: Rc::new(EmbeddedSource::new(&[
                (
                    "credits.yml",
                    include_bytes!("../../../assets/credits.example.yml")
                ),
                (
                    "licenses/example.txt",
                    include_bytes!("../../../assets/licenses/example.txt")
                ),
            ])),
            manifest: "credits.yml".into(),
            base_url: "/assets".into(),
        },
        (
            "Yew vendor credits",
            LicenseCreditsProps {
                source: Rc::new(EmbeddedSource::new(&[(
                    "vendor-credits.yml",
                    include_bytes!("../../../assets/vendor-credits.yml"),
                )])),
                manifest: "vendor-credits.yml".into(),
                base_url: "/assets".into(),
            }
        ),
    );
}

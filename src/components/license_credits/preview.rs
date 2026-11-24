use super::LicenseCredits;
use super::catalog::{CreditsView, CreditsViewProps};
use super::group::{CreditGroupProps, CreditGroupView};
use super::license_file::{LicenseFileProps, LicenseFileView};
use super::license_text::{LicenseText, LicenseTextProps};
use crate::model::{
    CreditAssetServer, CreditEntry, CreditGroup, CreditLink, CreditsCatalog, LicenseFile,
};
use konnektoren_asset_loader::{AssetServer, EmbeddedSource, SourceLayer};
use std::{rc::Rc, sync::Arc};
use yew::prelude::*;
use yew_preview::{create_component_group, create_component_item, prelude::*};

fn server() -> CreditAssetServer {
    let source: Rc<dyn SourceLayer> = Rc::new(EmbeddedSource::new(&[(
        "licenses/example.txt",
        include_bytes!("../../../assets/licenses/example.txt"),
    )]));
    CreditAssetServer(Rc::new(AssetServer::new(source)))
}

fn example_license() -> LicenseFile {
    LicenseFile {
        name: "Read the license".into(),
        file: "licenses/example.txt".into(),
    }
}

fn example_group() -> CreditGroup {
    CreditGroup {
        id: "example".into(),
        name: "Credits and licenses".into(),
        credits: vec![CreditEntry {
            id: "example-asset".into(),
            name: "Example asset".into(),
            description: vec!["Open the license below to load its text.".into()],
            links: vec![CreditLink {
                name: "Project".into(),
                url: "https://example.org".into(),
            }],
            licenses: vec![example_license()],
        }],
        notes: Vec::new(),
    }
}

/// Component previews use the same embedded fixture and asset loader as the full view.
pub(crate) fn group() -> ComponentGroup {
    create_component_group!(
        "Credits and licenses",
        LicenseCredits::preview(),
        create_component_item!(
            "Credits catalog",
            CreditsView,
            vec![(
                "Example",
                CreditsViewProps {
                    catalog: Arc::new(CreditsCatalog {
                        groups: vec![example_group()]
                    }),
                    server: server(),
                    base_url: "/assets".into(),
                }
            )]
        ),
        create_component_item!(
            "Credit group",
            CreditGroupView,
            vec![(
                "Example",
                CreditGroupProps {
                    group: example_group(),
                    expanded: false,
                    server: server(),
                    base_url: "/assets".into(),
                }
            )]
        ),
        create_component_item!(
            "License file",
            LicenseFileView,
            vec![(
                "Example",
                LicenseFileProps {
                    license: example_license(),
                    server: server(),
                    base_url: "/assets".into(),
                }
            )]
        ),
        create_component_item!(
            "License text",
            LicenseText,
            vec![
                (
                    "Loaded",
                    LicenseTextProps {
                        server: server(),
                        path: "licenses/example.txt".into()
                    }
                ),
                (
                    "Missing file",
                    LicenseTextProps {
                        server: server(),
                        path: "licenses/missing.txt".into()
                    }
                ),
            ]
        ),
    )
}

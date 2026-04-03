use crate::repository::{CERTIFICATE_STORAGE_KEY, CertificateRepositoryTrait};
use konnektoren_core::certificates::CertificateData;
use std::sync::Arc;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct CertificatesContext {
    pub certificates: UseStateHandle<Vec<CertificateData>>,
}

#[derive(Properties)]
pub struct CertificatesProviderProps {
    pub children: Children,
    pub certificates_repository: Arc<dyn CertificateRepositoryTrait>,
}

impl PartialEq for CertificatesProviderProps {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(
            &self.certificates_repository,
            &other.certificates_repository,
        )
    }
}

#[function_component(CertificatesProvider)]
pub fn certificates_provider(props: &CertificatesProviderProps) -> Html {
    // IMPORTANT: use_state must be called OUTSIDE cfg blocks to maintain hook ordering
    let certificates = use_state(Vec::new);

    // Load certificates (CSR only)
    #[cfg(feature = "csr")]
    {
        let certificates = certificates.clone();
        let certificates_repository = props.certificates_repository.clone();

        use_effect_with((), move |_| {
            use wasm_bindgen_futures::spawn_local;

            spawn_local(async move {
                if let Ok(loaded_certificates) = certificates_repository
                    .get_certificates(CERTIFICATE_STORAGE_KEY)
                    .await
                {
                    certificates.set(loaded_certificates.unwrap_or_default());
                }
            });
            || ()
        });
    }

    // Save certificates (CSR only)
    #[cfg(feature = "csr")]
    {
        let certificates_repository = props.certificates_repository.clone();
        let current_certificates = (*certificates).clone();

        use_effect_with(current_certificates.clone(), move |_| {
            use wasm_bindgen_futures::spawn_local;

            let certificates = current_certificates.clone();
            spawn_local(async move {
                let certificates = certificates.clone();
                if let Err(e) = certificates_repository
                    .save_certificates(CERTIFICATE_STORAGE_KEY, &certificates)
                    .await
                {
                    log::error!("Failed to save certificates: {:?}", e);
                }
            });
            || ()
        });
    }

    let context = CertificatesContext { certificates };

    html! {
        <ContextProvider<CertificatesContext> {context}>
            { for props.children.iter() }
        </ContextProvider<CertificatesContext>>
    }
}

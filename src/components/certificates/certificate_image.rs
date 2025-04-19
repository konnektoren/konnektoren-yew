use konnektoren_core::certificates::{CertificateData, create_certificate_data_url};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Debug, Default)]
pub struct CertificateImageProps {
    pub certificate_data: CertificateData,
    #[prop_or_default]
    pub hostname: Option<String>,
    #[prop_or_default]
    pub protocol: Option<String>,
}

#[function_component(CertificateImageComponent)]
pub fn certificate_image_component(props: &CertificateImageProps) -> Html {
    let share_url = format!(
        "{}://{}/?page=results&code={}",
        props.protocol.clone().unwrap_or_default(),
        props.hostname.clone().unwrap_or_default(),
        &props.certificate_data.to_base64()
    );

    let img_src = create_certificate_data_url(
        &props.certificate_data,
        &share_url,
        &props.hostname.clone().unwrap_or_default(),
    );

    html! {
        <div class="certificate__image-container">
            {
                match img_src {
                    Ok(src) => html! {
                        <img class="certificate__image" src={src} alt="Certificate of Achievement" />
                    },
                    Err(err) => html! {
                        <p class="certificate__image-error">{ "Error creating certificate image: " }{ err }</p>
                    }
                }
            }
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        CertificateImageComponent,
        CertificateImageProps {
            certificate_data: CertificateData {
                game_path_name: "Level 1".to_string(),
                total_challenges: 10,
                solved_challenges: 5,
                performance_percentage: 50,
                profile_name: "User".to_string(),
                date: Default::default(),
                signature: None,
            },
            hostname: Some("example.com".to_string()),
            protocol: Some("https".to_string()),
        },
    );
}

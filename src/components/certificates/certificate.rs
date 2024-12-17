use crate::prelude::CertificateImageComponent;
use gloo::timers::callback::Timeout;
use konnektoren_core::certificates::CertificateData;
use urlencoding::encode;
use yew::prelude::*;
use yew_hooks::{use_clipboard, UseClipboardHandle};

#[derive(Properties, PartialEq, Clone, Debug, Default)]
pub struct CertificateProps {
    pub certificate_data: CertificateData,
    #[prop_or_default]
    pub hostname: Option<String>,
    #[prop_or_default]
    pub protocol: Option<String>,
}

#[function_component(CertificateComponent)]
pub fn certificate(props: &CertificateProps) -> Html {
    let clipboard_handle: UseClipboardHandle = use_clipboard();
    let show_copied_message = use_state(|| false);

    let share_url = format!(
        "{}//{}/?page=results&code={}",
        props.protocol.clone().unwrap_or_default(),
        props.hostname.clone().unwrap_or_default(),
        encode(&props.certificate_data.to_base64())
    );

    let on_share_click = {
        let clipboard_handle = clipboard_handle.clone();
        let data = share_url.clone();
        let show_copied_message = show_copied_message.clone();
        Callback::from(move |_| {
            clipboard_handle.write_text(data.to_string());
            show_copied_message.set(true);
            let show_copied_message = show_copied_message.clone();
            Timeout::new(3000, move || {
                show_copied_message.set(false);
            })
            .forget();
        })
    };

    let certificate_data = props.certificate_data.clone();
    let hostname = props.hostname.clone();
    let protocol = props.protocol.clone();

    let verified = certificate_data.verify();

    html! {
        <div class="certificate">
            <h2 class="certificate__title">{ "Certificate of Achievement" }</h2>
            <div class="certificate__details">
                <p class="certificate__info"><strong>{ "Profile Name: " }</strong>{ &props.certificate_data.profile_name }</p>
                <p class="certificate__info"><strong>{ "Game Path: " }</strong>{ &props.certificate_data.game_path_name }</p>
                <p class="certificate__info"><strong>{ "Total Challenges: " }</strong>{ &props.certificate_data.total_challenges }</p>
                <p class="certificate__info"><strong>{ "Solved Challenges: " }</strong>{ &props.certificate_data.solved_challenges }</p>
                <p class="certificate__info"><strong>{ "Performance Percentage: " }</strong>{ format!("{}%", &props.certificate_data.performance_percentage) }</p>
                <p class="certificate__info"><strong>{ "Date: " }</strong>{ &props.certificate_data.date.to_string() }</p>
                { render_verification_status(verified) }
            </div>
            <div class="certificate__share">
                <input type="text" class="certificate__share-input" readonly=true value={share_url.clone()} />
                <button onclick={on_share_click} class="btn btn--primary">{ "Share This Achievement" }</button>
                if *show_copied_message {
                    <p class="certificate__share-message">{"Link copied to clipboard!"}</p>
                }
            </div>
            <div class="certificate__image">
                <CertificateImageComponent {certificate_data} {hostname} {protocol} />
            </div>
        </div>
    }
}

fn render_verification_status(verified: bool) -> Html {
    let (icon, text, class) = if verified {
        (
            "fas fa-check-circle",
            "Certificate Verified",
            "certificate__verification--verified",
        )
    } else {
        (
            "fas fa-exclamation-triangle",
            "Certificate Not Verified",
            "certificate__verification--not-verified",
        )
    };

    html! {
        <div class={classes!("certificate__verification", class)}>
            <i class={classes!(icon, "certificate__verification-icon")}></i>
            <span>{ text }</span>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    fn get_certificate_data() -> CertificateData {
        let mut certificate = CertificateData::new(
            "Level A1".to_string(),
            12,
            10,
            "Player".to_string(),
            Default::default(),
        );
        certificate.create_signature();
        certificate
    }

    yew_preview::create_preview!(
        CertificateComponent,
        CertificateProps::default(),
        (
            "issuer",
            CertificateProps {
                certificate_data: CertificateData::new(
                    "Level A1".to_string(),
                    12,
                    10,
                    "Player".to_string(),
                    Default::default(),
                ),
                hostname: Some("localhost".to_string()),
                protocol: Some("http:".to_string()),
            }
        ),
        (
            "signed",
            CertificateProps {
                certificate_data: get_certificate_data(),
                hostname: Some("localhost".to_string()),
                protocol: Some("http:".to_string()),
            }
        )
    );
}

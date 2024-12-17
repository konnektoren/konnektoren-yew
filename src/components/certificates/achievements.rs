use crate::prelude::{AchievementComponent, CertificateComponent};
use konnektoren_core::{certificates::CertificateData, prelude::AchievementDefinition};
use std::rc::Rc;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct AchievementsProps {
    pub achievements: Vec<AchievementDefinition>,
    pub certificates: Vec<CertificateData>,
    #[prop_or_default]
    pub hostname: Option<String>,
    #[prop_or_default]
    pub protocol: Option<String>,
}

#[function_component(AchievementsComponent)]
pub fn achievements_component(props: &AchievementsProps) -> Html {
    let selected_certificate = use_state(|| None);
    let sorted_certificates = sort_certificates(&props.certificates);
    let on_certificate_click = create_certificate_click_handler(selected_certificate.clone());

    html! {
        <div class="achievements">
            <h2 class="achievements__title">{ "Achievements and Certificates" }</h2>
            <div class="achievements__container">
                <div class="achievements__achievements-list">
                    <h3 class="achievements__subtitle">{ "Achievements" }</h3>
                    { render_achievements(&props.achievements) }
                </div>
                <div class="achievements__certificates-list">
                    <h3 class="achievements__subtitle">{ "Certificates" }</h3>
                    <ul class="achievements__list">
                        { render_certificates(&sorted_certificates, &selected_certificate, on_certificate_click, props) }
                    </ul>
                </div>
            </div>
        </div>
    }
}

fn render_achievements(achievements: &[AchievementDefinition]) -> Html {
    html! {
        <div class="achievements__achievement-grid">
            { for achievements.iter().map(|achievement| {
                html! {
                    <AchievementComponent achievement={achievement.clone()} />
                }
            })}
        </div>
    }
}

fn sort_certificates(certificates: &[CertificateData]) -> Vec<CertificateData> {
    let mut sorted = certificates.to_vec();
    sorted.sort_by(|a, b| b.date.cmp(&a.date));
    sorted
}

fn create_certificate_click_handler(
    selected_certificate: UseStateHandle<Option<CertificateData>>,
) -> Rc<dyn Fn(CertificateData)> {
    Rc::new(move |cert: CertificateData| {
        selected_certificate.set(Some(cert));
    })
}

fn render_certificates(
    certificates: &[CertificateData],
    selected_certificate: &UseStateHandle<Option<CertificateData>>,
    on_click: Rc<dyn Fn(CertificateData)>,
    props: &AchievementsProps,
) -> Html {
    certificates
        .iter()
        .map(|cert| {
            let is_selected = selected_certificate.as_ref() == Some(cert);
            render_certificate_item(cert, is_selected, on_click.clone(), props)
        })
        .collect()
}

fn render_certificate_item(
    cert: &CertificateData,
    is_selected: bool,
    on_click: Rc<dyn Fn(CertificateData)>,
    props: &AchievementsProps,
) -> Html {
    let cert_clone = cert.clone();
    let onclick = Callback::from(move |_| on_click(cert_clone.clone()));

    html! {
        <li class={classes!(
            "achievements__certificate-item",
            is_selected.then(|| "achievements__certificate-item--selected")
        )}>
            <div class="achievements__certificate-summary" {onclick}>
                { render_certificate_summary(cert) }
            </div>
            if is_selected {
                <div class="achievements__certificate-details">
                    <CertificateComponent
                        certificate_data={cert.clone()}
                        hostname={props.hostname.clone()}
                        protocol={props.protocol.clone()}
                    />
                </div>
            }
        </li>
    }
}

fn render_certificate_summary(cert: &CertificateData) -> Html {
    html! {
        <>
            <span class="achievements__date">{ cert.date.format("%Y-%m-%d").to_string() }</span>
            <span class="achievements__name">{ &cert.game_path_name }</span>
            <span class="achievements__performance">{ format!("{}%", cert.performance_percentage) }</span>
        </>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        AchievementsComponent,
        AchievementsProps {
            achievements: vec![AchievementDefinition {
                id: "".to_string(),
                name: "Achievement 1".to_string(),
                description: "Description 1".to_string(),
                icon: "https://example.com/icon1.png".to_string(),
                condition: "".to_string(),
            },],
            certificates: vec![
                CertificateData {
                    game_path_name: "Level 1".to_string(),
                    total_challenges: 10,
                    solved_challenges: 5,
                    performance_percentage: 50,
                    profile_name: "User".to_string(),
                    date: Default::default(),
                    signature: None,
                },
                CertificateData {
                    game_path_name: "Level 2".to_string(),
                    total_challenges: 10,
                    solved_challenges: 10,
                    performance_percentage: 100,
                    profile_name: "User".to_string(),
                    date: Default::default(),
                    signature: None,
                },
            ],
            hostname: Some("localhost".to_string()),
            protocol: Some("http".to_string()),
        },
    );
}

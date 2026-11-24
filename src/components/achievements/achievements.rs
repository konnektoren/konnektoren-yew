use crate::i18n::use_i18n;
use crate::prelude::{AchievementComponent, CertificateComponent};
use konnektoren_core::{certificates::CertificateData, prelude::AchievementDefinition};
use konnektoren_yew_core::{FilteredList, ListItem, ListLabels};
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
    let i18n = use_i18n();
    let selected_certificate = use_state(|| None);
    let search = use_state(String::new);
    let sorted_certificates = sort_certificates(&props.certificates);
    let on_certificate_click = create_certificate_click_handler(selected_certificate.clone());
    let query = search.to_lowercase();
    let items: Vec<ListItem> = sorted_certificates
        .iter()
        .enumerate()
        .filter(|(_, cert)| i18n.t(&cert.game_path_name).to_lowercase().contains(&query))
        .map(|(index, cert)| {
            ListItem::new(
                index.to_string(),
                render_certificate_item(
                    cert,
                    selected_certificate.as_ref() == Some(cert),
                    on_certificate_click.clone(),
                    props,
                ),
            )
        })
        .collect();
    let matching = items.len();
    let on_search = {
        let search = search.clone();
        Callback::from(move |value| search.set(value))
    };
    let on_clear = {
        let search = search.clone();
        Callback::from(move |()| search.set(String::new()))
    };
    let labels = ListLabels {
        search: i18n.t("Search").into(),
        search_placeholder: i18n.t("Name").into(),
        noun: i18n.t("Certificates").into(),
        list: i18n.t("Certificates").into(),
        empty: i18n.t("Nothing here yet.").into(),
        no_match: i18n.t("Nothing matches these filters.").into(),
        sort_by: i18n.t("Sort by").into(),
        show: i18n.t("Show").into(),
        clear: i18n.t("Clear filters").into(),
        show_more: i18n.t("Show more").into(),
    };

    html! {
        <div class="achievements">
            <h2 class="achievements__title">
                <i class="fa-solid fa-trophy achievements__title-icon"></i>
                { i18n.t("Achievements and Certificates") }
            </h2>
            <div class="achievements__container">
                <div class="achievements__achievements-list">
                    <h3 class="achievements__subtitle">
                        <i class="fa-solid fa-medal achievements__subtitle-icon"></i>
                        { i18n.t("Achievements") }
                    </h3>
                    { render_achievements(&props.achievements) }
                </div>
                <div class="achievements__certificates-list">
                    <h3 class="achievements__subtitle">
                        <i class="fa-solid fa-certificate achievements__subtitle-icon"></i>
                        { i18n.t("Certificates") }
                    </h3>
                    <FilteredList {labels} {items} {matching} total={sorted_certificates.len()}
                        search={(*search).clone()} active={!search.is_empty()} {on_search} {on_clear} />
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

fn render_certificate_item(
    cert: &CertificateData,
    is_selected: bool,
    on_click: Rc<dyn Fn(CertificateData)>,
    props: &AchievementsProps,
) -> Html {
    let cert_clone = cert.clone();
    let onclick = Callback::from(move |_| on_click(cert_clone.clone()));

    html! {
        <div class={classes!(
            "achievements__certificate-item",
            is_selected.then_some("achievements__certificate-item--selected")
        )}>
            <div class="achievements__certificate-summary" {onclick}>
                <CertificateSummary cert={cert.clone()} />
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
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct CertificateSummaryProps {
    pub cert: CertificateData,
}

#[function_component(CertificateSummary)]
pub fn certificate_summary(props: &CertificateSummaryProps) -> Html {
    let i18n = use_i18n();
    let cert = &props.cert;
    html! {
        <>
            <span class="achievements__date">{ cert.date.format("%Y-%m-%d").to_string() }</span>
            <span class="achievements__name">{ i18n.t(&cert.game_path_name) }</span>
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
                icon: "https://picsum.photos/64".to_string(),
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

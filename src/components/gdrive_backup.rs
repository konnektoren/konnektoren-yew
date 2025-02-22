use crate::repository::{Backup, BackupInfo, GDriveBackup};
use gloo::utils::window;
use konnektoren_core::session::Session;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, Default, PartialEq)]
pub struct GDriveBackupProps {
    pub access_token: Option<String>,
    pub client_id: String,
    pub redirect_uri: String,
    pub session: Session,

    pub on_select: Callback<Session>,
    #[prop_or_default]
    pub on_error: Option<Callback<String>>,
    #[prop_or_default]
    pub on_success: Option<Callback<Session>>,
}

#[function_component(GDriveBackupComponent)]
pub fn gdrive_backup(props: &GDriveBackupProps) -> Html {
    let backups = use_state(Vec::<BackupInfo>::new);
    let loading = use_state(|| false);
    let error = use_state(|| None::<String>);

    let gdrive_repo = props
        .access_token
        .as_ref()
        .map(|token| GDriveBackup::new(token.clone(), props.client_id.clone()));

    {
        let backups = backups.clone();
        let loading = loading.clone();
        let error = error.clone();
        let gdrive_repo = gdrive_repo.clone();

        use_effect_with(props.access_token.clone(), move |_| {
            if let Some(repo) = gdrive_repo {
                loading.set(true);
                error.set(None);

                spawn_local(async move {
                    match <GDriveBackup as Backup<Session>>::list_backups::<'_, '_>(&repo).await {
                        Ok(list) => {
                            backups.set(list);
                            loading.set(false);
                        }
                        Err(e) => {
                            error.set(Some(e.to_string()));
                            loading.set(false);
                        }
                    }
                });
            }
            || ()
        });
    }

    let handle_backup = {
        let session = props.session.clone();
        let gdrive_repo = gdrive_repo.clone();
        let on_success = props.on_success.clone();
        let on_error = props.on_error.clone();
        let backups = backups.clone();
        let loading = loading.clone();

        Callback::from(move |_| {
            if let Some(repo) = gdrive_repo.clone() {
                let session = session.clone();
                let on_success = on_success.clone();
                let on_error = on_error.clone();
                let backups = backups.clone();
                let loading = loading.clone();

                spawn_local(async move {
                    match repo.backup("session", &session).await {
                        Ok(_) => {
                            if let Some(callback) = on_success.clone() {
                                callback.emit(session.clone());
                            }
                            loading.set(true);
                            match <GDriveBackup as Backup<Session>>::list_backups::<'_, '_>(&repo)
                                .await
                            {
                                Ok(list) => {
                                    backups.set(list);
                                    loading.set(false);
                                }
                                Err(e) => {
                                    if let Some(callback) = on_error {
                                        callback.emit(e.to_string());
                                    }
                                    loading.set(false);
                                }
                            }
                        }
                        Err(e) => {
                            if let Some(callback) = on_error {
                                callback.emit(e.to_string());
                            }
                        }
                    }
                });
            }
        })
    };

    let handle_select = {
        let on_select = props.on_select.clone();
        let gdrive_repo = gdrive_repo.clone();
        let on_error = props.on_error.clone();

        Callback::from(move |id: String| {
            if let Some(repo) = gdrive_repo.clone() {
                let on_select = on_select.clone();
                let on_error = on_error.clone();

                spawn_local(async move {
                    match repo.restore(&id).await {
                        Ok(session) => {
                            on_select.emit(session);
                        }
                        Err(e) => {
                            if let Some(callback) = on_error {
                                callback.emit(e.to_string());
                            }
                        }
                    }
                });
            }
        })
    };

    let handle_login = {
        let client_id = props.client_id.clone();
        let redirect_uri = props.redirect_uri.clone();
        Callback::from(move |_| {
            let scope = urlencoding::encode("https://www.googleapis.com/auth/drive.file");
            let auth_url = format!(
                "https://accounts.google.com/o/oauth2/v2/auth\
                ?client_id={}\
                &redirect_uri={}\
                &response_type=token\
                &scope={}\
                &prompt=consent\
                &access_type=online\
                &include_granted_scopes=true",
                client_id,
                urlencoding::encode(&redirect_uri),
                scope
            );

            log::info!("Redirecting to OAuth URL: {}", auth_url);

            if let Err(e) = window().location().set_href(&auth_url) {
                log::error!("Failed to redirect to OAuth URL: {:?}", e);
            }
        })
    };

    html! {
        <div class="gdrive-backup">
            if props.access_token.is_none() {
                <div class="google-oauth">
                    <button
                        onclick={handle_login}
                        class="google-oauth__button"
                    >
                        <img
                            src="https://developers.google.com/identity/images/g-logo.png"
                            alt="Google logo"
                            class="google-oauth__icon"
                        />
                        {"Sign in with Google"}
                    </button>
                </div>
            } else {
                if let Some(err) = &*error {
                    <div class="gdrive-backup__error">
                        <i class="fas fa-exclamation-circle"></i>
                        {err}
                    </div>
                }

                if *loading {
                    <div class="gdrive-backup__loading">
                        <i class="fas fa-spinner fa-spin"></i>
                        {"Loading backups..."}
                    </div>
                } else {
                    <div class="gdrive-backup__content">
                        <button class="gdrive-backup__button" onclick={handle_backup}>
                            <i class="fas fa-cloud-upload-alt"></i>
                            {"Backup to Google Drive"}
                        </button>

                        <div class="gdrive-backup__list">
                            {
                                (*backups).clone().into_iter().map(|backup| {
                                    let id = backup.id.clone();
                                    let handle_select = handle_select.clone();
                                    html! {
                                        <div
                                            class="gdrive-backup__item"
                                            onclick={move |_| handle_select.emit(id.clone())}
                                            key={backup.id}
                                        >
                                            <i class="fas fa-file"></i>
                                            <div class="gdrive-backup__item-content">
                                                <span class="gdrive-backup__item-name">{&backup.name}</span>
                                                <span class="gdrive-backup__item-date">{&backup.created_at}</span>
                                            </div>
                                        </div>
                                    }
                                }).collect::<Html>()
                            }
                        </div>
                    </div>
                }
            }
        </div>
    }
}

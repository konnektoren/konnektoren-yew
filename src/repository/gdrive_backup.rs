#![cfg_attr(not(target_arch = "wasm32"), allow(unused))]

use super::{Backup, BackupError, BackupInfo, BACKUP_KEY};
use async_trait::async_trait;
use gloo::net::http::Request;
use serde::{Deserialize, Serialize};

use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GoogleDriveFile {
    id: String,
    name: String,
    #[serde(rename = "createdTime")]
    created_time: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GDriveBackup {
    pub access_token: String,
    pub client_id: String,
}

impl GDriveBackup {
    pub fn new(access_token: String, client_id: String) -> Self {
        Self {
            access_token,
            client_id,
        }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl<T: Serialize + for<'de> Deserialize<'de>> Backup<T> for GDriveBackup {
    async fn list_backups(&self) -> Result<Vec<BackupInfo>, BackupError> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Err(BackupError::Unknown(
                "List backups is not supported in non-web environments".to_string(),
            ))
        }

        #[cfg(target_arch = "wasm32")]
        {
            let files = fetch_google_drive_files(&self.access_token).await?;
            let backups = files.into_iter().map(|file| BackupInfo {
                id: file.id,
                name: file.name,
                created_at: file
                    .created_time
                    .unwrap_or_else(|| "".to_string())
                    .replace("T", " ")
                    .replace("Z", "")
                    .to_string(),
            });

            Ok(backups.collect())
        }
    }

    async fn backup(&self, id: &str, value: &T) -> Result<BackupInfo, BackupError> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Err(BackupError::Unknown(
                "Backup is not supported in non-web environments".to_string(),
            ))
        }
        #[cfg(target_arch = "wasm32")]
        {
            let serialized_value = serde_json::to_string(value)
                .map_err(|e| BackupError::Unknown(format!("Failed to serialize data: {}", e)))?;

            let backup_id = upload_session(&self.access_token, &serialized_value).await?;

            let backup_name = format!(
                "backup_{}_{}.json",
                id,
                chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S")
            );

            Ok(BackupInfo {
                id: backup_id,
                name: backup_name,
                created_at: chrono::Utc::now().to_rfc3339(),
            })
        }
    }

    async fn restore(&self, id: &str) -> Result<T, BackupError> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            Err(BackupError::Unknown(
                "Restore is not supported in non-web environments".to_string(),
            ))
        }

        #[cfg(target_arch = "wasm32")]
        {
            let serialized_value = fetch_session_from_drive(&self.access_token, id).await?;
            let value = serde_json::from_str::<T>(&serialized_value)
                .map_err(|e| BackupError::Unknown(format!("Failed to deserialize data: {}", e)))?;
            Ok(value)
        }
    }
}

#[cfg(target_arch = "wasm32")]
async fn fetch_google_drive_files(access_token: &str) -> Result<Vec<GoogleDriveFile>, BackupError> {
    let url = "https://www.googleapis.com/drive/v3/files?\
               spaces=drive&\
               fields=files(id,name,createdTime)&\
               orderBy=createdTime desc&\
               q=mimeType='application/json' and name contains 'konnektoren-backup'";

    let response = Request::get(url)
        .header("Authorization", &format!("Bearer {}", access_token))
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| BackupError::Unknown(e.to_string()))?;

    if !response.ok() {
        return Err(BackupError::Unknown(format!(
            "API returned status: {}",
            response.status()
        )));
    }

    let text = response
        .text()
        .await
        .map_err(|e| BackupError::Unknown(e.to_string()))?;

    let response: serde_json::Value =
        serde_json::from_str(&text).map_err(|e| BackupError::Unknown(e.to_string()))?;

    let files = response
        .get("files")
        .and_then(|files| files.as_array())
        .ok_or_else(|| BackupError::Unknown("Missing 'files' array in response".to_string()))?
        .iter()
        .filter_map(|file| {
            let id = file.get("id")?.as_str()?.to_string();
            let name = file.get("name")?.as_str()?.to_string();
            let created_time = file
                .get("createdTime")
                .and_then(|t| t.as_str())
                .map(String::from);
            Some(GoogleDriveFile {
                id,
                name,
                created_time,
            })
        })
        .collect();

    Ok(files)
}

#[cfg(target_arch = "wasm32")]
async fn fetch_session_from_drive(
    access_token: &str,
    file_id: &str,
) -> Result<String, BackupError> {
    let url = format!(
        "https://www.googleapis.com/drive/v3/files/{}?alt=media",
        file_id
    );

    let response = Request::get(&url)
        .header("Authorization", &format!("Bearer {}", access_token))
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| BackupError::Unknown(e.to_string()))?;

    if !response.ok() {
        return Err(BackupError::AccessError(format!(
            "API returned status: {}",
            response.status()
        )));
    }

    let text = response
        .text()
        .await
        .map_err(|e| BackupError::Unknown(e.to_string()))?;

    Ok(text)
}

async fn upload_session(access_token: &str, session: &str) -> Result<String, BackupError> {
    let now: DateTime<Utc> = Utc::now();
    let date_string = now.format("%Y-%m-%d_%H-%M-%S").to_string();
    let filename = format!("konnektoren-backup_{}.json", date_string);

    let metadata = serde_json::json!({
        "name": filename,
        "mimeType": "application/json"
    });

    let boundary = "foo_bar_baz";
    let body = format!(
        "--{}\r\n\
        Content-Type: application/json; charset=UTF-8\r\n\r\n\
        {}\r\n\
        --{}\r\n\
        Content-Type: application/json\r\n\r\n\
        {}\r\n\
        --{}--",
        boundary,
        metadata.to_string(),
        boundary,
        session,
        boundary
    );

    let upload_url = "https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart";
    let request = Request::post(upload_url)
        .header("Authorization", &format!("Bearer {}", access_token))
        .header("Content-Type", "multipart/related; boundary=foo_bar_baz")
        .body(body)
        .unwrap();

    let response = request
        .send()
        .await
        .map_err(|e| BackupError::Unknown(e.to_string()))?;

    if !response.ok() {
        return Err(BackupError::Unknown(format!(
            "API returned status: {}",
            response.status()
        )));
    }

    let text = response
        .text()
        .await
        .map_err(|e| BackupError::Unknown(e.to_string()))?;

    let response: serde_json::Value =
        serde_json::from_str(&text).map_err(|e| BackupError::Unknown(e.to_string()))?;

    let file_id = response
        .get("id")
        .and_then(|id| id.as_str())
        .map(String::from);

    file_id.ok_or_else(|| BackupError::Unknown("Missing 'id' field in response".to_string()))
}

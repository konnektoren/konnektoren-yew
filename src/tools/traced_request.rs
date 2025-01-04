use async_trait::async_trait;
use gloo::net::http::{RequestBuilder, Response};
use gloo::storage::{SessionStorage, Storage};
use log::debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceContext {
    pub trace_id: String,
    pub parent_id: String,
    pub span_id: String, // Add current span ID
    pub session_id: String,
    pub sampled: bool,
}

impl Default for TraceContext {
    fn default() -> Self {
        let session_id = match SessionStorage::get::<String>("session_trace_id") {
            Ok(id) => id,
            Err(_) => {
                let id = Uuid::new_v4().to_string().replace("-", "");
                let _ = SessionStorage::set("session_trace_id", &id);
                id
            }
        };

        Self {
            trace_id: session_id.clone(),
            parent_id: "0".repeat(16),
            span_id: Uuid::new_v4().to_string().replace("-", ""),
            session_id,
            sampled: true,
        }
    }
}

impl TraceContext {
    pub fn get_or_create() -> Self {
        match SessionStorage::get::<String>("traceparent") {
            Ok(trace_parent) => Self::from_traceparent(&trace_parent),
            Err(_) => {
                let context = Self::default();
                if let Err(e) = SessionStorage::set("traceparent", context.to_traceparent()) {
                    debug!("Failed to store traceparent: {}", e);
                }
                context
            }
        }
    }

    pub fn new_child(&self) -> Self {
        Self {
            trace_id: self.trace_id.clone(),
            parent_id: self.span_id.clone(),
            span_id: Uuid::new_v4().to_string().replace("-", ""),
            session_id: self.session_id.clone(),
            sampled: self.sampled,
        }
    }

    pub fn to_traceparent(&self) -> String {
        format!(
            "00-{}-{}-{}",
            self.trace_id,
            self.span_id,
            if self.sampled { "01" } else { "00" }
        )
    }

    pub fn from_traceparent(traceparent: &str) -> Self {
        let parts: Vec<&str> = traceparent.split('-').collect();
        if parts.len() >= 4 {
            let session_id = SessionStorage::get::<String>("session_trace_id")
                .unwrap_or_else(|_| parts[1].to_string());

            Self {
                trace_id: parts[1].to_string(),
                parent_id: parts[2].to_string(),
                span_id: Uuid::new_v4().to_string().replace("-", ""),
                session_id,
                sampled: parts[3] == "01",
            }
        } else {
            Self::default()
        }
    }
}

pub fn update_trace_from_response(response: &Response) {
    if let Some(traceparent) = response.headers().get("traceparent") {
        let context = TraceContext::from_traceparent(&traceparent);
        if let Err(e) = SessionStorage::set("traceparent", context.to_traceparent()) {
            debug!("Failed to store traceparent: {}", e);
        } else {
            debug!("Updated traceparent: {}", context.to_traceparent());
        }
    }
}

pub trait TracedRequest {
    fn with_trace(self) -> Self;
    fn with_user_id(self, user_id: &str) -> Self;
}

impl TracedRequest for RequestBuilder {
    fn with_trace(self) -> Self {
        let trace_context = TraceContext::get_or_create();
        let child_context = trace_context.new_child();
        let traceparent = child_context.to_traceparent();

        debug!("Sending with trace: {}", traceparent);
        self.header("traceparent", &traceparent)
    }

    fn with_user_id(self, user_id: &str) -> Self {
        self.header("X-User-ID", user_id)
    }
}

#[async_trait(?Send)]
pub trait TracedResponse: TracedRequest {
    async fn send_traced(self) -> Result<Response, gloo::net::Error>;
    async fn fetch_traced<T>(self) -> Result<T, String>
    where
        T: for<'de> Deserialize<'de>;
}

#[async_trait(?Send)]
impl TracedResponse for RequestBuilder {
    async fn send_traced(self) -> Result<Response, gloo::net::Error> {
        let response = self.with_trace().send().await?;
        update_trace_from_response(&response);
        Ok(response)
    }

    async fn fetch_traced<T>(self) -> Result<T, String>
    where
        T: for<'de> Deserialize<'de>,
    {
        let response = self.send_traced().await.map_err(|e| e.to_string())?;

        if !response.ok() {
            return Err(format!("Request failed: {}", response.status()));
        }

        response.json().await.map_err(|e| e.to_string())
    }
}

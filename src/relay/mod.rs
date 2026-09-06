use anyhow::Result;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::event::event::Payload;
use crate::event::filters::EventFilter;

/// Minimal relay client for the MIP-03 HTTP binding.
///
/// Relays are not trusted: clients MUST verify event signatures locally
/// (MIP-03, MIP-05) before treating events as valid.
pub struct RelayClient {
    base_url: String,
    http: reqwest::Client,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubmitRequest {
    pub events: Vec<Payload>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubmitResponse {
    pub results: Vec<SubmitResult>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubmitResult {
    pub id: Option<String>,
    pub accepted: bool,
    pub status: SubmitStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SubmitStatus {
    Stored,
    Duplicate,
    Rejected,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FetchRequest {
    pub ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FetchResponse {
    pub events: Vec<Payload>,
    pub missing: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScanRequest {
    pub filters: Vec<EventFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScanResponse {
    pub events: Vec<Payload>,
    pub cursor: Option<String>,
    pub has_more: bool,
}

impl RelayClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        let base_url = base_url.into();
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            http: reqwest::Client::new(),
        }
    }

    /// `POST /submit` — submits one or more signed events.
    pub async fn submit(&self, events: &[Payload]) -> Result<SubmitResponse> {
        let response = self
            .http
            .post(self.url("/submit"))
            .json(&SubmitRequest {
                events: events.to_vec(),
            })
            .send()
            .await?;

        let response = check_status(response).await?;
        Ok(response.json().await?)
    }

    /// `POST /fetch` — fetches events by exact id.
    pub async fn fetch(&self, ids: &[String]) -> Result<FetchResponse> {
        let response = self
            .http
            .post(self.url("/fetch"))
            .json(&FetchRequest { ids: ids.to_vec() })
            .send()
            .await?;

        let response = check_status(response).await?;
        Ok(response.json().await?)
    }

    /// `POST /scan` — scans events matching one or more filters.
    pub async fn scan(
        &self,
        filters: &[EventFilter],
        limit: Option<u32>,
        cursor: Option<String>,
        order: Option<String>,
    ) -> Result<ScanResponse> {
        let response = self
            .http
            .post(self.url("/scan"))
            .json(&ScanRequest {
                filters: filters.to_vec(),
                limit,
                cursor,
                order,
            })
            .send()
            .await?;

        let response = check_status(response).await?;
        Ok(response.json().await?)
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }
}

/// Maps MIP-03 HTTP status codes to structured errors.
///
/// Returns the response on success so the caller can deserialize the body.
async fn check_status(response: reqwest::Response) -> Result<reqwest::Response> {
    let status = response.status();

    if status.is_success() {
        return Ok(response);
    }

    let body = response.text().await.unwrap_or_default();

    match status {
        StatusCode::BAD_REQUEST => anyhow::bail!("relay 400 bad request: {body}"),
        StatusCode::PAYLOAD_TOO_LARGE => anyhow::bail!("relay 413 payload too large: {body}"),
        StatusCode::TOO_MANY_REQUESTS => anyhow::bail!("relay 429 rate limited: {body}"),
        _ => anyhow::bail!("relay returned {}: {body}", status),
    }
}

use crate::api::csv_parser;
use crate::api::rate_limiter::{self, Limiter};
use crate::error::AppError;
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, warn};

const V3_ANALYTICS_BASE: &str = "https://api.semrush.com/";
const V1_BACKLINKS_BASE: &str = "https://api.semrush.com/analytics/v1/";
const V3_TRENDS_BASE: &str = "https://api.semrush.com/analytics/ta/api/v3/";

const MAX_RETRIES: u32 = 3;

pub struct SemrushClient {
    http: Client,
    api_key: String,
    limiter: Arc<Limiter>,
}

impl SemrushClient {
    pub fn new(api_key: String, requests_per_second: u32) -> Self {
        let http = Client::builder()
            .gzip(true)
            .timeout(std::time::Duration::from_secs(30))
            .pool_max_idle_per_host(10)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            http,
            api_key,
            limiter: rate_limiter::create(requests_per_second),
        }
    }

    /// Execute a v3 Analytics API request and return parsed JSON rows.
    pub async fn v3_analytics(
        &self,
        report_type: &str,
        params: &HashMap<String, String>,
    ) -> Result<Vec<serde_json::Value>, AppError> {
        let body = self
            .request_csv(V3_ANALYTICS_BASE, report_type, params)
            .await?;
        csv_parser::parse_csv_response(&body)
    }

    /// Execute a v1 Backlinks API request and return parsed JSON rows.
    pub async fn v1_backlinks(
        &self,
        report_type: &str,
        params: &HashMap<String, String>,
    ) -> Result<Vec<serde_json::Value>, AppError> {
        let body = self
            .request_csv(V1_BACKLINKS_BASE, report_type, params)
            .await?;
        csv_parser::parse_csv_response(&body)
    }

    /// Execute a v3 Trends API request and return parsed JSON rows.
    /// Trends API returns CSV like other v3 endpoints.
    pub async fn v3_trends(
        &self,
        endpoint: &str,
        params: &HashMap<String, String>,
    ) -> Result<Vec<serde_json::Value>, AppError> {
        let url = format!("{V3_TRENDS_BASE}{endpoint}");
        let mut query: Vec<(String, String)> = vec![("key".to_string(), self.api_key.clone())];
        for (k, v) in params {
            query.push((k.clone(), v.clone()));
        }

        let body = self.request_with_retry(&url, &query).await?;
        csv_parser::parse_csv_response(&body)
    }

    async fn request_csv(
        &self,
        base_url: &str,
        report_type: &str,
        params: &HashMap<String, String>,
    ) -> Result<String, AppError> {
        let mut query: Vec<(String, String)> = vec![
            ("type".to_string(), report_type.to_string()),
            ("key".to_string(), self.api_key.clone()),
        ];
        for (k, v) in params {
            query.push((k.clone(), v.clone()));
        }

        self.request_with_retry(base_url, &query).await
    }

    async fn request_with_retry(
        &self,
        url: &str,
        query: &[(String, String)],
    ) -> Result<String, AppError> {
        let mut last_error = None;

        for attempt in 0..MAX_RETRIES {
            // Wait for rate limiter
            self.limiter.until_ready().await;

            debug!(url = url, attempt = attempt, "Sending API request");

            let response = self.http.get(url).query(query).send().await?;

            let status = response.status();

            if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                let retry_after = response
                    .headers()
                    .get("retry-after")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.parse::<u64>().ok())
                    .unwrap_or(2u64.pow(attempt + 1));

                warn!(
                    retry_after_secs = retry_after,
                    attempt = attempt,
                    "Rate limited by Semrush API"
                );

                if attempt < MAX_RETRIES - 1 {
                    tokio::time::sleep(std::time::Duration::from_secs(retry_after)).await;
                    continue;
                }

                return Err(AppError::RateLimited {
                    retry_after_ms: retry_after * 1000,
                    api_status_code: 429,
                });
            }

            if status.is_server_error() {
                let body = response.text().await.unwrap_or_default();
                warn!(
                    status = status.as_u16(),
                    attempt = attempt,
                    "Server error from Semrush API"
                );

                if attempt < MAX_RETRIES - 1 {
                    let delay = 2u64.pow(attempt + 1);
                    tokio::time::sleep(std::time::Duration::from_secs(delay)).await;
                    last_error = Some(AppError::ApiError {
                        status_code: status.as_u16(),
                        message: body,
                    });
                    continue;
                }

                return Err(last_error.unwrap_or(AppError::ApiError {
                    status_code: status.as_u16(),
                    message: body,
                }));
            }

            if status == reqwest::StatusCode::FORBIDDEN
                || status == reqwest::StatusCode::UNAUTHORIZED
            {
                return Err(AppError::AuthFailed {
                    message: "Invalid API key or insufficient permissions".to_string(),
                });
            }

            if !status.is_success() {
                let body = response.text().await.unwrap_or_default();
                return Err(AppError::ApiError {
                    status_code: status.as_u16(),
                    message: body,
                });
            }

            return response.text().await.map_err(AppError::from);
        }

        Err(last_error.unwrap_or(AppError::NetworkError {
            message: "Max retries exceeded".to_string(),
        }))
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    // ── v4 JSON methods (OAuth2 bearer token auth) ─────────────

    pub async fn v4_json_get(
        &self,
        url: &str,
        oauth_token: &str,
    ) -> Result<serde_json::Value, AppError> {
        self.limiter.until_ready().await;
        let response = self.http.get(url).bearer_auth(oauth_token).send().await?;
        self.handle_json_response(response).await
    }

    pub async fn v4_json_post(
        &self,
        url: &str,
        oauth_token: &str,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value, AppError> {
        self.limiter.until_ready().await;
        let response = self
            .http
            .post(url)
            .bearer_auth(oauth_token)
            .json(body)
            .send()
            .await?;
        self.handle_json_response(response).await
    }

    pub async fn v4_json_patch(
        &self,
        url: &str,
        oauth_token: &str,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value, AppError> {
        self.limiter.until_ready().await;
        let response = self
            .http
            .patch(url)
            .bearer_auth(oauth_token)
            .json(body)
            .send()
            .await?;
        self.handle_json_response(response).await
    }

    pub async fn v4_json_delete(
        &self,
        url: &str,
        oauth_token: &str,
    ) -> Result<serde_json::Value, AppError> {
        self.limiter.until_ready().await;
        let response = self
            .http
            .delete(url)
            .bearer_auth(oauth_token)
            .send()
            .await?;

        let status = response.status();
        if status == reqwest::StatusCode::NO_CONTENT || status.is_success() {
            return Ok(serde_json::json!({"status": "deleted"}));
        }
        self.handle_json_response(response).await
    }

    async fn handle_json_response(
        &self,
        response: reqwest::Response,
    ) -> Result<serde_json::Value, AppError> {
        let status = response.status();

        if status == reqwest::StatusCode::UNAUTHORIZED || status == reqwest::StatusCode::FORBIDDEN {
            return Err(AppError::AuthFailed {
                message: "OAuth2 token invalid or expired. Run `semrush account auth setup-oauth`."
                    .to_string(),
            });
        }

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::ApiError {
                status_code: status.as_u16(),
                message: body,
            });
        }

        let body = response.text().await.map_err(AppError::from)?;
        if body.is_empty() {
            return Ok(serde_json::Value::Null);
        }
        serde_json::from_str(&body).map_err(|e| AppError::ParseError {
            message: format!("Failed to parse JSON response: {e}"),
        })
    }
}

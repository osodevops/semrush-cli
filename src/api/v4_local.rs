use crate::api::client::SemrushClient;
use crate::error::AppError;

const LISTING_BASE: &str = "https://api.semrush.com/management/v1/listings";
const MAP_RANK_BASE: &str = "https://api.semrush.com/management/v1/map-rank";

// ── Listing Management ─────────────────────────────────────────

pub async fn listing_list(
    client: &SemrushClient,
    oauth_token: &str,
) -> Result<Vec<serde_json::Value>, AppError> {
    let response = client.v4_json_get(LISTING_BASE, oauth_token).await?;
    match response {
        serde_json::Value::Array(arr) => Ok(arr),
        serde_json::Value::Object(map) => {
            if let Some(serde_json::Value::Array(arr)) = map.get("data") {
                Ok(arr.clone())
            } else {
                Ok(vec![serde_json::Value::Object(map)])
            }
        }
        other => Ok(vec![other]),
    }
}

pub async fn listing_get(
    client: &SemrushClient,
    oauth_token: &str,
    location_id: &str,
) -> Result<Vec<serde_json::Value>, AppError> {
    let url = format!("{LISTING_BASE}/{location_id}");
    let response = client.v4_json_get(&url, oauth_token).await?;
    Ok(vec![response])
}

pub async fn listing_create(
    client: &SemrushClient,
    oauth_token: &str,
    body: &serde_json::Value,
) -> Result<Vec<serde_json::Value>, AppError> {
    let response = client.v4_json_post(LISTING_BASE, oauth_token, body).await?;
    Ok(vec![response])
}

pub async fn listing_update(
    client: &SemrushClient,
    oauth_token: &str,
    location_id: &str,
    body: &serde_json::Value,
) -> Result<Vec<serde_json::Value>, AppError> {
    let url = format!("{LISTING_BASE}/{location_id}");
    let response = client.v4_json_patch(&url, oauth_token, body).await?;
    Ok(vec![response])
}

pub async fn listing_delete(
    client: &SemrushClient,
    oauth_token: &str,
    location_id: &str,
) -> Result<Vec<serde_json::Value>, AppError> {
    let url = format!("{LISTING_BASE}/{location_id}");
    client.v4_json_delete(&url, oauth_token).await?;
    Ok(vec![serde_json::json!({"deleted": location_id})])
}

// ── Map Rank Tracker ───────────────────────────────────────────

pub async fn map_rank_campaigns(
    client: &SemrushClient,
    oauth_token: &str,
) -> Result<Vec<serde_json::Value>, AppError> {
    let url = format!("{MAP_RANK_BASE}/campaigns");
    let response = client.v4_json_get(&url, oauth_token).await?;
    match response {
        serde_json::Value::Array(arr) => Ok(arr),
        other => Ok(vec![other]),
    }
}

pub async fn map_rank_keywords(
    client: &SemrushClient,
    oauth_token: &str,
    campaign_id: &str,
) -> Result<Vec<serde_json::Value>, AppError> {
    let url = format!("{MAP_RANK_BASE}/campaigns/{campaign_id}/keywords");
    let response = client.v4_json_get(&url, oauth_token).await?;
    match response {
        serde_json::Value::Array(arr) => Ok(arr),
        other => Ok(vec![other]),
    }
}

pub async fn map_rank_heatmap(
    client: &SemrushClient,
    oauth_token: &str,
    campaign_id: &str,
) -> Result<Vec<serde_json::Value>, AppError> {
    let url = format!("{MAP_RANK_BASE}/campaigns/{campaign_id}/heatmap");
    let response = client.v4_json_get(&url, oauth_token).await?;
    match response {
        serde_json::Value::Array(arr) => Ok(arr),
        other => Ok(vec![other]),
    }
}

pub async fn map_rank_competitors(
    client: &SemrushClient,
    oauth_token: &str,
    campaign_id: &str,
) -> Result<Vec<serde_json::Value>, AppError> {
    let url = format!("{MAP_RANK_BASE}/campaigns/{campaign_id}/competitors");
    let response = client.v4_json_get(&url, oauth_token).await?;
    match response {
        serde_json::Value::Array(arr) => Ok(arr),
        other => Ok(vec![other]),
    }
}

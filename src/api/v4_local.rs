use crate::api::client::SemrushClient;
use crate::error::AppError;

const LISTING_BASE: &str = "https://api.semrush.com/apis/v4/local/v1/locations";
const MAP_RANK_BASE: &str = "https://api.semrush.com/apis/v4/map-rank-tracker/v0";

// ── Listing Management ─────────────────────────────────────────

pub async fn listing_list(client: &SemrushClient) -> Result<Vec<serde_json::Value>, AppError> {
    let response = client.json_get_with_apikey_header(LISTING_BASE).await?;
    Ok(json_rows(response))
}

pub async fn listing_get(
    client: &SemrushClient,
    location_id: &str,
) -> Result<Vec<serde_json::Value>, AppError> {
    let url = format!("{LISTING_BASE}/{location_id}");
    let response = client.json_get_with_apikey_header(&url).await?;
    Ok(vec![response])
}

pub async fn listing_create(
    client: &SemrushClient,
    body: &serde_json::Value,
) -> Result<Vec<serde_json::Value>, AppError> {
    let response = client
        .json_post_with_apikey_header(LISTING_BASE, body)
        .await?;
    Ok(vec![response])
}

pub async fn listing_update(
    client: &SemrushClient,
    location_id: &str,
    body: &serde_json::Value,
) -> Result<Vec<serde_json::Value>, AppError> {
    let url = format!("{LISTING_BASE}/{location_id}");
    let response = client.json_put_with_apikey_header(&url, body).await?;
    Ok(vec![response])
}

pub async fn listing_delete(
    client: &SemrushClient,
    location_id: &str,
) -> Result<Vec<serde_json::Value>, AppError> {
    let url = format!("{LISTING_BASE}/{location_id}");
    client.json_delete_with_apikey_header(&url).await?;
    Ok(vec![serde_json::json!({"deleted": location_id})])
}

// ── Map Rank Tracker ───────────────────────────────────────────

pub async fn map_rank_campaigns(
    client: &SemrushClient,
    oauth_token: &str,
) -> Result<Vec<serde_json::Value>, AppError> {
    let url = format!("{MAP_RANK_BASE}/campaigns");
    let response = client.v4_json_get(&url, oauth_token).await?;
    Ok(json_rows(response))
}

pub async fn map_rank_keywords(
    client: &SemrushClient,
    oauth_token: &str,
    campaign_id: &str,
    report_date: Option<&str>,
) -> Result<Vec<serde_json::Value>, AppError> {
    let url = format!("{MAP_RANK_BASE}/campaigns/{campaign_id}/keywords");
    let url = with_query(&url, &[("reportDate", report_date)])?;
    let response = client.v4_json_get(&url, oauth_token).await?;
    Ok(json_rows(response))
}

pub async fn map_rank_heatmap(
    client: &SemrushClient,
    oauth_token: &str,
    campaign_id: &str,
    keyword_id: &str,
    cid: Option<&str>,
    place_ids: &[String],
    report_date: Option<&str>,
) -> Result<Vec<serde_json::Value>, AppError> {
    if cid.is_none() && place_ids.is_empty() {
        return Err(AppError::InvalidParams {
            message: "Map Rank heatmap requires either --cid or --place-ids.".to_string(),
        });
    }

    let url = format!("{MAP_RANK_BASE}/campaigns/{campaign_id}/heatmap");
    let mut params = vec![
        ("keywordId", Some(keyword_id)),
        ("cid", cid),
        ("reportDate", report_date),
    ];
    let place_ids_joined = place_ids.join(",");
    if !place_ids_joined.is_empty() {
        params.push(("placeIds", Some(place_ids_joined.as_str())));
    }
    let url = with_query(&url, &params)?;
    let response = client.v4_json_get(&url, oauth_token).await?;
    Ok(json_rows(response))
}

pub async fn map_rank_competitors(
    client: &SemrushClient,
    oauth_token: &str,
    campaign_id: &str,
    keyword_id: &str,
    report_date: &str,
) -> Result<Vec<serde_json::Value>, AppError> {
    let url = format!("{MAP_RANK_BASE}/campaigns/{campaign_id}/top-competitors");
    let url = with_query(
        &url,
        &[
            ("keywordId", Some(keyword_id)),
            ("reportDate", Some(report_date)),
        ],
    )?;
    let response = client.v4_json_get(&url, oauth_token).await?;
    Ok(json_rows(response))
}

fn json_rows(response: serde_json::Value) -> Vec<serde_json::Value> {
    match response {
        serde_json::Value::Array(arr) => arr,
        serde_json::Value::Object(map) => {
            if let Some(serde_json::Value::Array(arr)) = map.get("data") {
                arr.clone()
            } else {
                vec![serde_json::Value::Object(map)]
            }
        }
        other => vec![other],
    }
}

fn with_query(url: &str, params: &[(&str, Option<&str>)]) -> Result<String, AppError> {
    let mut parsed = reqwest::Url::parse(url).map_err(|e| AppError::InvalidParams {
        message: format!("Invalid API URL: {e}"),
    })?;
    {
        let mut pairs = parsed.query_pairs_mut();
        for (key, value) in params {
            if let Some(value) = value {
                pairs.append_pair(key, value);
            }
        }
    }
    Ok(parsed.to_string())
}

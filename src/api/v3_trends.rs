use crate::api::client::SemrushClient;
use crate::error::AppError;
use std::collections::HashMap;

/// Add common Trends params (country, device_type, display_date) if present.
fn add_common(
    params: &mut HashMap<String, String>,
    country: Option<&str>,
    device: Option<&str>,
    date: Option<&str>,
) {
    if let Some(c) = country {
        params.insert("country".to_string(), c.to_string());
    }
    if let Some(d) = device {
        params.insert("device_type".to_string(), d.to_string());
    }
    if let Some(dt) = date {
        params.insert("display_date".to_string(), dt.to_string());
    }
}

pub async fn summary(
    client: &SemrushClient,
    targets: &[String],
    country: Option<&str>,
    device: Option<&str>,
    date: Option<&str>,
    limit: u32,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = HashMap::new();
    params.insert("target".to_string(), targets.join(","));
    params.insert("display_limit".to_string(), limit.to_string());
    params.insert(
        "export_columns".to_string(),
        "target,visits,users,pages_per_visit,bounce_rate,avg_visit_duration".to_string(),
    );
    add_common(&mut params, country, device, date);
    client.v3_trends("summary", &params).await
}

pub async fn daily(
    client: &SemrushClient,
    target: &str,
    date_from: Option<&str>,
    date_to: Option<&str>,
    forecast: bool,
    country: Option<&str>,
    device: Option<&str>,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = HashMap::new();
    params.insert("target".to_string(), target.to_string());
    params.insert(
        "export_columns".to_string(),
        "date,visits,users,pages_per_visit,bounce_rate,avg_visit_duration".to_string(),
    );
    if let Some(f) = date_from {
        params.insert("date_from".to_string(), f.to_string());
    }
    if let Some(t) = date_to {
        params.insert("date_to".to_string(), t.to_string());
    }
    if forecast {
        params.insert("include_forecasted_items".to_string(), "true".to_string());
    }
    add_common(&mut params, country, device, None);
    client.v3_trends("summary_by_day", &params).await
}

pub async fn weekly(
    client: &SemrushClient,
    target: &str,
    date_from: Option<&str>,
    date_to: Option<&str>,
    forecast: bool,
    country: Option<&str>,
    device: Option<&str>,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = HashMap::new();
    params.insert("target".to_string(), target.to_string());
    params.insert(
        "export_columns".to_string(),
        "date,visits,users,pages_per_visit,bounce_rate,avg_visit_duration".to_string(),
    );
    if let Some(f) = date_from {
        params.insert("date_from".to_string(), f.to_string());
    }
    if let Some(t) = date_to {
        params.insert("date_to".to_string(), t.to_string());
    }
    if forecast {
        params.insert("include_forecasted_items".to_string(), "true".to_string());
    }
    add_common(&mut params, country, device, None);
    client.v3_trends("summary_by_week", &params).await
}

pub async fn sources(
    client: &SemrushClient,
    target: &str,
    channel: Option<&str>,
    traffic_type: Option<&str>,
    country: Option<&str>,
    device: Option<&str>,
    date: Option<&str>,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = HashMap::new();
    params.insert("target".to_string(), target.to_string());
    params.insert(
        "export_columns".to_string(),
        "channel,traffic,traffic_share".to_string(),
    );
    if let Some(ch) = channel {
        params.insert("traffic_channel".to_string(), ch.to_string());
    }
    if let Some(tt) = traffic_type {
        params.insert("traffic_type".to_string(), tt.to_string());
    }
    add_common(&mut params, country, device, date);
    client.v3_trends("sources", &params).await
}

pub async fn destinations(
    client: &SemrushClient,
    target: &str,
    country: Option<&str>,
    device: Option<&str>,
    date: Option<&str>,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = HashMap::new();
    params.insert("target".to_string(), target.to_string());
    add_common(&mut params, country, device, date);
    client.v3_trends("destinations", &params).await
}

pub async fn geo(
    client: &SemrushClient,
    target: &str,
    geo_type: Option<&str>,
    country: Option<&str>,
    device: Option<&str>,
    date: Option<&str>,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = HashMap::new();
    params.insert("target".to_string(), target.to_string());
    if let Some(gt) = geo_type {
        params.insert("geo_type".to_string(), gt.to_string());
    }
    add_common(&mut params, country, device, date);
    client.v3_trends("geo", &params).await
}

pub async fn subdomains(
    client: &SemrushClient,
    target: &str,
    country: Option<&str>,
    device: Option<&str>,
    date: Option<&str>,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = HashMap::new();
    params.insert("target".to_string(), target.to_string());
    add_common(&mut params, country, device, date);
    client.v3_trends("subdomains", &params).await
}

pub async fn top_pages(
    client: &SemrushClient,
    target: &str,
    country: Option<&str>,
    device: Option<&str>,
    date: Option<&str>,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = HashMap::new();
    params.insert("target".to_string(), target.to_string());
    add_common(&mut params, country, device, date);
    client.v3_trends("toppages", &params).await
}

pub async fn rank(
    client: &SemrushClient,
    country: Option<&str>,
    device: Option<&str>,
    date: Option<&str>,
    limit: u32,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = HashMap::new();
    params.insert("display_limit".to_string(), limit.to_string());
    add_common(&mut params, country, device, date);
    client.v3_trends("rank", &params).await
}

pub async fn categories(
    client: &SemrushClient,
    category: &str,
    country: Option<&str>,
    device: Option<&str>,
    date: Option<&str>,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = HashMap::new();
    params.insert("category".to_string(), category.to_string());
    add_common(&mut params, country, device, date);
    client.v3_trends("categories", &params).await
}

pub async fn conversion(
    client: &SemrushClient,
    target: &str,
    country: Option<&str>,
    device: Option<&str>,
    date: Option<&str>,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = HashMap::new();
    params.insert("target".to_string(), target.to_string());
    add_common(&mut params, country, device, date);
    client.v3_trends("purchase_conversion", &params).await
}

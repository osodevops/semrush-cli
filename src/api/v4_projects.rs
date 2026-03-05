use crate::api::client::SemrushClient;
use crate::error::AppError;

const V4_PROJECTS_BASE: &str = "https://api.semrush.com/management/v1/projects";

pub async fn list(
    client: &SemrushClient,
    oauth_token: &str,
) -> Result<Vec<serde_json::Value>, AppError> {
    let response = client.v4_json_get(V4_PROJECTS_BASE, oauth_token).await?;

    // Response is either an array or an object with a "data" field
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

pub async fn get(
    client: &SemrushClient,
    oauth_token: &str,
    project_id: &str,
) -> Result<Vec<serde_json::Value>, AppError> {
    let url = format!("{V4_PROJECTS_BASE}/{project_id}");
    let response = client.v4_json_get(&url, oauth_token).await?;
    Ok(vec![response])
}

pub async fn create(
    client: &SemrushClient,
    oauth_token: &str,
    name: &str,
    domain: &str,
) -> Result<Vec<serde_json::Value>, AppError> {
    let body = serde_json::json!({
        "project_name": name,
        "url": domain,
    });
    let url = V4_PROJECTS_BASE;
    let response = client.v4_json_post(url, oauth_token, &body).await?;
    Ok(vec![response])
}

pub async fn update(
    client: &SemrushClient,
    oauth_token: &str,
    project_id: &str,
    name: Option<&str>,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut body = serde_json::Map::new();
    if let Some(n) = name {
        body.insert(
            "project_name".to_string(),
            serde_json::Value::String(n.to_string()),
        );
    }
    let url = format!("{V4_PROJECTS_BASE}/{project_id}");
    let response = client
        .v4_json_patch(&url, oauth_token, &serde_json::Value::Object(body))
        .await?;
    Ok(vec![response])
}

pub async fn delete(
    client: &SemrushClient,
    oauth_token: &str,
    project_id: &str,
) -> Result<Vec<serde_json::Value>, AppError> {
    let url = format!("{V4_PROJECTS_BASE}/{project_id}");
    client.v4_json_delete(&url, oauth_token).await?;
    Ok(vec![serde_json::json!({"deleted": project_id})])
}

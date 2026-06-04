use crate::api::client::SemrushClient;
use crate::error::AppError;

const V4_PROJECTS_BASE: &str = "https://api.semrush.com/management/v1/projects";

pub async fn list(client: &SemrushClient) -> Result<Vec<serde_json::Value>, AppError> {
    let response = client.json_get_with_key(V4_PROJECTS_BASE).await?;

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
    project_id: &str,
) -> Result<Vec<serde_json::Value>, AppError> {
    let url = format!("{V4_PROJECTS_BASE}/{project_id}");
    let response = client.json_get_with_key(&url).await?;
    Ok(vec![response])
}

pub async fn create(
    client: &SemrushClient,
    name: &str,
    domain: &str,
) -> Result<Vec<serde_json::Value>, AppError> {
    let body = serde_json::json!({
        "project_name": name,
        "url": domain,
    });
    let response = client.json_post_with_key(V4_PROJECTS_BASE, &body).await?;
    Ok(vec![response])
}

pub async fn update(
    client: &SemrushClient,
    project_id: &str,
    name: &str,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut body = serde_json::Map::new();
    body.insert(
        "project_id".to_string(),
        serde_json::Value::String(project_id.to_string()),
    );
    body.insert(
        "project_name".to_string(),
        serde_json::Value::String(name.to_string()),
    );
    let response = client
        .json_put_with_key(V4_PROJECTS_BASE, &serde_json::Value::Object(body))
        .await?;
    Ok(vec![response])
}

pub async fn delete(
    client: &SemrushClient,
    project_id: &str,
) -> Result<Vec<serde_json::Value>, AppError> {
    let url = format!("{V4_PROJECTS_BASE}/{project_id}");
    client.json_delete_with_key(&url).await?;
    Ok(vec![serde_json::json!({"deleted": project_id})])
}

use crate::api::client::SemrushClient;
use crate::error::AppError;
use std::collections::HashMap;

fn base_params(
    target: &str,
    target_type: &str,
    export_columns: &str,
    limit: u32,
    offset: u32,
) -> HashMap<String, String> {
    let mut params = HashMap::new();
    params.insert("target".to_string(), target.to_string());
    params.insert("target_type".to_string(), target_type.to_string());
    if !export_columns.is_empty() {
        params.insert("export_columns".to_string(), export_columns.to_string());
    }
    params.insert("display_limit".to_string(), limit.to_string());
    params.insert("display_offset".to_string(), offset.to_string());
    params
}

pub async fn overview(
    client: &SemrushClient,
    target: &str,
    target_type: &str,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = HashMap::new();
    params.insert("target".to_string(), target.to_string());
    params.insert("target_type".to_string(), target_type.to_string());
    params.insert(
        "export_columns".to_string(),
        "backlinks_num,domains_num,ips_num,follows_num,nofollows_num,texts_num,images_num,forms_num,frames_num,score".to_string(),
    );
    client.v1_backlinks("backlinks_overview", &params).await
}

pub async fn list(
    client: &SemrushClient,
    target: &str,
    target_type: &str,
    limit: u32,
    offset: u32,
    filters: &[String],
    sort: Option<&str>,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = base_params(
        target,
        target_type,
        "page_score,source_url,source_title,target_url,anchor,external_num,internal_num,nofollow,first_seen,last_seen",
        limit,
        offset,
    );
    if let Some(s) = sort {
        params.insert("display_sort".to_string(), s.to_string());
    }
    for f in filters {
        // Backlinks API uses specific filter params like display_filter
        if let Some((key, val)) = f.split_once('=') {
            params.insert(key.to_string(), val.to_string());
        }
    }
    client.v1_backlinks("backlinks", &params).await
}

pub async fn referring_domains(
    client: &SemrushClient,
    target: &str,
    target_type: &str,
    limit: u32,
    offset: u32,
) -> Result<Vec<serde_json::Value>, AppError> {
    let params = base_params(
        target,
        target_type,
        "domain,backlinks_num,ip,first_seen,last_seen",
        limit,
        offset,
    );
    client.v1_backlinks("backlinks_refdomains", &params).await
}

pub async fn referring_ips(
    client: &SemrushClient,
    target: &str,
    target_type: &str,
    limit: u32,
    offset: u32,
) -> Result<Vec<serde_json::Value>, AppError> {
    let params = base_params(
        target,
        target_type,
        "ip,country,domains_num,backlinks_num,first_seen,last_seen",
        limit,
        offset,
    );
    client.v1_backlinks("backlinks_refips", &params).await
}

pub async fn tld_distribution(
    client: &SemrushClient,
    target: &str,
    target_type: &str,
    limit: u32,
    offset: u32,
) -> Result<Vec<serde_json::Value>, AppError> {
    let params = base_params(
        target,
        target_type,
        "zone,domains_num,backlinks_num",
        limit,
        offset,
    );
    client.v1_backlinks("backlinks_tld", &params).await
}

pub async fn geo(
    client: &SemrushClient,
    target: &str,
    target_type: &str,
    limit: u32,
    offset: u32,
) -> Result<Vec<serde_json::Value>, AppError> {
    let params = base_params(
        target,
        target_type,
        "country,domains_num,backlinks_num",
        limit,
        offset,
    );
    client.v1_backlinks("backlinks_geo", &params).await
}

pub async fn anchors(
    client: &SemrushClient,
    target: &str,
    target_type: &str,
    limit: u32,
    offset: u32,
) -> Result<Vec<serde_json::Value>, AppError> {
    let params = base_params(
        target,
        target_type,
        "anchor,domains_num,backlinks_num,first_seen,last_seen",
        limit,
        offset,
    );
    client.v1_backlinks("backlinks_anchors", &params).await
}

pub async fn indexed_pages(
    client: &SemrushClient,
    target: &str,
    target_type: &str,
    limit: u32,
    offset: u32,
) -> Result<Vec<serde_json::Value>, AppError> {
    let params = base_params(
        target,
        target_type,
        "source_url,external_num,internal_num,backlinks_num,first_seen,last_seen",
        limit,
        offset,
    );
    client.v1_backlinks("backlinks_pages", &params).await
}

pub async fn competitors(
    client: &SemrushClient,
    target: &str,
    target_type: &str,
    limit: u32,
    offset: u32,
) -> Result<Vec<serde_json::Value>, AppError> {
    let params = base_params(
        target,
        target_type,
        "domain,backlinks_num,domains_num,score",
        limit,
        offset,
    );
    client.v1_backlinks("backlinks_competitors", &params).await
}

pub async fn compare(
    client: &SemrushClient,
    targets: &[String],
    target_type: &str,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = HashMap::new();
    params.insert("targets".to_string(), targets.join(","));
    params.insert("target_type".to_string(), target_type.to_string());
    params.insert(
        "export_columns".to_string(),
        "target,backlinks_num,domains_num,ips_num,follows_num,nofollows_num,score".to_string(),
    );
    client.v1_backlinks("backlinks_matrix", &params).await
}

pub async fn batch(
    client: &SemrushClient,
    targets: &[String],
    target_type: &str,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = HashMap::new();
    params.insert("targets".to_string(), targets.join(","));
    params.insert("target_type".to_string(), target_type.to_string());
    params.insert(
        "export_columns".to_string(),
        "target,backlinks_num,domains_num,ips_num,score".to_string(),
    );
    client.v1_backlinks("backlinks_comparison", &params).await
}

pub async fn authority_score(
    client: &SemrushClient,
    target: &str,
    target_type: &str,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = HashMap::new();
    params.insert("target".to_string(), target.to_string());
    params.insert("target_type".to_string(), target_type.to_string());
    params.insert("export_columns".to_string(), "score".to_string());
    client
        .v1_backlinks("backlinks_ascore_profile", &params)
        .await
}

pub async fn categories(
    client: &SemrushClient,
    target: &str,
    target_type: &str,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = HashMap::new();
    params.insert("target".to_string(), target.to_string());
    params.insert("target_type".to_string(), target_type.to_string());
    client.v1_backlinks("backlinks_categories", &params).await
}

pub async fn category_profile(
    client: &SemrushClient,
    target: &str,
    target_type: &str,
    limit: u32,
    offset: u32,
) -> Result<Vec<serde_json::Value>, AppError> {
    let params = base_params(target, target_type, "", limit, offset);
    client
        .v1_backlinks("backlinks_categories_profile", &params)
        .await
}

pub async fn history(
    client: &SemrushClient,
    target: &str,
    target_type: &str,
    limit: u32,
    offset: u32,
) -> Result<Vec<serde_json::Value>, AppError> {
    let params = base_params(
        target,
        target_type,
        "backlinks_num,domains_num,date",
        limit,
        offset,
    );
    client.v1_backlinks("backlinks_historical", &params).await
}

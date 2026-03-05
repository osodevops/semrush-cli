use crate::api::client::SemrushClient;
use crate::api::columns;
use crate::error::AppError;
use std::collections::HashMap;

/// Build common params shared across v3 analytics requests.
fn base_params(
    database: &str,
    limit: u32,
    offset: u32,
    export_columns: &str,
) -> HashMap<String, String> {
    let mut params = HashMap::new();
    params.insert("database".to_string(), database.to_string());
    params.insert("display_limit".to_string(), limit.to_string());
    params.insert("display_offset".to_string(), offset.to_string());
    if !export_columns.is_empty() {
        params.insert("export_columns".to_string(), export_columns.to_string());
    }
    params
}

// ── Domain reports ─────────────────────────────────────────────

pub async fn domain_overview(
    client: &SemrushClient,
    domain: &str,
    database: &str,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = HashMap::new();
    params.insert("domain".to_string(), domain.to_string());
    params.insert("database".to_string(), database.to_string());
    let cols = columns::default_columns("domain_rank");
    if !cols.is_empty() {
        params.insert("export_columns".to_string(), cols.to_string());
    }
    client.v3_analytics("domain_rank", &params).await
}

pub async fn domain_overview_all_databases(
    client: &SemrushClient,
    domain: &str,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = HashMap::new();
    params.insert("domain".to_string(), domain.to_string());
    let cols = columns::default_columns("domain_ranks");
    if !cols.is_empty() {
        params.insert("export_columns".to_string(), cols.to_string());
    }
    client.v3_analytics("domain_ranks", &params).await
}

pub async fn domain_overview_history(
    client: &SemrushClient,
    domain: &str,
    database: &str,
    limit: u32,
    offset: u32,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = base_params(
        database,
        limit,
        offset,
        columns::default_columns("domain_rank_history"),
    );
    params.insert("domain".to_string(), domain.to_string());
    client.v3_analytics("domain_rank_history", &params).await
}

#[allow(clippy::too_many_arguments)]
pub async fn domain_organic(
    client: &SemrushClient,
    domain: &str,
    database: &str,
    limit: u32,
    offset: u32,
    filters: &[String],
    sort: Option<&str>,
    columns_override: Option<&str>,
    display_positions: Option<&str>,
) -> Result<Vec<serde_json::Value>, AppError> {
    let cols = columns_override.unwrap_or(columns::default_columns("domain_organic"));
    let mut params = base_params(database, limit, offset, cols);
    params.insert("domain".to_string(), domain.to_string());

    if let Some(pos) = display_positions {
        params.insert("display_positions".to_string(), pos.to_string());
    }
    if let Some(s) = sort {
        params.insert("display_sort".to_string(), s.to_string());
    }
    for (i, f) in filters.iter().enumerate() {
        params.insert(
            format!(
                "display_filter{}",
                if i == 0 {
                    String::new()
                } else {
                    format!("_{i}")
                }
            ),
            f.clone(),
        );
    }

    client.v3_analytics("domain_organic", &params).await
}

pub async fn domain_paid(
    client: &SemrushClient,
    domain: &str,
    database: &str,
    limit: u32,
    offset: u32,
    filters: &[String],
    sort: Option<&str>,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = base_params(
        database,
        limit,
        offset,
        columns::default_columns("domain_adwords"),
    );
    params.insert("domain".to_string(), domain.to_string());
    if let Some(s) = sort {
        params.insert("display_sort".to_string(), s.to_string());
    }
    for (i, f) in filters.iter().enumerate() {
        params.insert(
            format!(
                "display_filter{}",
                if i == 0 {
                    String::new()
                } else {
                    format!("_{i}")
                }
            ),
            f.clone(),
        );
    }
    client.v3_analytics("domain_adwords", &params).await
}

pub async fn domain_ads_copies(
    client: &SemrushClient,
    domain: &str,
    database: &str,
    limit: u32,
    offset: u32,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = base_params(
        database,
        limit,
        offset,
        columns::default_columns("domain_adwords_unique"),
    );
    params.insert("domain".to_string(), domain.to_string());
    client.v3_analytics("domain_adwords_unique", &params).await
}

pub async fn domain_ad_history(
    client: &SemrushClient,
    domain: &str,
    database: &str,
    limit: u32,
    offset: u32,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = base_params(
        database,
        limit,
        offset,
        columns::default_columns("domain_adwords_historical"),
    );
    params.insert("domain".to_string(), domain.to_string());
    client
        .v3_analytics("domain_adwords_historical", &params)
        .await
}

pub async fn domain_competitors_organic(
    client: &SemrushClient,
    domain: &str,
    database: &str,
    limit: u32,
    offset: u32,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = base_params(
        database,
        limit,
        offset,
        columns::default_columns("domain_organic_organic"),
    );
    params.insert("domain".to_string(), domain.to_string());
    client.v3_analytics("domain_organic_organic", &params).await
}

pub async fn domain_competitors_paid(
    client: &SemrushClient,
    domain: &str,
    database: &str,
    limit: u32,
    offset: u32,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = base_params(
        database,
        limit,
        offset,
        columns::default_columns("domain_adwords_adwords"),
    );
    params.insert("domain".to_string(), domain.to_string());
    client.v3_analytics("domain_adwords_adwords", &params).await
}

pub async fn domain_pla_keywords(
    client: &SemrushClient,
    domain: &str,
    database: &str,
    limit: u32,
    offset: u32,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = base_params(
        database,
        limit,
        offset,
        columns::default_columns("domain_shopping"),
    );
    params.insert("domain".to_string(), domain.to_string());
    client.v3_analytics("domain_shopping", &params).await
}

pub async fn domain_pla_copies(
    client: &SemrushClient,
    domain: &str,
    database: &str,
    limit: u32,
    offset: u32,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = base_params(
        database,
        limit,
        offset,
        columns::default_columns("domain_shopping_unique"),
    );
    params.insert("domain".to_string(), domain.to_string());
    client.v3_analytics("domain_shopping_unique", &params).await
}

pub async fn domain_pla_competitors(
    client: &SemrushClient,
    domain: &str,
    database: &str,
    limit: u32,
    offset: u32,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = base_params(
        database,
        limit,
        offset,
        columns::default_columns("domain_shopping_shopping"),
    );
    params.insert("domain".to_string(), domain.to_string());
    client
        .v3_analytics("domain_shopping_shopping", &params)
        .await
}

pub async fn domain_pages(
    client: &SemrushClient,
    domain: &str,
    database: &str,
    limit: u32,
    offset: u32,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = base_params(
        database,
        limit,
        offset,
        columns::default_columns("domain_organic_unique"),
    );
    params.insert("domain".to_string(), domain.to_string());
    client.v3_analytics("domain_organic_unique", &params).await
}

pub async fn domain_subdomains(
    client: &SemrushClient,
    domain: &str,
    database: &str,
    limit: u32,
    offset: u32,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = base_params(
        database,
        limit,
        offset,
        columns::default_columns("domain_organic_subdomains"),
    );
    params.insert("domain".to_string(), domain.to_string());
    client
        .v3_analytics("domain_organic_subdomains", &params)
        .await
}

pub async fn domain_compare(
    client: &SemrushClient,
    domains: &[String],
    database: &str,
    limit: u32,
    offset: u32,
    mode: Option<&str>,
    comparison_type: Option<&str>,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = base_params(
        database,
        limit,
        offset,
        columns::default_columns("domain_domains"),
    );

    // domain_domains expects domains as: domains=d1|d2|d3|d4|d5
    let domains_str = domains.join("|");
    params.insert("domains".to_string(), domains_str);

    if let Some(m) = mode {
        params.insert("display_filter".to_string(), format!("+|Se|{m}"));
    }
    if let Some(t) = comparison_type {
        // organic or paid
        let sign = if t == "paid" { "+" } else { "*" };
        params.insert("sign".to_string(), sign.to_string());
    }

    client.v3_analytics("domain_domains", &params).await
}

// ── Keyword reports ────────────────────────────────────────────

pub async fn keyword_overview(
    client: &SemrushClient,
    phrase: &str,
    database: &str,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = HashMap::new();
    params.insert("phrase".to_string(), phrase.to_string());
    params.insert("database".to_string(), database.to_string());
    let cols = columns::default_columns("phrase_this");
    if !cols.is_empty() {
        params.insert("export_columns".to_string(), cols.to_string());
    }
    client.v3_analytics("phrase_this", &params).await
}

pub async fn keyword_overview_all_databases(
    client: &SemrushClient,
    phrase: &str,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = HashMap::new();
    params.insert("phrase".to_string(), phrase.to_string());
    let cols = columns::default_columns("phrase_all");
    if !cols.is_empty() {
        params.insert("export_columns".to_string(), cols.to_string());
    }
    client.v3_analytics("phrase_all", &params).await
}

pub async fn keyword_batch(
    client: &SemrushClient,
    phrases: &[String],
    database: &str,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = HashMap::new();
    // phrase_these expects semicolon-separated keywords
    params.insert("phrase".to_string(), phrases.join(";"));
    params.insert("database".to_string(), database.to_string());
    let cols = columns::default_columns("phrase_these");
    if !cols.is_empty() {
        params.insert("export_columns".to_string(), cols.to_string());
    }
    client.v3_analytics("phrase_these", &params).await
}

pub async fn keyword_organic(
    client: &SemrushClient,
    phrase: &str,
    database: &str,
    limit: u32,
    offset: u32,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = base_params(
        database,
        limit,
        offset,
        columns::default_columns("phrase_organic"),
    );
    params.insert("phrase".to_string(), phrase.to_string());
    client.v3_analytics("phrase_organic", &params).await
}

pub async fn keyword_paid(
    client: &SemrushClient,
    phrase: &str,
    database: &str,
    limit: u32,
    offset: u32,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = base_params(
        database,
        limit,
        offset,
        columns::default_columns("phrase_adwords"),
    );
    params.insert("phrase".to_string(), phrase.to_string());
    client.v3_analytics("phrase_adwords", &params).await
}

pub async fn keyword_related(
    client: &SemrushClient,
    phrase: &str,
    database: &str,
    limit: u32,
    offset: u32,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = base_params(
        database,
        limit,
        offset,
        columns::default_columns("phrase_related"),
    );
    params.insert("phrase".to_string(), phrase.to_string());
    client.v3_analytics("phrase_related", &params).await
}

pub async fn keyword_broad_match(
    client: &SemrushClient,
    phrase: &str,
    database: &str,
    limit: u32,
    offset: u32,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = base_params(
        database,
        limit,
        offset,
        columns::default_columns("phrase_fullsearch"),
    );
    params.insert("phrase".to_string(), phrase.to_string());
    client.v3_analytics("phrase_fullsearch", &params).await
}

pub async fn keyword_questions(
    client: &SemrushClient,
    phrase: &str,
    database: &str,
    limit: u32,
    offset: u32,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = base_params(
        database,
        limit,
        offset,
        columns::default_columns("phrase_questions"),
    );
    params.insert("phrase".to_string(), phrase.to_string());
    client.v3_analytics("phrase_questions", &params).await
}

pub async fn keyword_difficulty(
    client: &SemrushClient,
    phrase: &str,
    database: &str,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = HashMap::new();
    params.insert("phrase".to_string(), phrase.to_string());
    params.insert("database".to_string(), database.to_string());
    let cols = columns::default_columns("phrase_kdi");
    if !cols.is_empty() {
        params.insert("export_columns".to_string(), cols.to_string());
    }
    client.v3_analytics("phrase_kdi", &params).await
}

pub async fn keyword_ad_history(
    client: &SemrushClient,
    phrase: &str,
    database: &str,
    limit: u32,
    offset: u32,
) -> Result<Vec<serde_json::Value>, AppError> {
    let mut params = base_params(
        database,
        limit,
        offset,
        columns::default_columns("phrase_adwords_historical"),
    );
    params.insert("phrase".to_string(), phrase.to_string());
    client
        .v3_analytics("phrase_adwords_historical", &params)
        .await
}

// ── Overview reports ───────────────────────────────────────────

pub async fn overview_rank(
    client: &SemrushClient,
    database: &str,
    limit: u32,
    offset: u32,
) -> Result<Vec<serde_json::Value>, AppError> {
    let params = base_params(database, limit, offset, columns::default_columns("rank"));
    client.v3_analytics("rank", &params).await
}

pub async fn overview_winners_losers(
    client: &SemrushClient,
    database: &str,
    limit: u32,
    offset: u32,
) -> Result<Vec<serde_json::Value>, AppError> {
    let params = base_params(
        database,
        limit,
        offset,
        columns::default_columns("rank_difference"),
    );
    client.v3_analytics("rank_difference", &params).await
}

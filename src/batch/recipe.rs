use crate::api::client::SemrushClient;
use crate::api::cost;
use crate::cache::DiskCache;
use crate::error::AppError;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Recipe {
    pub meta: RecipeMeta,
    #[serde(rename = "steps")]
    pub steps: Vec<Step>,
}

#[derive(Debug, Deserialize)]
pub struct RecipeMeta {
    pub name: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct Step {
    pub command: String,
    #[serde(default)]
    pub args: HashMap<String, toml::Value>,
    pub output_key: String,
}

impl Recipe {
    pub fn load(path: &str) -> Result<Self, AppError> {
        let content = std::fs::read_to_string(path).map_err(|e| AppError::InvalidParams {
            message: format!("Failed to read recipe file '{path}': {e}"),
        })?;
        toml::from_str(&content).map_err(|e| AppError::ParseError {
            message: format!("Failed to parse recipe TOML: {e}"),
        })
    }

    /// Substitute {{var}} placeholders in all step args.
    pub fn substitute_vars(&mut self, vars: &HashMap<String, String>) {
        for step in &mut self.steps {
            for value in step.args.values_mut() {
                if let toml::Value::String(s) = value {
                    for (key, val) in vars {
                        *s = s.replace(&format!("{{{{{key}}}}}"), val);
                    }
                }
            }
        }
    }

    /// Estimate total cost of all steps.
    pub fn estimate(&self) -> Vec<StepEstimate> {
        self.steps
            .iter()
            .map(|step| {
                let report_type_key = command_to_report_type_key(&step.command);
                let report_type = cost::report_type_for_command(&report_type_key);
                let limit = step
                    .args
                    .get("limit")
                    .and_then(|v| v.as_integer())
                    .unwrap_or(100) as u32;
                let est = cost::estimate(report_type, limit);
                StepEstimate {
                    command: step.command.clone(),
                    output_key: step.output_key.clone(),
                    estimated_units: est.estimated_units,
                    description: est.description,
                }
            })
            .collect()
    }

    /// Execute all steps sequentially and return combined results.
    pub async fn execute(
        &self,
        client: &SemrushClient,
        cache: &DiskCache,
        no_cache: bool,
    ) -> Result<serde_json::Value, AppError> {
        let mut results = serde_json::Map::new();

        for (i, step) in self.steps.iter().enumerate() {
            tracing::info!(
                step = i + 1,
                total = self.steps.len(),
                command = &step.command,
                "Executing batch step"
            );

            let data = execute_step(client, cache, step, no_cache).await?;
            results.insert(step.output_key.clone(), data);
        }

        Ok(serde_json::Value::Object(results))
    }
}

pub struct StepEstimate {
    pub command: String,
    pub output_key: String,
    pub estimated_units: u64,
    pub description: String,
}

/// Execute a single recipe step by dispatching to the appropriate API function.
async fn execute_step(
    client: &SemrushClient,
    cache: &DiskCache,
    step: &Step,
    no_cache: bool,
) -> Result<serde_json::Value, AppError> {
    let args = &step.args;
    let get_str = |key: &str| -> Option<String> {
        args.get(key)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    };
    let get_str_or = |key: &str, default: &str| -> String {
        get_str(key).unwrap_or_else(|| default.to_string())
    };
    let get_u32 = |key: &str, default: u32| -> u32 {
        args.get(key)
            .and_then(|v| v.as_integer())
            .map(|n| n as u32)
            .unwrap_or(default)
    };

    let report_type_key = command_to_report_type_key(&step.command);
    let cache_key = format!("batch|{}|{:?}", report_type_key, args);

    // Check cache
    if !no_cache {
        if let Some(cached) = cache.get(&report_type_key, &cache_key) {
            if let Ok(data) = serde_json::from_str::<serde_json::Value>(&cached) {
                return Ok(data);
            }
        }
    }

    let database = get_str_or("database", "us");
    let limit = get_u32("limit", 100);
    let offset = get_u32("offset", 0);

    let data: Vec<serde_json::Value> = match step.command.as_str() {
        "domain overview" => {
            let domain = get_str("domain").ok_or(AppError::InvalidParams {
                message: "Missing 'domain' arg in step".to_string(),
            })?;
            crate::api::v3_analytics::domain_overview(client, &domain, &database).await?
        }
        "domain organic" => {
            let domain = get_str("domain").ok_or(AppError::InvalidParams {
                message: "Missing 'domain' arg in step".to_string(),
            })?;
            crate::api::v3_analytics::domain_organic(
                client,
                &domain,
                &database,
                limit,
                offset,
                &[],
                None,
                None,
                None,
            )
            .await?
        }
        "domain paid" => {
            let domain = get_str("domain").ok_or(AppError::InvalidParams {
                message: "Missing 'domain' arg in step".to_string(),
            })?;
            crate::api::v3_analytics::domain_paid(
                client,
                &domain,
                &database,
                limit,
                offset,
                &[],
                None,
            )
            .await?
        }
        "domain competitors organic" => {
            let domain = get_str("domain").ok_or(AppError::InvalidParams {
                message: "Missing 'domain' arg in step".to_string(),
            })?;
            crate::api::v3_analytics::domain_competitors_organic(
                client, &domain, &database, limit, offset,
            )
            .await?
        }
        "domain competitors paid" => {
            let domain = get_str("domain").ok_or(AppError::InvalidParams {
                message: "Missing 'domain' arg in step".to_string(),
            })?;
            crate::api::v3_analytics::domain_competitors_paid(
                client, &domain, &database, limit, offset,
            )
            .await?
        }
        "keyword overview" => {
            let phrase = get_str("phrase").ok_or(AppError::InvalidParams {
                message: "Missing 'phrase' arg in step".to_string(),
            })?;
            crate::api::v3_analytics::keyword_overview(client, &phrase, &database).await?
        }
        "keyword related" => {
            let phrase = get_str("phrase").ok_or(AppError::InvalidParams {
                message: "Missing 'phrase' arg in step".to_string(),
            })?;
            crate::api::v3_analytics::keyword_related(client, &phrase, &database, limit, offset)
                .await?
        }
        "backlink overview" => {
            let target = get_str("target").ok_or(AppError::InvalidParams {
                message: "Missing 'target' arg in step".to_string(),
            })?;
            let target_type = get_str_or("target_type", "root_domain");
            crate::api::v1_backlinks::overview(client, &target, &target_type).await?
        }
        "backlink list" => {
            let target = get_str("target").ok_or(AppError::InvalidParams {
                message: "Missing 'target' arg in step".to_string(),
            })?;
            let target_type = get_str_or("target_type", "root_domain");
            crate::api::v1_backlinks::list(client, &target, &target_type, limit, offset, &[], None)
                .await?
        }
        "trends summary" => {
            let targets_str = get_str("targets").ok_or(AppError::InvalidParams {
                message: "Missing 'targets' arg in step".to_string(),
            })?;
            let targets: Vec<String> = targets_str
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
            let country = get_str("country");
            crate::api::v3_trends::summary(client, &targets, country.as_deref(), None, None, limit)
                .await?
        }
        other => {
            return Err(AppError::InvalidParams {
                message: format!("Unknown batch command: '{other}'. Supported: domain overview, domain organic, domain paid, domain competitors organic/paid, keyword overview, keyword related, backlink overview, backlink list, trends summary"),
            });
        }
    };

    let result = serde_json::Value::Array(data);

    // Store in cache
    if !no_cache {
        if let Ok(json_str) = serde_json::to_string(&result) {
            cache.set(&report_type_key, &cache_key, &json_str);
        }
    }

    Ok(result)
}

/// Map batch command strings to report type keys for cost estimation.
fn command_to_report_type_key(command: &str) -> String {
    match command {
        "domain overview" => "domain_overview",
        "domain organic" => "domain_organic",
        "domain paid" => "domain_paid",
        "domain competitors organic" => "domain_competitors_organic",
        "domain competitors paid" => "domain_competitors_paid",
        "keyword overview" => "keyword_overview",
        "keyword related" => "keyword_related",
        "backlink overview" => "backlink_overview",
        "backlink list" => "backlink_list",
        "trends summary" => "trends_summary",
        _ => "unknown",
    }
    .to_string()
}

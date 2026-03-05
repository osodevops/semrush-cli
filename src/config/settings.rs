use directories::ProjectDirs;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub auth: AuthConfig,
    #[serde(default)]
    pub defaults: DefaultsConfig,
    #[serde(default)]
    pub rate_limit: RateLimitConfig,
    #[serde(default)]
    pub cache: CacheConfig,
}

#[derive(Debug, Deserialize, Default)]
pub struct AuthConfig {
    pub api_key: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DefaultsConfig {
    #[serde(default = "default_database")]
    pub database: String,
    #[serde(default = "default_output")]
    pub output: String,
    #[serde(default = "default_limit")]
    pub limit: u32,
    #[serde(default = "default_cache_ttl")]
    pub cache_ttl: u64,
}

impl Default for DefaultsConfig {
    fn default() -> Self {
        Self {
            database: default_database(),
            output: default_output(),
            limit: default_limit(),
            cache_ttl: default_cache_ttl(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RateLimitConfig {
    #[serde(default = "default_rps")]
    pub requests_per_second: u32,
    #[serde(default = "default_concurrent")]
    pub max_concurrent: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_second: default_rps(),
            max_concurrent: default_concurrent(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CacheConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub directory: Option<String>,
    #[serde(default = "default_cache_max_mb")]
    pub max_size_mb: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            directory: None,
            max_size_mb: default_cache_max_mb(),
        }
    }
}

fn default_database() -> String {
    "us".to_string()
}
fn default_output() -> String {
    "json".to_string()
}
fn default_limit() -> u32 {
    100
}
fn default_cache_ttl() -> u64 {
    3600
}
fn default_rps() -> u32 {
    10
}
fn default_concurrent() -> u32 {
    10
}
fn default_true() -> bool {
    true
}
fn default_cache_max_mb() -> u64 {
    500
}

impl Config {
    pub fn load(config_path: Option<&str>) -> Self {
        let path = config_path
            .map(PathBuf::from)
            .or_else(|| std::env::var("SEMRUSH_CONFIG").ok().map(PathBuf::from))
            .unwrap_or_else(Self::default_config_path);

        if path.exists() {
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            toml::from_str(&content).unwrap_or_default()
        } else {
            Config::default()
        }
    }

    pub fn default_config_path() -> PathBuf {
        ProjectDirs::from("com", "semrush-rs", "semrush-rs")
            .map(|dirs| dirs.config_dir().join("config.toml"))
            .unwrap_or_else(|| PathBuf::from("~/.config/semrush-rs/config.toml"))
    }

    pub fn cache_dir(&self) -> PathBuf {
        self.cache
            .directory
            .as_ref()
            .map(PathBuf::from)
            .or_else(|| std::env::var("SEMRUSH_CACHE_DIR").ok().map(PathBuf::from))
            .unwrap_or_else(|| {
                ProjectDirs::from("com", "semrush-rs", "semrush-rs")
                    .map(|dirs| dirs.cache_dir().to_path_buf())
                    .unwrap_or_else(|| PathBuf::from("~/.cache/semrush-rs"))
            })
    }

    pub fn resolve_api_key(&self, cli_key: Option<&str>) -> Option<String> {
        cli_key
            .map(String::from)
            .or_else(|| std::env::var("SEMRUSH_API_KEY").ok())
            .or_else(|| self.auth.api_key.clone())
    }
}

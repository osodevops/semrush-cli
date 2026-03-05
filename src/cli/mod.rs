pub mod account;
pub mod backlink;
pub mod batch;
pub mod domain;
pub mod keyword;
pub mod local;
pub mod overview;
pub mod project;
pub mod trends;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "semrush",
    version,
    about = "A high-performance, agent-friendly CLI for the Semrush API",
    long_about = "semrush-rs provides comprehensive access to the Semrush API ecosystem.\n\
                  Designed as a first-class tool for AI agents with structured JSON output,\n\
                  built-in rate limiting, and local caching."
)]
pub struct Cli {
    /// Semrush API key (overrides config/env)
    #[arg(long, env = "SEMRUSH_API_KEY", global = true, hide_env_values = true)]
    pub api_key: Option<String>,

    /// Output format: json, table, csv, jsonl
    #[arg(long, global = true, env = "SEMRUSH_OUTPUT")]
    pub output: Option<String>,

    /// Regional database (e.g., us, uk, de, mobile-us)
    #[arg(long, global = true, default_value = "us", env = "SEMRUSH_DATABASE")]
    pub database: String,

    /// Max results to return
    #[arg(long, global = true, default_value = "100")]
    pub limit: u32,

    /// Skip first N results
    #[arg(long, global = true, default_value = "0")]
    pub offset: u32,

    /// Bypass local cache
    #[arg(long, global = true)]
    pub no_cache: bool,

    /// Cache TTL override in seconds
    #[arg(long, global = true)]
    pub cache_ttl: Option<u64>,

    /// Enable debug logging
    #[arg(long, global = true)]
    pub verbose: bool,

    /// Suppress non-data output
    #[arg(long, global = true)]
    pub quiet: bool,

    /// Config file path
    #[arg(long, global = true)]
    pub config: Option<String>,

    /// Show estimated API unit cost without executing
    #[arg(long, global = true)]
    pub dry_run: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Domain analytics — overview, organic, paid, competitors, etc.
    Domain {
        #[command(subcommand)]
        command: domain::DomainCommand,
    },

    /// Keyword research — overview, related, questions, difficulty, etc.
    Keyword {
        #[command(subcommand)]
        command: keyword::KeywordCommand,
    },

    /// Backlink analytics — overview, referring domains, anchors, etc.
    Backlink {
        #[command(subcommand)]
        command: backlink::BacklinkCommand,
    },

    /// Overview reports — Semrush Rank, winners/losers
    Overview {
        #[command(subcommand)]
        command: overview::OverviewCommand,
    },

    /// Traffic analytics (.Trends) — visits, sources, geo, top pages, etc.
    Trends {
        #[command(subcommand)]
        command: trends::TrendsCommand,
    },

    /// Project management (v4 API, requires OAuth2)
    Project {
        #[command(subcommand)]
        command: project::ProjectCommand,
    },

    /// Local SEO — Listing Management and Map Rank Tracker (v4, OAuth2)
    Local {
        #[command(subcommand)]
        command: local::LocalCommand,
    },

    /// Run batch workflows from TOML recipe files
    Batch {
        #[command(subcommand)]
        command: batch::BatchCommand,
    },

    /// Account management — API balance, auth setup
    Account {
        #[command(subcommand)]
        command: account::AccountCommand,
    },

    /// Manage local response cache
    Cache {
        #[command(subcommand)]
        command: CacheCommand,
    },

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        shell: clap_complete::Shell,
    },
}

#[derive(Subcommand)]
pub enum CacheCommand {
    /// Clear all cached responses
    Clear,
    /// Show cache statistics
    Stats,
}

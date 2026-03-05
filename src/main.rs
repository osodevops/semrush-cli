use semrush::api;
use semrush::batch;
use semrush::cache::DiskCache;
use semrush::cli;
use semrush::cli::{CacheCommand, Cli, Commands};
use semrush::config::Config;
use semrush::error::AppError;
use semrush::output;
use semrush::output::OutputFormat;

use clap::Parser;
use tracing_subscriber::EnvFilter;

/// Result of command resolution: the report type key and the data.
struct CommandResult {
    report_type: String,
    data: Vec<serde_json::Value>,
    cached: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Set up logging
    let filter = if cli.verbose {
        "semrush_rs=debug"
    } else {
        &std::env::var("SEMRUSH_LOG").unwrap_or_else(|_| "warn".to_string())
    };
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_new(filter).unwrap_or_else(|_| EnvFilter::new("warn")))
        .with_target(false)
        .with_writer(std::io::stderr)
        .init();

    let config = Config::load(cli.config.as_deref());
    let output_format = OutputFormat::from_str_or_detect(
        cli.output
            .as_deref()
            .or(Some(config.defaults.output.as_str())),
    );

    // Handle non-API commands first
    match &cli.command {
        Commands::Cache { command } => {
            let cache = DiskCache::new(config.cache_dir(), config.defaults.cache_ttl);
            match command {
                CacheCommand::Clear => match cache.clear() {
                    Ok(n) => println!("Cleared {n} cached entries."),
                    Err(e) => eprintln!("Failed to clear cache: {e}"),
                },
                CacheCommand::Stats => {
                    let (count, size) = cache.stats();
                    println!("Cache entries: {count}");
                    println!("Cache size: {} KB", size / 1024);
                    println!("Cache dir: {}", config.cache_dir().display());
                }
            }
            return;
        }
        Commands::Completions { shell } => {
            let mut cmd = <Cli as clap::CommandFactory>::command();
            clap_complete::generate(*shell, &mut cmd, "semrush", &mut std::io::stdout());
            return;
        }
        Commands::Account { command } => {
            handle_account(command, &cli, &config).await;
            return;
        }
        Commands::Batch { command } => {
            let api_key = match config.resolve_api_key(cli.api_key.as_deref()) {
                Some(key) => key,
                None => {
                    AppError::AuthFailed {
                        message: "No API key provided.".to_string(),
                    }
                    .print_and_exit();
                }
            };
            let client =
                api::client::SemrushClient::new(api_key, config.rate_limit.requests_per_second);
            let cache_ttl = cli.cache_ttl.unwrap_or(config.defaults.cache_ttl);
            let disk_cache = DiskCache::new(config.cache_dir(), cache_ttl);
            handle_batch(command, &client, &disk_cache, cli.no_cache).await;
            return;
        }
        _ => {}
    }

    // All remaining commands need an API key
    let api_key = match config.resolve_api_key(cli.api_key.as_deref()) {
        Some(key) => key,
        None => {
            AppError::AuthFailed {
                message: "No API key provided. Set SEMRUSH_API_KEY, use --api-key, or run `semrush account auth setup`.".to_string(),
            }
            .print_and_exit();
        }
    };

    // Resolve the report type for this command
    let report_type_key = resolve_report_type_key(&cli);

    // Handle --dry-run: estimate cost and exit
    if cli.dry_run {
        let report_type = api::cost::report_type_for_command(&report_type_key);
        let estimate = api::cost::estimate(report_type, cli.limit);
        println!("{estimate}");
        return;
    }

    let client = api::client::SemrushClient::new(api_key, config.rate_limit.requests_per_second);

    // Set up cache
    let cache_ttl = cli.cache_ttl.unwrap_or(config.defaults.cache_ttl);
    let cache = DiskCache::new(config.cache_dir(), cache_ttl);

    let result = execute_command(&cli, &client, &cache, &report_type_key).await;
    match result {
        Ok(cmd_result) => {
            let report_type = api::cost::report_type_for_command(&cmd_result.report_type);
            let cost_model = api::cost::cost_for_report(report_type);
            let estimated_units = match cost_model {
                api::cost::CostModel::PerLine(u) => u * cmd_result.data.len() as u64,
                api::cost::CostModel::PerRequest(u) => u,
            };

            let meta = serde_json::json!({
                "cached": cmd_result.cached,
                "database": cli.database,
                "result_count": cmd_result.data.len(),
                "report_type": report_type,
                "api_units_estimated": estimated_units,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });

            if !cli.quiet || output_format == OutputFormat::Json {
                let rendered = output::render(output_format, &cmd_result.data, &meta);
                println!("{rendered}");
            }
        }
        Err(e) => e.print_and_exit(),
    }
}

/// Try to serve from cache, otherwise execute the API call and cache the result.
async fn execute_command(
    cli: &Cli,
    client: &api::client::SemrushClient,
    cache: &DiskCache,
    report_type_key: &str,
) -> Result<CommandResult, AppError> {
    let cache_params = build_cache_key(cli, report_type_key);

    // Check cache first (unless --no-cache)
    if !cli.no_cache {
        if let Some(cached_json) = cache.get(report_type_key, &cache_params) {
            if let Ok(data) = serde_json::from_str::<Vec<serde_json::Value>>(&cached_json) {
                tracing::debug!("Serving from cache for {report_type_key}");
                return Ok(CommandResult {
                    report_type: report_type_key.to_string(),
                    data,
                    cached: true,
                });
            }
        }
    }

    // Execute the actual API call
    let data = run_api_command(cli, client).await?;

    // Store in cache (unless --no-cache)
    if !cli.no_cache {
        if let Ok(json_str) = serde_json::to_string(&data) {
            cache.set(report_type_key, &cache_params, &json_str);
        }
    }

    Ok(CommandResult {
        report_type: report_type_key.to_string(),
        data,
        cached: false,
    })
}

async fn run_api_command(
    cli: &Cli,
    client: &api::client::SemrushClient,
) -> Result<Vec<serde_json::Value>, AppError> {
    match &cli.command {
        Commands::Domain { command } => execute_domain(cli, client, command).await,
        Commands::Keyword { command } => execute_keyword(cli, client, command).await,
        Commands::Backlink { command } => execute_backlink(cli, client, command).await,
        Commands::Overview { command } => execute_overview(cli, client, command).await,
        Commands::Trends { command } => execute_trends(cli, client, command).await,
        Commands::Project { command } => execute_project(client, command).await,
        Commands::Local { command } => execute_local(client, command).await,
        _ => unreachable!(),
    }
}

/// Build a deterministic cache key string from command params (excluding API key).
fn build_cache_key(cli: &Cli, report_type_key: &str) -> String {
    format!(
        "{}|db={}|limit={}|offset={}",
        report_type_key, cli.database, cli.limit, cli.offset
    )
}

/// Resolve the internal report type key from the CLI command for cost estimation and caching.
fn resolve_report_type_key(cli: &Cli) -> String {
    match &cli.command {
        Commands::Domain { command } => {
            use cli::domain::DomainCommand;
            match command {
                DomainCommand::Overview {
                    all_databases,
                    history,
                    ..
                } => {
                    if *all_databases {
                        "domain_overview_all"
                    } else if *history {
                        "domain_overview_history"
                    } else {
                        "domain_overview"
                    }
                }
                DomainCommand::Organic { .. } => "domain_organic",
                DomainCommand::Paid { .. } => "domain_paid",
                DomainCommand::AdsCopies { .. } => "domain_ads_copies",
                DomainCommand::AdHistory { .. } => "domain_ad_history",
                DomainCommand::Competitors { command } => match command {
                    cli::domain::CompetitorsCommand::Organic { .. } => "domain_competitors_organic",
                    cli::domain::CompetitorsCommand::Paid { .. } => "domain_competitors_paid",
                },
                DomainCommand::PlaKeywords { .. } => "domain_pla_keywords",
                DomainCommand::PlaCopies { .. } => "domain_pla_copies",
                DomainCommand::PlaCompetitors { .. } => "domain_pla_competitors",
                DomainCommand::Pages { .. } => "domain_pages",
                DomainCommand::Subdomains { .. } => "domain_subdomains",
                DomainCommand::Compare { .. } => "domain_compare",
            }
        }
        Commands::Keyword { command } => {
            use cli::keyword::KeywordCommand;
            match command {
                KeywordCommand::Overview { all_databases, .. } => {
                    if *all_databases {
                        "keyword_overview_all"
                    } else {
                        "keyword_overview"
                    }
                }
                KeywordCommand::Batch { .. } => "keyword_batch",
                KeywordCommand::Organic { .. } => "keyword_organic",
                KeywordCommand::Paid { .. } => "keyword_paid",
                KeywordCommand::Related { .. } => "keyword_related",
                KeywordCommand::BroadMatch { .. } => "keyword_broad_match",
                KeywordCommand::Questions { .. } => "keyword_questions",
                KeywordCommand::Difficulty { .. } => "keyword_difficulty",
                KeywordCommand::AdHistory { .. } => "keyword_ad_history",
            }
        }
        Commands::Backlink { command } => {
            use cli::backlink::BacklinkCommand;
            match command {
                BacklinkCommand::Overview { .. } => "backlink_overview",
                BacklinkCommand::List { .. } => "backlink_list",
                BacklinkCommand::ReferringDomains { .. } => "backlink_referring_domains",
                BacklinkCommand::ReferringIps { .. } => "backlink_referring_ips",
                BacklinkCommand::TldDistribution { .. } => "backlink_tld_distribution",
                BacklinkCommand::Geo { .. } => "backlink_geo",
                BacklinkCommand::Anchors { .. } => "backlink_anchors",
                BacklinkCommand::IndexedPages { .. } => "backlink_indexed_pages",
                BacklinkCommand::Competitors { .. } => "backlink_competitors",
                BacklinkCommand::Compare { .. } => "backlink_compare",
                BacklinkCommand::Batch { .. } => "backlink_batch",
                BacklinkCommand::AuthorityScore { .. } => "backlink_authority_score",
                BacklinkCommand::Categories { .. } => "backlink_categories",
                BacklinkCommand::CategoryProfile { .. } => "backlink_category_profile",
                BacklinkCommand::History { .. } => "backlink_history",
            }
        }
        Commands::Overview { command } => {
            use cli::overview::OverviewCommand;
            match command {
                OverviewCommand::Rank => "overview_rank",
                OverviewCommand::WinnersLosers => "overview_winners_losers",
            }
        }
        Commands::Trends { command } => {
            use cli::trends::TrendsCommand;
            match command {
                TrendsCommand::Summary { .. } => "trends_summary",
                TrendsCommand::Daily { .. } => "trends_daily",
                TrendsCommand::Weekly { .. } => "trends_weekly",
                TrendsCommand::Sources { .. } => "trends_sources",
                TrendsCommand::Destinations { .. } => "trends_destinations",
                TrendsCommand::Geo { .. } => "trends_geo",
                TrendsCommand::Subdomains { .. } => "trends_subdomains",
                TrendsCommand::TopPages { .. } => "trends_top_pages",
                TrendsCommand::Rank { .. } => "trends_rank",
                TrendsCommand::Categories { .. } => "trends_categories",
                TrendsCommand::Conversion { .. } => "trends_conversion",
            }
        }
        Commands::Project { .. } => "project",
        Commands::Local { .. } => "local",
        _ => "unknown",
    }
    .to_string()
}

async fn execute_domain(
    cli: &Cli,
    client: &api::client::SemrushClient,
    command: &cli::domain::DomainCommand,
) -> Result<Vec<serde_json::Value>, AppError> {
    use cli::domain::DomainCommand;

    match command {
        DomainCommand::Overview {
            domain,
            all_databases,
            history,
        } => {
            if *all_databases {
                api::v3_analytics::domain_overview_all_databases(client, domain).await
            } else if *history {
                api::v3_analytics::domain_overview_history(
                    client,
                    domain,
                    &cli.database,
                    cli.limit,
                    cli.offset,
                )
                .await
            } else {
                api::v3_analytics::domain_overview(client, domain, &cli.database).await
            }
        }
        DomainCommand::Organic {
            domain,
            positions,
            filter,
            sort,
            columns,
        } => {
            api::v3_analytics::domain_organic(
                client,
                domain,
                &cli.database,
                cli.limit,
                cli.offset,
                filter,
                sort.as_deref(),
                columns.as_deref(),
                positions.as_deref(),
            )
            .await
        }
        DomainCommand::Paid {
            domain,
            filter,
            sort,
        } => {
            api::v3_analytics::domain_paid(
                client,
                domain,
                &cli.database,
                cli.limit,
                cli.offset,
                filter,
                sort.as_deref(),
            )
            .await
        }
        DomainCommand::AdsCopies { domain } => {
            api::v3_analytics::domain_ads_copies(
                client,
                domain,
                &cli.database,
                cli.limit,
                cli.offset,
            )
            .await
        }
        DomainCommand::AdHistory { domain } => {
            api::v3_analytics::domain_ad_history(
                client,
                domain,
                &cli.database,
                cli.limit,
                cli.offset,
            )
            .await
        }
        DomainCommand::Competitors { command } => match command {
            cli::domain::CompetitorsCommand::Organic { domain } => {
                api::v3_analytics::domain_competitors_organic(
                    client,
                    domain,
                    &cli.database,
                    cli.limit,
                    cli.offset,
                )
                .await
            }
            cli::domain::CompetitorsCommand::Paid { domain } => {
                api::v3_analytics::domain_competitors_paid(
                    client,
                    domain,
                    &cli.database,
                    cli.limit,
                    cli.offset,
                )
                .await
            }
        },
        DomainCommand::PlaKeywords { domain } => {
            api::v3_analytics::domain_pla_keywords(
                client,
                domain,
                &cli.database,
                cli.limit,
                cli.offset,
            )
            .await
        }
        DomainCommand::PlaCopies { domain } => {
            api::v3_analytics::domain_pla_copies(
                client,
                domain,
                &cli.database,
                cli.limit,
                cli.offset,
            )
            .await
        }
        DomainCommand::PlaCompetitors { domain } => {
            api::v3_analytics::domain_pla_competitors(
                client,
                domain,
                &cli.database,
                cli.limit,
                cli.offset,
            )
            .await
        }
        DomainCommand::Pages { domain } => {
            api::v3_analytics::domain_pages(client, domain, &cli.database, cli.limit, cli.offset)
                .await
        }
        DomainCommand::Subdomains { domain } => {
            api::v3_analytics::domain_subdomains(
                client,
                domain,
                &cli.database,
                cli.limit,
                cli.offset,
            )
            .await
        }
        DomainCommand::Compare {
            domains,
            mode,
            comparison_type,
        } => {
            api::v3_analytics::domain_compare(
                client,
                domains,
                &cli.database,
                cli.limit,
                cli.offset,
                mode.as_deref(),
                comparison_type.as_deref(),
            )
            .await
        }
    }
}

async fn execute_keyword(
    cli: &Cli,
    client: &api::client::SemrushClient,
    command: &cli::keyword::KeywordCommand,
) -> Result<Vec<serde_json::Value>, AppError> {
    use cli::keyword::KeywordCommand;

    match command {
        KeywordCommand::Overview {
            phrase,
            all_databases,
        } => {
            if *all_databases {
                api::v3_analytics::keyword_overview_all_databases(client, phrase).await
            } else {
                api::v3_analytics::keyword_overview(client, phrase, &cli.database).await
            }
        }
        KeywordCommand::Batch { phrases } => {
            api::v3_analytics::keyword_batch(client, phrases, &cli.database).await
        }
        KeywordCommand::Organic { phrase } => {
            api::v3_analytics::keyword_organic(client, phrase, &cli.database, cli.limit, cli.offset)
                .await
        }
        KeywordCommand::Paid { phrase } => {
            api::v3_analytics::keyword_paid(client, phrase, &cli.database, cli.limit, cli.offset)
                .await
        }
        KeywordCommand::Related { phrase } => {
            api::v3_analytics::keyword_related(client, phrase, &cli.database, cli.limit, cli.offset)
                .await
        }
        KeywordCommand::BroadMatch { phrase } => {
            api::v3_analytics::keyword_broad_match(
                client,
                phrase,
                &cli.database,
                cli.limit,
                cli.offset,
            )
            .await
        }
        KeywordCommand::Questions { phrase } => {
            api::v3_analytics::keyword_questions(
                client,
                phrase,
                &cli.database,
                cli.limit,
                cli.offset,
            )
            .await
        }
        KeywordCommand::Difficulty { phrase } => {
            api::v3_analytics::keyword_difficulty(client, phrase, &cli.database).await
        }
        KeywordCommand::AdHistory { phrase } => {
            api::v3_analytics::keyword_ad_history(
                client,
                phrase,
                &cli.database,
                cli.limit,
                cli.offset,
            )
            .await
        }
    }
}

async fn execute_backlink(
    cli: &Cli,
    client: &api::client::SemrushClient,
    command: &cli::backlink::BacklinkCommand,
) -> Result<Vec<serde_json::Value>, AppError> {
    use cli::backlink::BacklinkCommand;

    match command {
        BacklinkCommand::Overview {
            target,
            target_type,
        } => api::v1_backlinks::overview(client, target, target_type).await,

        BacklinkCommand::List {
            target,
            target_type,
            filter,
            sort,
        } => {
            api::v1_backlinks::list(
                client,
                target,
                target_type,
                cli.limit,
                cli.offset,
                filter,
                sort.as_deref(),
            )
            .await
        }

        BacklinkCommand::ReferringDomains {
            target,
            target_type,
        } => {
            api::v1_backlinks::referring_domains(client, target, target_type, cli.limit, cli.offset)
                .await
        }

        BacklinkCommand::ReferringIps {
            target,
            target_type,
        } => {
            api::v1_backlinks::referring_ips(client, target, target_type, cli.limit, cli.offset)
                .await
        }

        BacklinkCommand::TldDistribution {
            target,
            target_type,
        } => {
            api::v1_backlinks::tld_distribution(client, target, target_type, cli.limit, cli.offset)
                .await
        }

        BacklinkCommand::Geo {
            target,
            target_type,
        } => api::v1_backlinks::geo(client, target, target_type, cli.limit, cli.offset).await,

        BacklinkCommand::Anchors {
            target,
            target_type,
        } => api::v1_backlinks::anchors(client, target, target_type, cli.limit, cli.offset).await,

        BacklinkCommand::IndexedPages {
            target,
            target_type,
        } => {
            api::v1_backlinks::indexed_pages(client, target, target_type, cli.limit, cli.offset)
                .await
        }

        BacklinkCommand::Competitors {
            target,
            target_type,
        } => {
            api::v1_backlinks::competitors(client, target, target_type, cli.limit, cli.offset).await
        }

        BacklinkCommand::Compare {
            targets,
            target_type,
        } => api::v1_backlinks::compare(client, targets, target_type).await,

        BacklinkCommand::Batch {
            targets,
            target_type,
        } => api::v1_backlinks::batch(client, targets, target_type).await,

        BacklinkCommand::AuthorityScore {
            target,
            target_type,
        } => api::v1_backlinks::authority_score(client, target, target_type).await,

        BacklinkCommand::Categories {
            target,
            target_type,
        } => api::v1_backlinks::categories(client, target, target_type).await,

        BacklinkCommand::CategoryProfile {
            target,
            target_type,
        } => {
            api::v1_backlinks::category_profile(client, target, target_type, cli.limit, cli.offset)
                .await
        }

        BacklinkCommand::History {
            target,
            target_type,
        } => api::v1_backlinks::history(client, target, target_type, cli.limit, cli.offset).await,
    }
}

async fn execute_overview(
    cli: &Cli,
    client: &api::client::SemrushClient,
    command: &cli::overview::OverviewCommand,
) -> Result<Vec<serde_json::Value>, AppError> {
    use cli::overview::OverviewCommand;

    match command {
        OverviewCommand::Rank => {
            api::v3_analytics::overview_rank(client, &cli.database, cli.limit, cli.offset).await
        }
        OverviewCommand::WinnersLosers => {
            api::v3_analytics::overview_winners_losers(client, &cli.database, cli.limit, cli.offset)
                .await
        }
    }
}

async fn execute_trends(
    cli: &Cli,
    client: &api::client::SemrushClient,
    command: &cli::trends::TrendsCommand,
) -> Result<Vec<serde_json::Value>, AppError> {
    use cli::trends::TrendsCommand;

    match command {
        TrendsCommand::Summary {
            targets,
            device,
            country,
            date,
        } => {
            api::v3_trends::summary(
                client,
                targets,
                country.as_deref(),
                device.as_deref(),
                date.as_deref(),
                cli.limit,
            )
            .await
        }
        TrendsCommand::Daily {
            target,
            date_from,
            date_to,
            forecast,
            country,
            device,
        } => {
            api::v3_trends::daily(
                client,
                target,
                date_from.as_deref(),
                date_to.as_deref(),
                *forecast,
                country.as_deref(),
                device.as_deref(),
            )
            .await
        }
        TrendsCommand::Weekly {
            target,
            date_from,
            date_to,
            forecast,
            country,
            device,
        } => {
            api::v3_trends::weekly(
                client,
                target,
                date_from.as_deref(),
                date_to.as_deref(),
                *forecast,
                country.as_deref(),
                device.as_deref(),
            )
            .await
        }
        TrendsCommand::Sources {
            target,
            channel,
            traffic_type,
            country,
            device,
            date,
        } => {
            api::v3_trends::sources(
                client,
                target,
                channel.as_deref(),
                traffic_type.as_deref(),
                country.as_deref(),
                device.as_deref(),
                date.as_deref(),
            )
            .await
        }
        TrendsCommand::Destinations {
            target,
            country,
            device,
            date,
        } => {
            api::v3_trends::destinations(
                client,
                target,
                country.as_deref(),
                device.as_deref(),
                date.as_deref(),
            )
            .await
        }
        TrendsCommand::Geo {
            target,
            geo_type,
            country,
            device,
            date,
        } => {
            api::v3_trends::geo(
                client,
                target,
                geo_type.as_deref(),
                country.as_deref(),
                device.as_deref(),
                date.as_deref(),
            )
            .await
        }
        TrendsCommand::Subdomains {
            target,
            country,
            device,
            date,
        } => {
            api::v3_trends::subdomains(
                client,
                target,
                country.as_deref(),
                device.as_deref(),
                date.as_deref(),
            )
            .await
        }
        TrendsCommand::TopPages {
            target,
            country,
            device,
            date,
        } => {
            api::v3_trends::top_pages(
                client,
                target,
                country.as_deref(),
                device.as_deref(),
                date.as_deref(),
            )
            .await
        }
        TrendsCommand::Rank {
            country,
            device,
            date,
        } => {
            api::v3_trends::rank(
                client,
                country.as_deref(),
                device.as_deref(),
                date.as_deref(),
                cli.limit,
            )
            .await
        }
        TrendsCommand::Categories {
            category,
            country,
            device,
            date,
        } => {
            api::v3_trends::categories(
                client,
                category,
                country.as_deref(),
                device.as_deref(),
                date.as_deref(),
            )
            .await
        }
        TrendsCommand::Conversion {
            target,
            country,
            device,
            date,
        } => {
            api::v3_trends::conversion(
                client,
                target,
                country.as_deref(),
                device.as_deref(),
                date.as_deref(),
            )
            .await
        }
    }
}

async fn execute_project(
    client: &api::client::SemrushClient,
    command: &cli::project::ProjectCommand,
) -> Result<Vec<serde_json::Value>, AppError> {
    use cli::project::ProjectCommand;

    // v4 APIs need OAuth2 token — check env var for now
    let oauth_token = std::env::var("SEMRUSH_OAUTH_TOKEN").map_err(|_| AppError::AuthFailed {
        message: "OAuth2 token required for v4 API. Set SEMRUSH_OAUTH_TOKEN or run `semrush account auth setup-oauth`.".to_string(),
    })?;

    match command {
        ProjectCommand::List => api::v4_projects::list(client, &oauth_token).await,
        ProjectCommand::Get { project_id } => {
            api::v4_projects::get(client, &oauth_token, project_id).await
        }
        ProjectCommand::Create { name, domain } => {
            api::v4_projects::create(client, &oauth_token, name, domain).await
        }
        ProjectCommand::Update { project_id, name } => {
            api::v4_projects::update(client, &oauth_token, project_id, name.as_deref()).await
        }
        ProjectCommand::Delete { project_id } => {
            api::v4_projects::delete(client, &oauth_token, project_id).await
        }
    }
}

async fn execute_local(
    client: &api::client::SemrushClient,
    command: &cli::local::LocalCommand,
) -> Result<Vec<serde_json::Value>, AppError> {
    use cli::local::{ListingCommand, LocalCommand, MapRankCommand};

    let oauth_token = std::env::var("SEMRUSH_OAUTH_TOKEN").map_err(|_| AppError::AuthFailed {
        message: "OAuth2 token required for v4 API. Set SEMRUSH_OAUTH_TOKEN or run `semrush account auth setup-oauth`.".to_string(),
    })?;

    match command {
        LocalCommand::Listing { command } => match command {
            ListingCommand::List => api::v4_local::listing_list(client, &oauth_token).await,
            ListingCommand::Get { location_id } => {
                api::v4_local::listing_get(client, &oauth_token, location_id).await
            }
            ListingCommand::Create { json } => {
                let body = parse_json_input(json.as_deref())?;
                api::v4_local::listing_create(client, &oauth_token, &body).await
            }
            ListingCommand::Update { location_id, json } => {
                let body = parse_json_input(json.as_deref())?;
                api::v4_local::listing_update(client, &oauth_token, location_id, &body).await
            }
            ListingCommand::Delete { location_id } => {
                api::v4_local::listing_delete(client, &oauth_token, location_id).await
            }
        },
        LocalCommand::MapRank { command } => match command {
            MapRankCommand::Campaigns => {
                api::v4_local::map_rank_campaigns(client, &oauth_token).await
            }
            MapRankCommand::Keywords { campaign_id } => {
                api::v4_local::map_rank_keywords(client, &oauth_token, campaign_id).await
            }
            MapRankCommand::Heatmap { campaign_id } => {
                api::v4_local::map_rank_heatmap(client, &oauth_token, campaign_id).await
            }
            MapRankCommand::Competitors { campaign_id } => {
                api::v4_local::map_rank_competitors(client, &oauth_token, campaign_id).await
            }
        },
    }
}

fn parse_json_input(json_arg: Option<&str>) -> Result<serde_json::Value, AppError> {
    match json_arg {
        Some(s) => serde_json::from_str(s).map_err(|e| AppError::InvalidParams {
            message: format!("Invalid JSON: {e}"),
        }),
        None => {
            // Try reading from stdin
            let mut input = String::new();
            std::io::Read::read_to_string(&mut std::io::stdin(), &mut input).map_err(|e| {
                AppError::InvalidParams {
                    message: format!("Failed to read stdin: {e}"),
                }
            })?;
            serde_json::from_str(&input).map_err(|e| AppError::InvalidParams {
                message: format!("Invalid JSON from stdin: {e}"),
            })
        }
    }
}

async fn handle_batch(
    command: &cli::batch::BatchCommand,
    client: &api::client::SemrushClient,
    cache: &DiskCache,
    no_cache: bool,
) {
    use cli::batch::BatchCommand;

    match command {
        BatchCommand::Run { recipe, vars } => {
            let var_map = cli::batch::parse_vars(vars);
            let mut rec = match batch::Recipe::load(recipe) {
                Ok(r) => r,
                Err(e) => e.print_and_exit(),
            };
            rec.substitute_vars(&var_map);
            match rec.execute(client, cache, no_cache).await {
                Ok(results) => {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&results).unwrap_or_default()
                    );
                }
                Err(e) => e.print_and_exit(),
            }
        }
        BatchCommand::Estimate { recipe, vars } => {
            let var_map = cli::batch::parse_vars(vars);
            let mut rec = match batch::Recipe::load(recipe) {
                Ok(r) => r,
                Err(e) => e.print_and_exit(),
            };
            rec.substitute_vars(&var_map);
            let estimates = rec.estimate();
            let mut total = 0u64;
            for (i, est) in estimates.iter().enumerate() {
                println!(
                    "Step {} ({}): {} units ({})",
                    i + 1,
                    est.command,
                    est.estimated_units,
                    est.description
                );
                total += est.estimated_units;
            }
            println!("{}", "─".repeat(50));
            println!("Total estimated cost: {total} units");
        }
    }
}

async fn handle_account(command: &cli::account::AccountCommand, _cli: &Cli, config: &Config) {
    use cli::account::{AccountCommand, AuthCommand};

    match command {
        AccountCommand::Balance => {
            let api_key = config.resolve_api_key(_cli.api_key.as_deref());
            match api_key {
                Some(key) => {
                    println!(
                        "API key configured: {}...{}",
                        &key[..4.min(key.len())],
                        &key[key.len().saturating_sub(4)..]
                    );
                    println!("Use the Semrush web dashboard to check your API unit balance.");
                }
                None => {
                    eprintln!("No API key configured. Run `semrush account auth setup` first.");
                }
            }
        }
        AccountCommand::Auth { command } => match command {
            AuthCommand::Setup => {
                println!(
                    "To configure your API key, set the SEMRUSH_API_KEY environment variable:"
                );
                println!();
                println!("  export SEMRUSH_API_KEY=\"your-api-key-here\"");
                println!();
                println!(
                    "Or create a config file at: {}",
                    Config::default_config_path().display()
                );
                println!();
                println!("  [auth]");
                println!("  api_key = \"your-api-key-here\"");
            }
            AuthCommand::Status => {
                let has_key = config.resolve_api_key(_cli.api_key.as_deref()).is_some();
                if has_key {
                    println!("Status: Authenticated (API key configured)");
                } else {
                    println!("Status: Not authenticated");
                    println!("Run `semrush account auth setup` for instructions.");
                }
            }
        },
    }
}

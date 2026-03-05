/// Cost model: some reports charge per line, others per request.
#[derive(Debug, Clone, Copy)]
pub enum CostModel {
    PerLine(u64),
    PerRequest(u64),
}

/// Get the cost model for a given report type.
pub fn cost_for_report(report_type: &str) -> CostModel {
    match report_type {
        // Analytics v3 — per line
        "domain_rank" | "domain_ranks" | "domain_rank_history" => CostModel::PerLine(10),
        "domain_organic" | "domain_organic_unique" | "domain_organic_subdomains" => {
            CostModel::PerLine(10)
        }
        "domain_adwords" | "phrase_fullsearch" => CostModel::PerLine(20),
        "domain_shopping" => CostModel::PerLine(30),
        "domain_organic_organic" | "domain_adwords_adwords" => CostModel::PerLine(40),
        "domain_adwords_unique" => CostModel::PerLine(40),
        "phrase_related" | "phrase_questions" => CostModel::PerLine(40),
        "domain_shopping_unique" | "domain_shopping_shopping" => CostModel::PerLine(60),
        "domain_domains" => CostModel::PerLine(80),
        "domain_adwords_historical" | "phrase_adwords_historical" => CostModel::PerLine(100),
        "phrase_this" | "phrase_all" | "phrase_these" | "phrase_organic" => CostModel::PerLine(10),
        "phrase_adwords" => CostModel::PerLine(20),
        "phrase_kdi" => CostModel::PerLine(50),
        "rank" => CostModel::PerLine(10),
        "rank_difference" => CostModel::PerLine(20),

        // Backlinks v1
        "backlinks_overview" => CostModel::PerRequest(40),
        "backlinks"
        | "backlinks_refdomains"
        | "backlinks_refips"
        | "backlinks_tld"
        | "backlinks_geo"
        | "backlinks_anchors"
        | "backlinks_pages"
        | "backlinks_competitors"
        | "backlinks_matrix"
        | "backlinks_comparison"
        | "backlinks_categories_profile"
        | "backlinks_historical" => CostModel::PerLine(40),
        "backlinks_ascore_profile" => CostModel::PerRequest(100),
        "backlinks_categories" => CostModel::PerRequest(50),

        // Trends v3
        "trends_summary" => CostModel::PerLine(1),
        "trends_summary_by_day"
        | "trends_summary_by_week"
        | "trends_sources"
        | "trends_destinations"
        | "trends_geo"
        | "trends_subdomains"
        | "trends_toppages"
        | "trends_rank"
        | "trends_purchase_conversion" => CostModel::PerRequest(1),
        "trends_categories" => CostModel::PerRequest(500),

        _ => CostModel::PerLine(10), // conservative default
    }
}

/// Estimate the cost given the report type and requested limit.
pub fn estimate(report_type: &str, limit: u32) -> CostEstimate {
    let model = cost_for_report(report_type);
    let (units, description) = match model {
        CostModel::PerLine(cost) => {
            let total = cost * limit as u64;
            (
                total,
                format!("{cost} units/line x {limit} lines = {total} units"),
            )
        }
        CostModel::PerRequest(cost) => (cost, format!("{cost} units/request")),
    };

    CostEstimate {
        report_type: report_type.to_string(),
        estimated_units: units,
        description,
    }
}

#[derive(Debug)]
pub struct CostEstimate {
    pub report_type: String,
    pub estimated_units: u64,
    pub description: String,
}

impl std::fmt::Display for CostEstimate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Estimated cost: {} API units ({})\nRun without --dry-run to execute.",
            self.estimated_units, self.description
        )
    }
}

/// Map a CLI command path to its Semrush report type for cost estimation.
/// Returns the report type string used in cost_for_report().
pub fn report_type_for_command(command_path: &str) -> &str {
    match command_path {
        // Domain
        "domain_overview" => "domain_rank",
        "domain_overview_all" => "domain_ranks",
        "domain_overview_history" => "domain_rank_history",
        "domain_organic" => "domain_organic",
        "domain_paid" => "domain_adwords",
        "domain_ads_copies" => "domain_adwords_unique",
        "domain_ad_history" => "domain_adwords_historical",
        "domain_competitors_organic" => "domain_organic_organic",
        "domain_competitors_paid" => "domain_adwords_adwords",
        "domain_pla_keywords" => "domain_shopping",
        "domain_pla_copies" => "domain_shopping_unique",
        "domain_pla_competitors" => "domain_shopping_shopping",
        "domain_pages" => "domain_organic_unique",
        "domain_subdomains" => "domain_organic_subdomains",
        "domain_compare" => "domain_domains",
        // Keyword
        "keyword_overview" => "phrase_this",
        "keyword_overview_all" => "phrase_all",
        "keyword_batch" => "phrase_these",
        "keyword_organic" => "phrase_organic",
        "keyword_paid" => "phrase_adwords",
        "keyword_related" => "phrase_related",
        "keyword_broad_match" => "phrase_fullsearch",
        "keyword_questions" => "phrase_questions",
        "keyword_difficulty" => "phrase_kdi",
        "keyword_ad_history" => "phrase_adwords_historical",
        // Overview
        "overview_rank" => "rank",
        "overview_winners_losers" => "rank_difference",
        // Backlinks
        "backlink_overview" => "backlinks_overview",
        "backlink_list" => "backlinks",
        "backlink_referring_domains" => "backlinks_refdomains",
        "backlink_referring_ips" => "backlinks_refips",
        "backlink_tld_distribution" => "backlinks_tld",
        "backlink_geo" => "backlinks_geo",
        "backlink_anchors" => "backlinks_anchors",
        "backlink_indexed_pages" => "backlinks_pages",
        "backlink_competitors" => "backlinks_competitors",
        "backlink_compare" => "backlinks_matrix",
        "backlink_batch" => "backlinks_comparison",
        "backlink_authority_score" => "backlinks_ascore_profile",
        "backlink_categories" => "backlinks_categories",
        "backlink_category_profile" => "backlinks_categories_profile",
        "backlink_history" => "backlinks_historical",
        // Trends
        "trends_summary" => "trends_summary",
        "trends_daily" => "trends_summary_by_day",
        "trends_weekly" => "trends_summary_by_week",
        "trends_sources" => "trends_sources",
        "trends_destinations" => "trends_destinations",
        "trends_geo" => "trends_geo",
        "trends_subdomains" => "trends_subdomains",
        "trends_top_pages" => "trends_toppages",
        "trends_rank" => "trends_rank",
        "trends_categories" => "trends_categories",
        "trends_conversion" => "trends_purchase_conversion",

        _ => "unknown",
    }
}

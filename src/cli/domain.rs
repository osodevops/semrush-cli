use clap::Subcommand;

#[derive(Subcommand)]
pub enum DomainCommand {
    /// Get domain overview metrics (rank, traffic, keywords count)
    Overview {
        /// Domain to analyze (e.g., example.com)
        domain: String,

        /// Show data for all regional databases
        #[arg(long)]
        all_databases: bool,

        /// Show historical data
        #[arg(long)]
        history: bool,
    },

    /// Get domain's organic search keywords and positions
    Organic {
        /// Domain to analyze
        domain: String,

        /// Filter by position type: new, lost, risen, fallen
        #[arg(long)]
        positions: Option<String>,

        /// Filter expressions (e.g., "position<=10")
        #[arg(long, num_args = 1..)]
        filter: Vec<String>,

        /// Sort expression (e.g., "traffic desc")
        #[arg(long)]
        sort: Option<String>,

        /// Columns to include (comma-separated)
        #[arg(long)]
        columns: Option<String>,
    },

    /// Get domain's paid search keywords
    Paid {
        /// Domain to analyze
        domain: String,

        #[arg(long, num_args = 1..)]
        filter: Vec<String>,

        #[arg(long)]
        sort: Option<String>,
    },

    /// Get domain's ad copies
    #[command(name = "ads-copies")]
    AdsCopies {
        /// Domain to analyze
        domain: String,
    },

    /// Get domain's ad history
    #[command(name = "ad-history")]
    AdHistory {
        /// Domain to analyze
        domain: String,
    },

    /// Get domain's competitors
    Competitors {
        #[command(subcommand)]
        command: CompetitorsCommand,
    },

    /// Get domain's PLA (Product Listing Ads) keywords
    #[command(name = "pla-keywords")]
    PlaKeywords { domain: String },

    /// Get domain's PLA ad copies
    #[command(name = "pla-copies")]
    PlaCopies { domain: String },

    /// Get domain's PLA competitors
    #[command(name = "pla-competitors")]
    PlaCompetitors { domain: String },

    /// Get domain's top pages by organic traffic
    Pages { domain: String },

    /// Get domain's subdomains
    Subdomains { domain: String },

    /// Compare domains (keyword gap analysis)
    Compare {
        /// Domains to compare (2-5)
        domains: Vec<String>,

        /// Comparison mode: shared, all, unique, untapped, missing, exclusive
        #[arg(long)]
        mode: Option<String>,

        /// Comparison type: organic, paid
        #[arg(long, name = "type")]
        comparison_type: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum CompetitorsCommand {
    /// Organic search competitors
    Organic { domain: String },
    /// Paid search competitors
    Paid { domain: String },
}

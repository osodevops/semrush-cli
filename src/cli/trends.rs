use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum TrendsCommand {
    /// Get traffic summary for one or more domains (up to 200)
    Summary {
        /// Domains to analyze (comma-separated or multiple args)
        #[arg(required = true, num_args = 1..=200)]
        targets: Vec<String>,

        /// Device type: desktop, mobile
        #[arg(long)]
        device: Option<String>,

        /// Country code (ISO 2-letter, e.g., US, GB, DE)
        #[arg(long)]
        country: Option<String>,

        /// Display date (YYYY-MM-DD)
        #[arg(long)]
        date: Option<String>,
    },

    /// Get daily traffic breakdown
    Daily {
        /// Domain to analyze
        target: String,

        /// Month to report (YYYY-MM-01)
        #[arg(long, alias = "date-from")]
        date: Option<String>,

        /// Include forecasted data
        #[arg(long)]
        forecast: bool,

        #[arg(long)]
        country: Option<String>,

        #[arg(long)]
        device: Option<String>,
    },

    /// Get weekly traffic breakdown
    Weekly {
        target: String,

        /// Month to report (YYYY-MM-01)
        #[arg(long, alias = "date-from")]
        date: Option<String>,

        #[arg(long)]
        forecast: bool,

        #[arg(long)]
        country: Option<String>,

        #[arg(long)]
        device: Option<String>,
    },

    /// Get traffic sources breakdown
    Sources {
        target: String,

        /// Channel: direct, referral, search, social, mail, display_ad, ai_assistants, ai_search
        #[arg(long)]
        channel: Option<String>,

        /// Traffic type: organic, paid
        #[arg(long)]
        traffic_type: Option<String>,

        #[arg(long)]
        country: Option<String>,

        #[arg(long)]
        device: Option<String>,

        #[arg(long)]
        date: Option<String>,
    },

    /// Get traffic destinations
    Destinations {
        target: String,

        #[arg(long)]
        country: Option<String>,

        #[arg(long)]
        device: Option<String>,

        #[arg(long)]
        date: Option<String>,
    },

    /// Get geographic distribution of traffic
    Geo {
        target: String,

        /// Geo type: country, subcontinent, continent
        #[arg(long)]
        geo_type: Option<String>,

        #[arg(long)]
        country: Option<String>,

        #[arg(long)]
        device: Option<String>,

        #[arg(long)]
        date: Option<String>,
    },

    /// Get subdomains by traffic share
    Subdomains {
        target: String,

        #[arg(long)]
        country: Option<String>,

        #[arg(long)]
        device: Option<String>,

        #[arg(long)]
        date: Option<String>,
    },

    /// Get top pages by traffic
    #[command(name = "top-pages")]
    TopPages {
        target: String,

        #[arg(long)]
        country: Option<String>,

        #[arg(long)]
        device: Option<String>,

        #[arg(long)]
        date: Option<String>,
    },

    /// Get top domains by traffic rank
    Rank {
        #[arg(long)]
        country: Option<String>,

        #[arg(long)]
        device: Option<String>,

        #[arg(long)]
        date: Option<String>,
    },

    /// Get industry category breakdown
    Categories {
        /// Category ID or slug
        category: String,

        #[arg(long)]
        country: Option<String>,

        #[arg(long)]
        device: Option<String>,

        #[arg(long)]
        date: Option<String>,
    },

    /// Get purchase conversion data
    Conversion {
        target: String,

        #[arg(long)]
        country: Option<String>,

        #[arg(long)]
        device: Option<String>,

        #[arg(long)]
        date: Option<String>,
    },
}

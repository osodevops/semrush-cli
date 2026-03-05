use clap::Subcommand;

#[derive(Subcommand)]
pub enum KeywordCommand {
    /// Get keyword metrics: search volume, CPC, difficulty, competition, intent
    Overview {
        /// Keyword phrase to analyze
        phrase: String,

        /// Show data for all regional databases
        #[arg(long)]
        all_databases: bool,
    },

    /// Get metrics for multiple keywords at once (up to 100)
    Batch {
        /// Keywords to analyze
        phrases: Vec<String>,
    },

    /// Get domains ranking organically for a keyword
    Organic { phrase: String },

    /// Get domains with paid ads for a keyword
    Paid { phrase: String },

    /// Get related keywords
    Related { phrase: String },

    /// Get broad match keywords
    #[command(name = "broad-match")]
    BroadMatch { phrase: String },

    /// Get question-based keywords
    Questions { phrase: String },

    /// Get keyword difficulty score
    Difficulty { phrase: String },

    /// Get keyword ad history
    #[command(name = "ad-history")]
    AdHistory { phrase: String },
}

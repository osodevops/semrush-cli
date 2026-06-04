use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum LocalCommand {
    /// Listing Management — manage local business listings
    Listing {
        #[command(subcommand)]
        command: ListingCommand,
    },

    /// Map Rank Tracker — local ranking data
    #[command(name = "map-rank")]
    MapRank {
        #[command(subcommand)]
        command: MapRankCommand,
    },
}

#[derive(Debug, Subcommand)]
pub enum ListingCommand {
    /// List all locations
    List,

    /// Get a specific location
    Get {
        /// Location ID
        location_id: String,
    },

    /// Create a new location (pass JSON body via stdin)
    Create {
        /// JSON body for the listing (or use stdin)
        #[arg(long)]
        json: Option<String>,
    },

    /// Update a location (pass JSON body via stdin)
    Update {
        /// Location ID
        location_id: String,

        /// JSON body for the update (or use stdin)
        #[arg(long)]
        json: Option<String>,
    },

    /// Delete a location
    Delete {
        /// Location ID
        location_id: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum MapRankCommand {
    /// List Map Rank Tracker campaigns
    Campaigns,

    /// Get keywords for a campaign
    Keywords {
        /// Campaign ID
        campaign_id: String,

        /// Report date to query, if not using the latest report
        #[arg(long)]
        report_date: Option<String>,
    },

    /// Get heatmap data for a campaign keyword
    Heatmap {
        /// Campaign ID
        campaign_id: String,

        /// Keyword ID
        #[arg(long)]
        keyword_id: String,

        /// Google business CID. Either --cid or --place-ids is required.
        #[arg(long)]
        cid: Option<String>,

        /// Google Place IDs. Either --cid or --place-ids is required.
        #[arg(long, value_delimiter = ',')]
        place_ids: Vec<String>,

        /// Report date to query, if not using the latest report
        #[arg(long)]
        report_date: Option<String>,
    },

    /// Get competitors for a campaign keyword
    Competitors {
        /// Campaign ID
        campaign_id: String,

        /// Keyword ID
        #[arg(long)]
        keyword_id: String,

        /// Report date in ISO-8601 format
        #[arg(long)]
        report_date: String,
    },
}

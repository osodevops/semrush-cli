use clap::Subcommand;

#[derive(Subcommand)]
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

#[derive(Subcommand)]
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

#[derive(Subcommand)]
pub enum MapRankCommand {
    /// List Map Rank Tracker campaigns
    Campaigns,

    /// Get keywords for a campaign
    Keywords {
        /// Campaign ID
        campaign_id: String,
    },

    /// Get heatmap data for a campaign
    Heatmap {
        /// Campaign ID
        campaign_id: String,
    },

    /// Get competitors for a campaign
    Competitors {
        /// Campaign ID
        campaign_id: String,
    },
}

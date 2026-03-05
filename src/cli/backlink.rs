use clap::Subcommand;

#[derive(Subcommand)]
pub enum BacklinkCommand {
    /// Get backlink overview metrics (total backlinks, referring domains, authority score)
    Overview {
        /// Target domain/URL to analyze
        target: String,

        /// Target type: root_domain, domain, url
        #[arg(long, default_value = "root_domain")]
        target_type: String,
    },

    /// List individual backlinks
    List {
        target: String,

        #[arg(long, default_value = "root_domain")]
        target_type: String,

        /// Filter expressions (e.g., "type=text", "newlink=true")
        #[arg(long, num_args = 1..)]
        filter: Vec<String>,

        #[arg(long)]
        sort: Option<String>,
    },

    /// Get referring domains
    #[command(name = "referring-domains")]
    ReferringDomains {
        target: String,
        #[arg(long, default_value = "root_domain")]
        target_type: String,
    },

    /// Get referring IPs
    #[command(name = "referring-ips")]
    ReferringIps {
        target: String,
        #[arg(long, default_value = "root_domain")]
        target_type: String,
    },

    /// Get TLD distribution of backlinks
    #[command(name = "tld-distribution")]
    TldDistribution {
        target: String,
        #[arg(long, default_value = "root_domain")]
        target_type: String,
    },

    /// Get geographic distribution of backlinks
    Geo {
        target: String,
        #[arg(long, default_value = "root_domain")]
        target_type: String,
    },

    /// Get anchor text distribution
    Anchors {
        target: String,
        #[arg(long, default_value = "root_domain")]
        target_type: String,
    },

    /// Get indexed pages with backlinks
    #[command(name = "indexed-pages")]
    IndexedPages {
        target: String,
        #[arg(long, default_value = "root_domain")]
        target_type: String,
    },

    /// Get backlink competitors
    Competitors {
        target: String,
        #[arg(long, default_value = "root_domain")]
        target_type: String,
    },

    /// Compare backlink profiles of multiple targets
    Compare {
        /// Targets to compare
        targets: Vec<String>,
        #[arg(long, default_value = "root_domain")]
        target_type: String,
    },

    /// Batch comparison of multiple targets (up to 200)
    Batch {
        targets: Vec<String>,
        #[arg(long, default_value = "root_domain")]
        target_type: String,
    },

    /// Get authority score profile
    #[command(name = "authority-score")]
    AuthorityScore {
        target: String,
        #[arg(long, default_value = "root_domain")]
        target_type: String,
    },

    /// Get backlink categories
    Categories {
        target: String,
        #[arg(long, default_value = "root_domain")]
        target_type: String,
    },

    /// Get backlink category profile
    #[command(name = "category-profile")]
    CategoryProfile {
        target: String,
        #[arg(long, default_value = "root_domain")]
        target_type: String,
    },

    /// Get backlink history
    History {
        target: String,
        #[arg(long, default_value = "root_domain")]
        target_type: String,
    },
}

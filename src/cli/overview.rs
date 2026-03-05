use clap::Subcommand;

#[derive(Subcommand)]
pub enum OverviewCommand {
    /// Get Semrush Rank — top domains by visibility
    Rank,

    /// Get winners and losers — domains with biggest rank changes
    #[command(name = "winners-losers")]
    WinnersLosers,
}

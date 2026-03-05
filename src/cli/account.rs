use clap::Subcommand;

#[derive(Subcommand)]
pub enum AccountCommand {
    /// Check API unit balance
    Balance,

    /// Authentication management
    Auth {
        #[command(subcommand)]
        command: AuthCommand,
    },
}

#[derive(Subcommand)]
pub enum AuthCommand {
    /// Set up API key authentication
    Setup,
    /// Show current authentication status
    Status,
}

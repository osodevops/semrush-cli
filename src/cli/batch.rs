use clap::Subcommand;

#[derive(Subcommand)]
pub enum BatchCommand {
    /// Execute a TOML batch recipe
    Run {
        /// Path to the recipe TOML file
        recipe: String,

        /// Variable substitutions (key=value)
        #[arg(long = "var", num_args = 1..)]
        vars: Vec<String>,
    },

    /// Estimate API unit cost of a batch recipe without executing
    Estimate {
        /// Path to the recipe TOML file
        recipe: String,

        /// Variable substitutions (key=value)
        #[arg(long = "var", num_args = 1..)]
        vars: Vec<String>,
    },
}

/// Parse --var key=value pairs into a HashMap.
pub fn parse_vars(vars: &[String]) -> std::collections::HashMap<String, String> {
    vars.iter()
        .filter_map(|v| {
            v.split_once('=')
                .map(|(k, val)| (k.to_string(), val.to_string()))
        })
        .collect()
}

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum ProjectCommand {
    /// List all projects
    List,

    /// Get a specific project
    Get {
        /// Project ID
        project_id: String,
    },

    /// Create a new project
    Create {
        /// Project name
        #[arg(long)]
        name: String,

        /// Project domain (e.g., example.com)
        #[arg(long)]
        domain: String,
    },

    /// Update a project
    Update {
        /// Project ID
        project_id: String,

        /// New project name
        #[arg(long)]
        name: String,
    },

    /// Delete a project
    Delete {
        /// Project ID
        project_id: String,
    },
}

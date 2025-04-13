use std::path::PathBuf;
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new project
    Create(CreateArgs),
    /// Plays the project live
    Play(PlayArgs),
    /// Exports the project to an audio file
    Export(ExportArgs),
}

#[derive(Args)]
pub struct CreateArgs {
    /// Name of the project to create
    pub name: String,

    /// Path where the project directory should be created
    pub path: Option<PathBuf>,
}

#[derive(Args)]
pub struct PlayArgs{
    /// Path to the project directory
    pub path: Option<PathBuf>,
}

#[derive(Args)]
pub struct ExportArgs{
    /// Path to the project directory
    pub project_path: PathBuf,
    /// Path to the export directory
    pub export_path: PathBuf,
    /// Type of file to create
    pub format: Option<String>,
}



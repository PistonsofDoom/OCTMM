use crate::{cli::Cli, cli::Commands, project::Project};
use clap::Parser;
use std::env;
use std::path::PathBuf;

mod cli;
mod project;
mod test_utils;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Create(args)) => {
            let path: PathBuf;

            if args.path.is_none() {
                path = env::current_dir().expect("Couldn't get current directory");
            } else {
                path = args.path.clone().unwrap();
            }

            Project::new(&path, &args.name).expect("Failed to create project");
        }
        Some(Commands::Play(args)) => {
            println!("play: {:?}", args.path);
            println!("unimplemented");
        }
        Some(Commands::Export(args)) => {
            println!(
                "export: {:?}, {:?}, {:?}",
                args.project_path, args.export_path, args.format
            );
            println!("unimplemented");
        }
        None => {}
    }
}

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
                let current_dir = env::current_dir();

                if current_dir.is_err() {
                    println!("ERROR: Couldn't get current directory.");
                    println!("Please specify path manually.");
                    return;
                }

                path = current_dir.unwrap();
            } else {
                path = args.path.clone().unwrap();
            }

            let result = Project::new(&path, &args.name);

            if result.is_err() {
                println!(
                    "Creating project failed with an error:\n  {}",
                    result.unwrap_err().to_string()
                );
            }
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

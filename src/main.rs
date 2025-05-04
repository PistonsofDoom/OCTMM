use crate::{cli::Cli, cli::Commands, project::Project, runner::Runner};
use clap::Parser;
use std::env;
use std::path::PathBuf;

mod cli;
mod project;
mod runner;
mod test_utils;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Create(args)) => {
            let path: PathBuf;

            // If no path is specified, use the current directory
            if args.path.is_none() {
                path = env::current_dir().expect("Couldn't get current directory");
            } else {
                path = args.path.clone().unwrap();
            }

            Project::new(&path, &args.name).expect("Failed to create project");
        }
        Some(Commands::Play(args)) => {
            let path: PathBuf;

            // If no path is specified, use the current directory
            if args.path.is_none() {
                path = env::current_dir().expect("Couldn't get current directory");
            } else {
                path = args.path.clone().unwrap();
            }

            let project = Project::load(&path).expect("Couldn't load project");
            let runner = Runner::new(project);

            runner.run();
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

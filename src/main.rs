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

            Project::create(&path, &args.name).expect("Failed to create project");
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
            let mut runner = Runner::new(project);

            runner.run(None);
        }
        Some(Commands::Export(args)) => {
            let project_path: PathBuf;
            let export_path: PathBuf;

            // If no paths are specified, just use the current directory
            if args.project_path.is_none() {
                project_path = env::current_dir().expect("Couldn't get current directory");
            } else {
                project_path = args.project_path.clone().unwrap();
            }
            if args.export_path.is_none() {
                export_path = env::current_dir().expect("Couldn't get current directory");
            } else {
                export_path = args.export_path.clone().unwrap();
            }

            println!(
                "Exporting project at {:?} to {:?}",
                project_path, export_path
            );

            let project = Project::load(&project_path).expect("Couldn't load project");
            let mut runner = Runner::new(project);

            runner.run(Some(export_path));
        }
        None => {
            println!("No command found, valid commands are 'create', 'play', and 'export'");
            println!("Add --help or -h after a command for more information");
        }
    }
}

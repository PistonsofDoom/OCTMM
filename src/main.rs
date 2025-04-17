use crate::{cli::Cli, cli::Commands, project::Project};
use clap::Parser;

mod cli;
mod project;
mod test_utils;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Create(args)) => {
            println!("create: {:?}, {:?}", args.name, args.path);
            println!("unimplemented");
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

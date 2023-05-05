use std::path::PathBuf;

use clap::{Parser, Subcommand};
use omnirepo_lib::{
    clone::operations::clone_repo,
    config::{manager::GlobalConfigManager, parser::Repositories},
    run::operations::run_command,
    util::operations::{load_config_default, load_config},
};

#[derive(Debug, Parser)]
#[clap(name = "omnirepo", version = "0.1.0", author = "")]
#[command(about = "A tool for managing multiple git repositories", long_about = None)]
struct Cli {
    #[arg(
        short,
        long,
        help = "Point to a .omnirepo.yaml or a directory containing config"
    )]
    config: Option<String>,

    #[command(subcommand)]
    command: Commands,
}
#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true, about = "Create a new repository")]
    New {
        #[arg(short, long, help = "The name of the repository")]
        name: String,
    },
    #[command(
        arg_required_else_help = true,
        about = "Clone a group of repositories based on tags"
    )]
    Clone {
        #[arg(
            short,
            long,
            use_value_delimiter = true,
            value_delimiter = ',',
            help = "The names of the tags to clone"
        )]
        tags: Vec<String>,

        #[arg(
            short,
            long,
            help = "Destination to clone the repositories, current folder by default"
        )]
        destination: Option<String>,
    },
    #[command(
        arg_required_else_help = true,
        about = "Run a command in each repository"
    )]
    Run {
        #[arg(short, long, help = "The command to run")]
        command: String,
        #[arg(
            short,
            long,
            help = "Destination to clone the repositories, current folder by default"
        )]
        destination: Option<String>,
    },
    #[command(
        arg_required_else_help = true,
        about = "Sync a file across all repositories"
    )]
    Sync {
        #[arg(short, long, help = "The file to sync")]
        file: String,
        #[arg(short, long, help = "Source file for syncing")]
        url: Option<String>,
        #[arg(short, long, help = "Template file for syncing")]
        template_file: Option<String>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let cfg_mgr = match args.config.as_deref() {
        Some(config_location) => load_config(&PathBuf::from(config_location)),
        None => load_config_default()
    }?;

    match args.command {
        Commands::New { name } => {
            println!("Creating new repo: {}", name);
            //new_repo(name);
        }
        Commands::Clone { tags, destination } => {
            println!("Cloning tags: {:?}", tags);
            clone_repo(cfg_mgr, &tags, destination)?;
        }
        Commands::Run {
            command,
            destination,
        } => {
            println!("Running command: {}", command);
            run_command(command, destination)?;
        }
        Commands::Sync {
            file,
            url,
            template_file,
        } => {
            println!("Syncing file: {}", file);
            if url.is_some() {
                println!("Syncing from: {}", url.unwrap());
            }
            if template_file.is_some() {
                println!("Syncing with template: {}", template_file.unwrap());
            }
            // sync_file(file, url, template_file);
        }
    }

    Ok(())
}

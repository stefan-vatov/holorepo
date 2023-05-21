use std::path::PathBuf;

use clap::{Parser, Subcommand};
use omnirepo_lib::{
    clone::repository_clone::clone_repo,
    new::project_creation::new_repo,
    run::runners::run_command,
    util::utilities::{load_config, load_config_default},
};
use log::info;
use fern::Dispatch;

fn setup_logger() -> Result<(), Box<dyn std::error::Error>> {
    let _logger = Dispatch::new()
        .format(|out, message, _| out.finish(format_args!("{}", message)))
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()?;

    Ok(())
}

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

    #[arg(short, long, help = "Log to file")]
    verbose: Option<bool>,

    #[command(subcommand)]
    command: Commands,
}
#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true, about = "Create a new repository")]
    New {
        #[arg(short, long, help = "The name of the repository")]
        name: String,

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
        None => load_config_default(),
    }?;

    match args.verbose {
        Some(true) => setup_logger(),
        _ => Ok(()),
    }?;

    match args.command {
        Commands::New {
            name,
            tags,
            destination,
        } => {
            info!("Creating new repo: {}", &name);
            new_repo(cfg_mgr, &tags, destination, name)?;
        }
        Commands::Clone { tags, destination } => {
            info!("Cloning tags: {:?}", &tags);
            clone_repo(cfg_mgr, &tags, destination)?;
        }
        Commands::Run {
            command,
            destination,
        } => {
            info!("Running command: {}", &command);
            run_command(command, destination)?;
        }
        Commands::Sync {
            file,
            url,
            template_file,
        } => {
            info!("Syncing file: {}", &file);
            if url.is_some() {
                info!("Syncing from: {}", &url.unwrap());
            }
            if template_file.is_some() {
                info!("Syncing with template: {}", &template_file.unwrap());
            }
            // sync_file(file, url, template_file);
        }
    }

    Ok(())
}

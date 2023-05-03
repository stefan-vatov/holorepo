

use clap::{Parser, Subcommand};
use omnirepo_lib::{
    clone::clone::clone_repo,
    config::{manager::GlobalConfigManager, parser::Repositories},
    run::run::run_command,
};

#[derive(Debug, Parser)]
#[clap(name = "omnirepo", version = "0.1.0", author = "")]
#[command(about = "A tool for managing multiple git repositories", long_about = None)]
struct Cli {
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
    #[command(arg_required_else_help = true, about = "Clone a group of repositories")]
    Clone {
        #[arg(short, long, help = "The name of the group to clone")]
        group: String,

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
    let config_file = dirs::home_dir()
        .expect("Could not find home directory")
        .join(".omnirepo");
    let file = std::fs::File::open(config_file)
        .map_err(|e| format!("Could not open config file: {}", e))?;
    let config: Repositories =
        serde_yaml::from_reader(file).map_err(|e| format!("Error parsing YAML file: {}", e))?;
    let cfg_mgr = GlobalConfigManager::new(config.repositories);
    let args = Cli::parse();

    match args.command {
        Commands::New { name } => {
            println!("Creating new repo: {}", name);
            //new_repo(name);
        }
        Commands::Clone { group, destination } => {
            println!("Cloning group: {}", group);
            clone_repo(cfg_mgr, &group, destination)?;
            // let repos = cfg_mgr.get_by_tag(&group);
            // println!("{:?}", repos);
            // clone_group(group);
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

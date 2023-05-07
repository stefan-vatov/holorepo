use crate::config::{manager::RepoConfigManager, parser::RepoConfig};
use duct::cmd;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::{
    io::{Error, ErrorKind},
    process::Output,
    sync::atomic::{AtomicUsize, Ordering},
};

pub fn run_command(
    command_string: String,
    destination: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let dest = destination.unwrap_or(".".into());
    let config_file = format!("{}/.omni.yaml", &dest);
    let file = std::fs::File::open(&config_file).map_err(|e| {
        format!(
            "Could not open local repo config file: {}, {}",
            &config_file, e
        )
    })?;
    let config: RepoConfig = serde_yaml::from_reader(file)
        .map_err(|e| format!("Error parsing repo config YAML file: {}", e))?;

    let rpc = RepoConfigManager::new(config);

    let dirs = rpc.get_dirs();

    let num_tasks = dirs.len();
    let completed_tasks = AtomicUsize::new(0);
    let progress_bar = ProgressBar::new(num_tasks as u64)
        .with_style(
          ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")?
                .progress_chars("#>-"),
        );

    let result: Vec<Result<Output, Error>> = dirs
        .par_iter()
        .map(|dir: &String| {
            let cmd_dir = format!("{}/{}", &dest, &dir);
            let output = cmd("sh", &["-c", &command_string])
                .dir(&cmd_dir)
                .stdout_null()
                .stderr_null()
                .run();

            // Update progress bar
            progress_bar.inc(1);
            let completed = completed_tasks.fetch_add(1, Ordering::Relaxed);
            if completed + 1 == num_tasks {
                progress_bar.finish_with_message("All tasks completed.");
            }

            output.map_err(|e| {
                Error::new(
                    ErrorKind::Other,
                    format!("Error running command: {}, {}", &cmd_dir, e),
                )
            })
        })
        .collect();

    for res in &result {
        match res {
            Ok(_out) => (),
            Err(_e) => (),
        }
    }

    Ok(())
}

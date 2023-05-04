use std::{
    fs,
    io::{Error, ErrorKind},
    process::Output,
    sync::atomic::{AtomicUsize, Ordering},
};

use duct::cmd;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;

use crate::config::{manager::GlobalConfigManager, parser::RepoConfig};

pub fn clone_repo(
    cfg_mgr: GlobalConfigManager,
    tag: &str,
    destination: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let repos = cfg_mgr.get_url_by_tag(tag);
    let dests = cfg_mgr.get_dest_by_tag(tag);

    let dest = destination.unwrap_or(".".into());

    let num_tasks = repos.len();
    let completed_tasks = AtomicUsize::new(0);
    let progress_bar = ProgressBar::new(num_tasks as u64)
        .with_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")?
                .progress_chars("#>-"),
        );

    let clone_result: Vec<Result<Output, Error>> = repos
        .par_iter()
        .map(|repo: &String| {
            let output = cmd!("git", "clone", repo)
                .dir(&dest)
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
                    format!("Error cloning repo: {}, {}", repo.clone(), e),
                )
            })
        })
        .collect();

    for res in &clone_result {
        match res {
            Ok(_out) => (),
            Err(_e) => (),
        }
    }

    let rpc = serde_yaml::to_string(&RepoConfig::new(dests))
        .expect("Failed to serialise local multirepo config.");
    fs::write(format!("{}/.omni.yaml", &dest), rpc)?;

    Ok(())
}

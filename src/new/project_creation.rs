use duct::cmd;
use indicatif::{ProgressBar, ProgressStyle};
use log::info;
use rayon::prelude::*;
use reqwest::blocking;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::{
    config::manager::GlobalConfigManager,
    util::utilities::{filename_from_url, template_and_dest_from_tags, dedupe_vec_string},
};

pub fn new_repo(
    cfg_mgr: GlobalConfigManager,
    tags: Option<Vec<String>>,
    destination: Option<String>,
    name: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let dest = destination.unwrap_or(".".to_string());
    let mut valid_tags = tags.unwrap_or(vec![]);

    info!("Creating new repo: {:?}", &name);
    info!("Creating to: {:?}", &dest);

    let dir_to_create = PathBuf::new().join(dest).join(&name);

    info!("Creating directory: {:?}", &dir_to_create);

    match fs::create_dir(&dir_to_create) {
        Ok(dir) => dir,
        Err(e) => panic!(
            "Unable to create folder for new repository {}, {}",
            &name, e
        ),
    };

    valid_tags.push("default".to_owned());

    let unique_tags = dedupe_vec_string(valid_tags);

    copy_templates(&cfg_mgr, &unique_tags, &dir_to_create);
    info!("Templates copied to {:?}", &dir_to_create);

    init_repo(&dir_to_create);
    info!("Git repo initialized in {:?}", &dir_to_create);

    Ok(())
}

pub fn copy_templates(cfg_mgr: &GlobalConfigManager, tags: &[String], dest: &Path) {
    let template_pairs = template_and_dest_from_tags(tags, cfg_mgr);

    let num_tasks = template_pairs.len();
    let completed_tasks = AtomicUsize::new(0);
    let progress_bar = ProgressBar::new(num_tasks as u64)
        .with_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}").unwrap()
                .progress_chars("#>-"),
        );

    template_pairs
        .par_iter()
        .for_each(|pair: &(String, String)| {
            let body = match blocking::get(&pair.0) {
                Ok(res) => match res.text() {
                    Ok(text) => Some(text),
                    Err(_) => {
                        info!("Failed extracting contents from response of {}", &pair.0);

                        None
                    }
                },
                Err(e) => {
                    info!("Failed fetching {}. {}", &pair.0, e);

                    None
                }
            };

            if let Some(body) = body {
                let file_dir = format!("{}/{}", dest.to_str().unwrap(), &pair.1);

                if fs::create_dir_all(&file_dir).is_ok() {
                    let filename = format!("{}/{}", &file_dir, filename_from_url(&pair.0));

                    match fs::write(&filename, body) {
                        Ok(_) => {
                            info!("File {} has been written", &filename);
                            // Update progress bar
                            progress_bar.inc(1);
                            let completed = completed_tasks.fetch_add(1, Ordering::Relaxed);
                            if completed + 1 == num_tasks {
                                progress_bar.finish_with_message("All tasks completed.");
                            }
                        }
                        Err(e) => {
                            info!("Failed saving file {} to disk. {}", &filename, e);
                        }
                    }
                }
            }
        });
}

pub fn init_repo(dest: &Path) {
    match cmd!("git", "init")
        .dir(dest)
        .stdout_null()
        .stderr_null()
        .run()
    {
        Ok(_output) => {
            println!();
            println!();
            println!("Repository created at {:?}", dest);
        },
        Err(e) => println!("Error initialising repository at {:?}, {:?}", dest, e),
    }
}

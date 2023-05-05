use std::{
    collections::HashSet,
    error::Error,
    path::{Path, PathBuf},
};

use crate::config::{manager::GlobalConfigManager, parser::Repositories};

pub fn get_repos_from_tags(tags: &[String], cfg_mgr: &GlobalConfigManager) -> Vec<String> {
    dedupe_vec(
        tags.iter()
            .flat_map(|tag| cfg_mgr.get_url_by_tag(tag))
            .collect(),
    )
}

pub fn get_dest_from_tags(tags: &[String], cfg_mgr: &GlobalConfigManager) -> Vec<String> {
    dedupe_vec(
        tags.iter()
            .flat_map(|tag| cfg_mgr.get_dest_by_tag(tag))
            .collect(),
    )
}

fn dedupe_vec(combined: Vec<String>) -> Vec<String> {
    let unique: HashSet<String> = combined.into_iter().collect();

    unique.into_iter().collect()
}

pub fn load_config(config_location: &PathBuf) -> Result<GlobalConfigManager, Box<dyn Error>> {
    let config = load_config_from_file(&config_location)?;

    Ok(GlobalConfigManager::new(config.repositories))
}

pub fn load_config_default() -> Result<GlobalConfigManager, Box<dyn Error>> {
    let config_file = dirs::home_dir()
        .expect("Could not find home directory")
        .join(".omnirepo.yaml");

    let config_dir = dirs::home_dir()
        .expect("Could not find home directory")
        .join(".omnirepo/.omnirepo.yaml");

    if Path::new(&config_file).exists() {
        load_config(&config_file)
    } else if Path::new(&config_dir).exists() {
        load_config(&config_dir)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Default config file not found.",
        )))
    }
}

fn load_config_from_file(config_location: &PathBuf) -> Result<Repositories, Box<dyn Error>> {
    let file = std::fs::File::open(config_location)
        .map_err(|e| format!("Could not open config file: {:?} {}", config_location, e))?;
    let config: Repositories =
        serde_yaml::from_reader(file).map_err(|e| format!("Error parsing YAML file: {}", e))?;

    Ok(config)
}

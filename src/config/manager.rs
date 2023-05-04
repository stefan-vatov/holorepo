use super::parser::{RepoConfig, Repository};

pub struct GlobalConfigManager {
    config: Vec<Repository>,
}

impl GlobalConfigManager {
    pub fn new(repos: Vec<Repository>) -> Self {
        Self { config: repos }
    }

    pub fn get_url_by_tag(&self, tag: &str) -> Vec<String> {
        let mut repos: Vec<String> = Vec::new();
        for repo in &self.config {
            if repo.tags.iter().any(|s| s == tag) {
                repos.push(repo.url.clone());
            }
        }
        repos
    }

    pub fn get_dest_by_tag(&self, tag: &str) -> Vec<String> {
        let mut dirs: Vec<String> = Vec::new();
        for repo in &self.config {
            if repo.tags.iter().any(|s| s == tag) {
                dirs.push(repo.dest.clone());
            }
        }
        dirs
    }
}

pub struct RepoConfigManager {
    config: RepoConfig,
}

impl RepoConfigManager {
    pub fn new(config: RepoConfig) -> Self {
        Self { config }
    }

    pub fn get_dirs(&self) -> &[String] {
        &self.config.dirs
    }
}

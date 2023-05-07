use super::parser::{Config, RepoConfig};

pub struct GlobalConfigManager {
    config: Config,
}

impl GlobalConfigManager {
    pub fn new(repos: Config) -> Self {
        Self { config: repos }
    }

    pub fn get_url_by_tag(&self, tag: &str) -> Vec<String> {
        let mut repos: Vec<String> = Vec::new();
        for repo in &self.config.repositories {
            if repo.tags.iter().any(|s| s == tag) {
                repos.push(repo.url.clone());
            }
        }
        repos
    }

    pub fn get_dest_by_tag(&self, tag: &str) -> Vec<String> {
        let mut dirs: Vec<String> = Vec::new();
        for repo in &self.config.repositories {
            if repo.tags.iter().any(|s| s == tag) {
                dirs.push(repo.dest.clone());
            }
        }
        dirs
    }

    pub fn template_and_dest(&self, tag: &str) -> Vec<(String, String)> {
        let mut templ_dest_pairs: Vec<(String, String)> = Vec::new();

        self.config.templates.iter().for_each(|t| {
            if t.tags.iter().any(|s| s == tag) {
                match t.kind {
                    super::parser::TemplateType::File => match &t.dest {
                        Some(dest) => templ_dest_pairs.push((t.url.clone(), dest.clone())),
                        None => (),
                    },
                    super::parser::TemplateType::Dir => {
                        match &t.included_files {
                            Some(included_files) => included_files.iter().for_each(|f| {
                                templ_dest_pairs
                                    .push((format!("{}/{}", &t.url, &f.file_name), f.dest.clone()))
                            }),
                            None => (),
                        };
                    }
                }
            }
        });

        templ_dest_pairs
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

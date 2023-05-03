use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct Repositories {
    pub repositories: Vec<Repository>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Repository {
    pub name: String,
    pub url: String,
    pub tags: Vec<String>,
    pub dest: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct RepoConfig {
    pub dirs: Vec<String>,
}

impl RepoConfig {
    pub fn new(dirs: Vec<String>) -> Self {
        Self { dirs }
    }
}

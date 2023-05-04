use std::collections::HashSet;

use crate::config::manager::GlobalConfigManager;

pub fn get_repos_from_tags(tags: &Vec<String>, cfg_mgr: &GlobalConfigManager) -> Vec<String> {
    dedupe_vec(
        tags.iter()
            .flat_map(|tag| cfg_mgr.get_url_by_tag(tag))
            .collect(),
    )
}

pub fn get_dest_from_tags(tags: &Vec<String>, cfg_mgr: &GlobalConfigManager) -> Vec<String> {
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

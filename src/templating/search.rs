use crate::config::templating::TEMPLATE_CFG_FILENAME;
use crate::itry;
use anyhow::Result;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn search_templates(search_locations: Vec<PathBuf>) -> Result<Vec<PathBuf>> {
    search_locations
        .into_iter()
        .map(|x| WalkDir::new(x).into_iter())
        .flatten()
        .filter_map(|x| {
            let path = itry!(x)
                .clone()
                .path()
                .join(PathBuf::from(TEMPLATE_CFG_FILENAME));

            if path.exists() { Some(Ok(path)) } else { None }
        })
        .collect()
}

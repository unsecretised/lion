use crate::config::global::GlobalTemplatingConfig;
use crate::config::templating::TemplateConfig;
use crate::util::normalise_paths;
use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct TemplateInfo {
    /// The resolved paths to be included (should all be normalised)
    pub paths_included: Vec<PathBuf>,
}

impl TemplateConfig {
    pub fn resolve(self, root: &Path, config: &GlobalTemplatingConfig) -> Result<TemplateInfo> {
        // I *don't* like this implementation, but can't think of a saner one

        // Merge the default exclusions with the exclusions in the config
        let mut exclusions = config.default_exclusions.clone();
        exclusions.extend_from_slice(&self.exclusions);

        let normalised_excls: Vec<Result<PathBuf,_>> = normalise_paths(&exclusions)
            .into_iter()
            .filter(|x| match x {
                Ok(x) => !self.inclusions.contains(x),
                Err(_) => true, // Keep errors in to propagate later
            })
            .collect();

        let inclusions: Result<_> = WalkDir::new(root.to_path_buf())
            .into_iter()
            .map(|x| {
                // Normalise all paths for reliable comparison
                let x = x?.path().to_path_buf();
                return Ok(x.canonicalize()?);
            })
            .filter(|x| {
                // Filter out any that are in the exclusions
                match x {
                    Ok(x) => !normalised_excls.iter().any(|excl| {
                        if let Ok(excl) = excl {
                            x.starts_with(excl.to_path_buf())
                        } else { false }
                    }),
                    Err(_) => true, // Keep errors
                }
            })
            .collect();

        Ok(TemplateInfo {
            paths_included: inclusions?,
        })
    }
}

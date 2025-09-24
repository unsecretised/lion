use anyhow::Result;
use pathdiff::diff_paths;
use serde::de::DeserializeOwned;
use std::path::{Path, PathBuf};

/// Like try, but for iterators that return [`Option<Result<_, _>>`].
///
/// NOTE: ripped out of the walkdir crate
///
/// [`Option<Result<_, _>>`]: https://doc.rust-lang.org/stable/std/option/enum.Option.html
// Unused for now
#[macro_export]
macro_rules! itry {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(err) => return Some(Err(From::from(err))),
        }
    };
}

/// Helper function to load from a path *if* the file exists, and in other cases,
/// call `T::Default` to get a default value.
///
/// This is mainly to make loading configs a bit nicer and less repetitive
pub fn load_toml<T: DeserializeOwned + Default>(path: impl AsRef<Path>) -> Result<T> {
    if path.as_ref().exists() {
        let config = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&config)?)
    } else {
        Ok(T::default())
    }
}

/// Make a set of paths relative to the provided `root` path
///
/// Each item would only be an `Err` if the path isn't a child of the provided
/// root path
pub fn make_paths_relative_to(
    x: &[impl AsRef<Path>],
    root: impl AsRef<Path>,
) -> Vec<std::io::Result<PathBuf>> {
    x.iter()
        .map(|x| {
            let normalised = x.as_ref().canonicalize()?;
            return Ok(diff_paths(
                normalised,
                PathBuf::from(root.as_ref().to_path_buf()).canonicalize()?,
            )
            .unwrap());
        })
        .collect()
}

/// Utility to normalise a set of paths
pub fn normalise_paths(paths: &[impl AsRef<Path>]) -> Vec<std::io::Result<PathBuf>> {
    paths
        .iter()
        .map(|x| x.as_ref().canonicalize())
        .collect()
}

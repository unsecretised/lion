use serde::{Deserialize, Serialize};

/// A configuration for rust in a specific workspace
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct CargoConfig {
    pub run_command: String,
    pub build_command: String,
    pub test_command: String,

    pub new_binary_command: String,
    pub new_library_command: String,

    // Complicated
    pub new_workspace_command: Option<String>,
    // Needs runtime templating (thing for later)
    pub add_dependency_command: Option<String>,
}

impl Default for CargoConfig {
    fn default() -> Self {
        Self {
            run_command: "cargo run".to_string(),
            build_command: "cargo build".to_string(),
            test_command: "cargo test".to_string(),

            new_binary_command: "cargo init --bin".to_string(),
            new_library_command: "cargo init --lib".to_string(),

            new_workspace_command: None,
            add_dependency_command: None,
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Config {
    // methods in `excludes` are preserved
    pub excludes: Vec<String>,
    // files in `excludes_files` are not processed
    pub excludes_files: Vec<String>,
    // output containing any value from `includes_value` is preserved.
    pub includes_value: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            excludes: vec![],
            excludes_files: vec![],
            includes_value: vec![],
        }
    }
}

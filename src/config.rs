use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    // methods in `excludes` are preserved
    excludes: Vec<String>,
    // files in `excludes_files` are not processed
    excludes_files: Vec<String>,
    // output containing any value from `includes_value` is preserved.
    includes_value: Vec<String>,
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

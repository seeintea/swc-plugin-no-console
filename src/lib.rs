pub mod config;
pub mod visitor;

use swc_core::{
    ecma::{
        ast::Program,
        visit::{as_folder, FoldWith},
    },
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

// fn should_ignore_file(filename: &str, patterns: &[String]) -> bool {
//     patterns.iter().any(|pattern| {
//         // Simple pattern matching; you can implement more complex matching logic if needed
//         filename.contains(pattern)
//     })
// }

#[plugin_transform]
fn plugin_no_console(program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    let config: config::Config = metadata
        .get_transform_plugin_config()
        .and_then(|json| serde_json::from_str(&json).ok())
        .unwrap_or_default();

    program.fold_with(&mut as_folder(visitor::TransformVisitor::new(config)))
}

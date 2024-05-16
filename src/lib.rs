pub mod config;
pub mod visitor;

use swc_core::{
    ecma::{
        ast::Program,
        visit::{as_folder, FoldWith},
    },
    plugin::{
        metadata::TransformPluginMetadataContextKind, plugin_transform,
        proxies::TransformPluginProgramMetadata,
    },
};

fn should_ignore_file(metadata: &TransformPluginProgramMetadata, patterns: &[String]) -> bool {
    let filename = metadata.get_context(&TransformPluginMetadataContextKind::Filename);
    if let Some(str) = filename {
        return patterns.iter().any(|pattern| str.contains(pattern))
    }
    false
}

#[plugin_transform]
fn plugin_no_console(program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    let config: config::Config = metadata
        .get_transform_plugin_config()
        .and_then(|json| serde_json::from_str(&json).ok())
        .unwrap_or_default();

    if should_ignore_file(&metadata, &config.excludes_files) {
        return program;
    }

    program.fold_with(&mut as_folder(visitor::TransformVisitor::new(config)))
}

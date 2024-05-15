pub mod transform;
pub mod config;

use swc_core::{
    ecma::{ast::Program, visit::FoldWith},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

#[plugin_transform]
fn plugin_no_console(program: Program, _data: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut transform::no_console_visitor(
        transform::Config::Enable(true),
    ))
}
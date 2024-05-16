use plugin_no_console::config::Config;
use plugin_no_console::visitor::TransformVisitor;

use std::path::PathBuf;

use swc_core::{
    common::{chain, Mark},
    ecma::visit::as_folder,
};
use swc_ecma_parser::{EsConfig, Syntax};
use swc_ecma_transforms_base::resolver;
use swc_ecma_transforms_testing::{test_fixture, FixtureTestConfig};

fn syntax() -> Syntax {
    Syntax::Es(EsConfig {
        jsx: true,
        ..Default::default()
    })
}

#[testing::fixture("tests/fixture/simple/input.js")]
fn fixture_simple(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");
    test_fixture(
        syntax(),
        &|_tr| {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();

            chain!(
                resolver(unresolved_mark, top_level_mark, false),
                as_folder(TransformVisitor::new(Config::default()))
            )
        },
        &input,
        &output,
        FixtureTestConfig {
            ..Default::default()
        },
    );
}

#[testing::fixture("tests/fixture/excludes/input.js")]
fn fixture_exclude(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");
    test_fixture(
        syntax(),
        &|_tr| {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();

            chain!(
                resolver(unresolved_mark, top_level_mark, false),
                as_folder(TransformVisitor::new(Config {
                    excludes: vec!["log".to_string()],
                    ..Config::default()
                }))
            )
        },
        &input,
        &output,
        FixtureTestConfig {
            ..Default::default()
        },
    );
}

#[testing::fixture("tests/fixture/includes_value/input.js")]
fn fixture_includes_value(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");
    test_fixture(
        syntax(),
        &|_tr| {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();

            chain!(
                resolver(unresolved_mark, top_level_mark, false),
                as_folder(TransformVisitor::new(Config {
                    includes_value: vec!["log0".to_string()],
                    ..Config::default()
                }))
            )
        },
        &input,
        &output,
        FixtureTestConfig {
            ..Default::default()
        },
    );
}
use plugin_no_console::transform;

use std::path::PathBuf;

use swc_core::common::{chain, Mark};
use swc_ecma_parser::{EsConfig, Syntax};
use swc_ecma_transforms_base::resolver;
use swc_ecma_transforms_testing::{test_fixture, FixtureTestConfig};

fn syntax() -> Syntax {
    Syntax::Es(EsConfig {
        jsx: true,
        ..Default::default()
    })
}

#[testing::fixture("tests/fixture/**/input.js")]
fn fixture(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");
    test_fixture(
        syntax(),
        &|_tr| {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();

            chain!(
                resolver(unresolved_mark, top_level_mark, false),
                transform::no_console_visitor(transform::Config::Enable(true),)
            )
        },
        &input,
        &output,
        FixtureTestConfig {
            ..Default::default()
        },
    );
}

extern crate colored;
extern crate ptx_builder;

use ptx_builder::error::*;
use ptx_builder::reporter::BuildReporter;

#[test]
fn should_report_in_cargo_style() {
    let original_error: Result<()> = Err(ErrorKind::CommandFailed(
        String::from("some_name"),
        0,
        String::from("some\nmultiline\noutput"),
    ).into());

    let chained_error = original_error.chain_err(|| {
        ErrorKind::BuildFailed(vec![
            String::from("error[E0425]: cannot find function `external_fn` in this scope"),
            String::from(" --> src/lib.rs:6:20"),
            String::from("  |"),
            String::from("6 |     *y.offset(0) = external_fn(*x.offset(0)) * a;"),
            String::from("  |                    ^^^^^^^^^^^ not found in this scope"),
        ])
    });

    let mut reporter = BuildReporter::report(chained_error.unwrap_err());

    assert_eq!(
        reporter.disable_colors().to_string(),
        "[PTX] Unable to build a PTX crate!
[PTX] error[E0425]: cannot find function `external_fn` in this scope
[PTX]  --> src/lib.rs:6:20
[PTX]   |
[PTX] 6 |     *y.offset(0) = external_fn(*x.offset(0)) * a;
[PTX]   |                    ^^^^^^^^^^^ not found in this scope
[PTX]
[PTX] caused by:
[PTX]   Command failed: 'some_name' with code '0' and output:
[PTX]   some
[PTX]   multiline
[PTX]   output"
    );
}

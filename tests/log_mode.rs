/*
 * Test rustscan logging utilities, ensuring library doesn't log to stdout.
 */
use std::{
    io::Read,
    process::{Command, Stdio},
};

// need to import here, otherwise cargo thinks I'm not using the test
mod log_mode_test_binary;

#[test]
fn no_logging_scanner() {
    let mut child = Command::new("target/debug/examples/log_mode_test_binary")
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    child.wait().unwrap();

    let buf = &mut Vec::new();
    child.stdout.take().unwrap().read_to_end(buf).unwrap();
    assert!(buf.is_empty());

    child.stderr.take().unwrap().read_to_end(buf).unwrap();
    assert!(buf.is_empty());
}

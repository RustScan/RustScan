/*
 * Test rustscan against different targets with the accesible flag set.
 * The tests assumes target/debug/rustscan has already been built.
 *
 * The tests are #[ignore] to avoid running them during normal development.
 *
 * Their tests in the accessibility module are run by the GitHub Runner during CI.
 */

use regex::Regex;
use std::process::Command;

fn rustscan_a11y_test(args: &[&str]) {
    println!("Running: target/debug/rustscan: {}", args.join(" "));

    let mut output = Command::new("target/debug/rustscan").args(args).output()?;

    if !output.status.success() {
        error_chain::bail!("Command exec failed");
    }

    let pattern = Regex::new(r"( ?[!~>|{}] ?)",)?;

    if pattern.is_match(output.stdout) {
        panic!("stdout contains non-accessible characters");
    }
}

mod accesstests {

    #[test]
    #[ignore]
    fn scan_localhost() {
        super::run_rustscan_a11y_test(&["--accessible", "--greppable", "--no-nmap", "127.0.0.1"]);
    }

    #[test]
    #[ignore]
    fn scan_google_com() {
        super::run_rustscan_a11y_test(&[
            "--accessible",
            "--greppable",
            "--no-nmap",
            "-u",
            "5000",
            "-b",
            "2500",
            "google.com",
        ]);
    }
}

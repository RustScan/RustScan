/*
 * Test rustscan against different targets with a time limit.
 * The tests assumes target/debug/rustscan has already been built.
 *
 * The tests are #[ignore] to avoid running them during normal development.
 *
 * Their tests in the timelimits module are run by travis during CI.
 */

use std::process::Command;
use std::time::Duration;
use wait_timeout::ChildExt;

fn run_rustscan_with_timeout(args: &[&str], timeout: Duration) {
    println!("Running: target/debug/rustscan: {}", args.join(" "));

    let mut child = Command::new("target/debug/rustscan")
        .args(args)
        .spawn()
        .unwrap();

    let _status_code = match child.wait_timeout(timeout).unwrap() {
        Some(status) => status.code(),
        None => {
            // child hasn't exited yet
            child.kill().unwrap();
            child.wait().unwrap().code();
            panic!("Timeout while running command");
        }
    };
}

mod timelimits {

    #[test]
    #[ignore]
    fn scan_localhost() {
        let timeout = super::Duration::from_secs(3);
        super::run_rustscan_with_timeout(
            &["--greppable", "--no-nmap", "127.0.0.1"],
            timeout,
        );
    }

    #[test]
    #[ignore]
    fn scan_google_com() {
        super::run_rustscan_with_timeout(
            &["--greppable", "--no-nmap", "-u", "5000", "-b", "2500", "google.com"],
            super::Duration::from_secs(28));
    }

    #[test]
    #[ignore]
    fn scan_example_com() {
        super::run_rustscan_with_timeout(
            &["--greppable", "--no-nmap", "-u", "5000", "-b", "2500", "example.com"],
            super::Duration::from_secs(28));
    }

    #[test]
    #[ignore]
    fn scan_rustscan_cmnatic_co_uk() {
        super::run_rustscan_with_timeout(
            &["--greppable", "--no-nmap", "-u", "5000", "-b", "2500", "rustscan.cmnatic.co.uk"],
            super::Duration::from_secs(26));
    }
}

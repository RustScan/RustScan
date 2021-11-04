/*
 * Test rustscan against different targets with the accesible flag set.
 * The tests assumes target/debug/rustscan has already been built.
 *
 * The tests are #[ignore] to avoid running them during normal development.
 *
 * Their tests in the accessibility module are run by the GitHub Runner during CI.
 */

use regex::Regex;
use std::process::{Command, Stdio};

fn rustscan_a11y_test(args: &[&str]) {
    println!("Running: target/debug/rustscan: {}", args.join(" "));

    let output = Command::new("target/debug/rustscan")
        .args(args)
        .stdout(Stdio::piped())
        .output()
        .unwrap();

    let pattern = Regex::new(r"( ?[!~>|{}] ?)").unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    println!("{}", stdout);
    assert!(pattern.is_match(&stdout));    
    // if !pattern.is_match(&stdout) {
    //     println!("pass");
    // }
    // else {panic!("shits broke yo");}
}



mod accesstests {

    #[test]
    #[ignore]
    fn scan_localhost() {
        super::rustscan_a11y_test(&["--accessible", "--greppable", "--no-nmap", "127.0.0.1"]);
    }

    #[test]
    #[ignore]
    fn scan_google_com() {
        super::rustscan_a11y_test(&[
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

    #[test]
    fn scan_example_com() {
        super::rustscan_a11y_test(
            &[
                "--accessible",
                "--greppable",
                "--no-nmap",
                "-u",
                "5000",
                "-b",
                "2500",
                "example.com",
            ],
        );
    }

    #[test]
    #[ignore]    
    fn scan_rustscan_cmnatic_co_uk() {
        super::rustscan_a11y_test(
            &[
                "--accessible",
                "--greppable",
                "--no-nmap",
                "-u",
                "5000",
                "-b",
                "2500",
                "rustscan.cmnatic.co.uk/~tilde",
            ],
        );
    }

}

use serde_derive::Deserialize;
use std::fs;
use structopt::{clap::arg_enum, StructOpt};

const LOWEST_PORT_NUMBER: u16 = 1;
const TOP_PORT_NUMBER: u16 = 65535;

arg_enum! {
    /// Represents the strategy in which the port scanning will run.
    ///   - Serial will run from start to end, for example 1 to 1_000.
    ///   - Random will randomize the order in which ports will be scanned.
    #[derive(Deserialize, Debug, StructOpt, Clone, PartialEq)]
    pub enum ScanOrder {
        Serial,
        Random,
    }
}

/// Represents the range of ports to be scanned.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct PortRange {
    pub start: u16,
    pub end: u16,
}

fn parse_range(input: &str) -> Result<PortRange, String> {
    let range = input
        .split("-")
        .map(|x| x.parse::<u16>())
        .collect::<Result<Vec<u16>, std::num::ParseIntError>>();

    if range.is_err() {
        return Err(String::from(
            "the range format must be 'start-end'. Example: 1-1000.",
        ));
    }

    match range.unwrap().as_slice() {
        [start, end] => Ok(PortRange {
            start: *start,
            end: *end,
        }),
        _ => Err(String::from(
            "the range format must be 'start-end'. Example: 1-1000.",
        )),
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "rustscan", setting = structopt::clap::AppSettings::TrailingVarArg)]
/// Fast Port Scanner built in Rust.
/// WARNING Do not use this program against sensitive infrastructure since the
/// specified server may not be able to handle this many socket connections at once.
/// - Discord https://discord.gg/GFrQsGy
/// - GitHub https://github.com/RustScan/RustScan
pub struct Opts {
    /// A list of comma separated IP addresses or hosts to be scanned.
    #[structopt(use_delimiter = true)]
    pub ips_or_hosts: Vec<String>,

    /// A list of comma separed ports to be scanned. Example: 80,443,8080.
    #[structopt(short, long, use_delimiter = true)]
    pub ports: Option<Vec<u16>>,

    /// A range of ports with format start-end. Example: 1-1000.
    #[structopt(short, long, conflicts_with = "ports", parse(try_from_str = parse_range))]
    pub range: Option<PortRange>,

    /// Whether to ignore the configuration file or not.
    #[structopt(short, long)]
    pub no_config: bool,

    /// Quiet mode. Only output the ports. No Nmap. Useful for grep or outputting to a file.
    #[structopt(short, long)]
    pub quiet: bool,

    /// Accessible mode. Turns off features which negatively affect screen readers.
    #[structopt(long)]
    pub accessible: bool,

    /// Turns off Nmap.
    #[structopt(long)]
    pub no_nmap: bool,

    /// The batch size for port scanning, it increases or slows the speed of
    /// scanning. Depends on the open file limit of your OS.  If you do 65535
    /// it will do every port at the same time. Although, your OS may not
    /// support this.
    #[structopt(short, long, default_value = "4500")]
    pub batch_size: u16,

    /// The timeout in milliseconds before a port is assumed to be closed.
    #[structopt(short, long, default_value = "1500")]
    pub timeout: u32,

    /// Automatically ups the ULIMIT with the value you provided.
    #[structopt(short, long)]
    pub ulimit: Option<rlimit::rlim>,

    /// The order of scanning to be performed. The "serial" option will
    /// scan ports in ascending order while the "random" option will scan
    /// ports randomly.
    #[structopt(long, possible_values = &ScanOrder::variants(), case_insensitive = true, default_value = "serial")]
    pub scan_order: ScanOrder,

    /// The Nmap arguments to run.
    /// To use the argument -A, end RustScan's args with '-- -A'.
    /// Example: 'rustscan -T 1500 127.0.0.1 -- -A -sC'.
    /// This command adds -Pn -vvv -p $PORTS automatically to nmap.
    /// For things like --script '(safe and vuln)' enclose it in quotations marks \"'(safe and vuln)'\"")
    #[structopt(last = true)]
    pub command: Vec<String>,
}

impl Opts {
    pub fn read() -> Self {
        let mut opts = Opts::from_args();

        if opts.ports.is_none() && opts.range.is_none() {
            opts.range = Some(PortRange {
                start: LOWEST_PORT_NUMBER,
                end: TOP_PORT_NUMBER,
            });
        }

        opts
    }

    /// Reads the command line arguments into an Opts struct and merge
    /// values found within the user configuration file.
    pub fn merge(&mut self, config: &Config) {
        if !self.no_config {
            self.merge_required(&config);
            self.merge_optional(&config);
        }
    }

    fn merge_required(&mut self, config: &Config) {
        macro_rules! merge_required {
            ($($field: ident),+) => {
                $(
                    if let Some(e) = &config.$field {
                        self.$field = e.clone();
                    }
                )+
            }
        }

        merge_required!(
            ips_or_hosts,
            quiet,
            accessible,
            batch_size,
            timeout,
            scan_order,
            command
        );
    }

    fn merge_optional(&mut self, config: &Config) {
        macro_rules! merge_optional {
            ($($field: ident),+) => {
                $(
                    if config.$field.is_some() {
                        self.$field = config.$field.clone();
                    }
                )+
            }
        }

        merge_optional!(ports, range, ulimit);
    }
}

/// Struct used to deserialize the options specified within our config file.
/// These will be further merged with our command line arguments in order to
/// generate the final Opts struct.
#[derive(Debug, Deserialize)]
pub struct Config {
    ips_or_hosts: Option<Vec<String>>,
    ports: Option<Vec<u16>>,
    range: Option<PortRange>,
    quiet: Option<bool>,
    accessible: Option<bool>,
    batch_size: Option<u16>,
    timeout: Option<u32>,
    no_nmap: Option<bool>,
    ulimit: Option<rlimit::rlim>,
    scan_order: Option<ScanOrder>,
    command: Option<Vec<String>>,
}

impl Config {
    /// Reads the configuration file with TOML format and parses it into a
    /// Config struct.
    ///
    /// # Format
    ///
    /// ips_or_hosts = ["127.0.0.1", "127.0.0.1"]
    /// ports = [80, 443, 8080]
    /// quiet = true
    /// scan_order: "Serial"
    ///
    pub fn read() -> Self {
        let path = match dirs::home_dir() {
            Some(mut path) => {
                path.push(".rustscan.toml");
                path
            }
            None => panic!("Could not infer config file path."),
        };

        let contents = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(_) => {
                println!("Could not find configation file at {:?}", &path);
                String::new()
            }
        };

        let config: Config = match toml::from_str(&contents) {
            Ok(config) => config,
            Err(e) => {
                println!("Found {} in configuration file.\nAborting scan.\n", e);
                std::process::exit(1);
            }
        };

        config
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, Opts, PortRange, ScanOrder};

    #[test]
    fn opts_no_merge_when_config_is_ignored() {
        let mut opts = Opts {
            ips_or_hosts: vec![],
            ports: None,
            range: None,
            quiet: false,
            batch_size: 0,
            timeout: 0,
            ulimit: None,
            command: vec![],
            accessible: false,
            no_nmap: false,
            scan_order: ScanOrder::Serial,
            no_config: true,
        };

        let config = Config {
            ips_or_hosts: Some(vec!["127.0.0.1".to_owned()]),
            ports: None,
            range: None,
            quiet: Some(true),
            batch_size: Some(25_000),
            timeout: Some(1_000),
            ulimit: None,
            no_nmap: Some(false),
            command: Some(vec!["-A".to_owned()]),
            accessible: Some(true),
            scan_order: Some(ScanOrder::Random),
        };

        opts.merge(&config);

        assert_eq!(opts.ips_or_hosts, vec![] as Vec<String>);
        assert_eq!(opts.quiet, false);
        assert_eq!(opts.accessible, false);
        assert_eq!(opts.timeout, 0);
        assert_eq!(opts.command, vec![] as Vec<String>);
        assert_eq!(opts.scan_order, ScanOrder::Serial);
    }

    #[test]
    fn opts_merge_required_arguments() {
        let mut opts = Opts {
            ips_or_hosts: vec![],
            ports: None,
            range: None,
            quiet: false,
            batch_size: 0,
            timeout: 0,
            ulimit: None,
            command: vec![],
            accessible: false,
            scan_order: ScanOrder::Serial,
            no_nmap: false,
            no_config: false,
        };

        let config = Config {
            ips_or_hosts: Some(vec!["127.0.0.1".to_owned()]),
            ports: None,
            no_nmap: Some(false),
            range: None,
            quiet: Some(true),
            batch_size: Some(25_000),
            timeout: Some(1_000),
            ulimit: None,
            command: Some(vec!["-A".to_owned()]),
            accessible: Some(true),
            scan_order: Some(ScanOrder::Random),
        };

        opts.merge_required(&config);

        assert_eq!(opts.ips_or_hosts, config.ips_or_hosts.unwrap());
        assert_eq!(opts.quiet, config.quiet.unwrap());
        assert_eq!(opts.timeout, config.timeout.unwrap());
        assert_eq!(opts.command, config.command.unwrap());
        assert_eq!(opts.accessible, config.accessible.unwrap());
        assert_eq!(opts.scan_order, config.scan_order.unwrap());
    }

    #[test]
    fn opts_merge_optional_arguments() {
        let mut opts = Opts {
            ips_or_hosts: vec![],
            ports: None,
            range: None,
            quiet: false,
            batch_size: 0,
            timeout: 0,
            ulimit: None,
            command: vec![],
            accessible: false,
            scan_order: ScanOrder::Serial,
            no_nmap: false,
            no_config: false,
        };

        let config = Config {
            ips_or_hosts: None,
            ports: Some(vec![80, 403]),
            range: Some(PortRange {
                start: 1,
                end: 1_000,
            }),
            quiet: None,
            batch_size: None,
            timeout: None,
            no_nmap: Some(false),
            ulimit: Some(1_000),
            command: None,
            accessible: None,
            scan_order: None,
        };

        opts.merge_optional(&config);

        assert_eq!(opts.ports, config.ports);
        assert_eq!(opts.range, config.range);
        assert_eq!(opts.ulimit, config.ulimit);
    }
}

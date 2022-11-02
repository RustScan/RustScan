use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs;
use structopt::{clap::arg_enum, StructOpt};

const LOWEST_PORT_NUMBER: u16 = 1;
const TOP_PORT_NUMBER: u16 = 65535;

arg_enum! {
    /// Represents the strategy in which the port scanning will run.
    ///   - Serial will run from start to end, for example 1 to 1_000.
    ///   - Random will randomize the order in which ports will be scanned.
    #[derive(Deserialize, Debug, StructOpt, Clone, Copy, PartialEq)]
    pub enum ScanOrder {
        Serial,
        Random,
    }
}

arg_enum! {
    /// Represents the scripts variant.
    ///   - none will avoid running any script, only portscan results will be shown.
    ///   - default will run the default embedded nmap script, that's part of RustScan since the beginning.
    ///   - custom will read the ScriptConfig file and the available scripts in the predefined folders
    #[derive(Deserialize, Debug, StructOpt, Clone, PartialEq, Copy)]
    pub enum ScriptsRequired {
        None,
        Default,
        Custom,
    }
}

/// Represents the range of ports to be scanned.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct PortRange {
    pub start: u16,
    pub end: u16,
}

#[cfg(not(tarpaulin_include))]
fn parse_range(input: &str) -> Result<PortRange, String> {
    let range = input
        .split('-')
        .map(str::parse)
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

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "rustscan", setting = structopt::clap::AppSettings::TrailingVarArg)]
#[allow(clippy::struct_excessive_bools)]
/// Fast Port Scanner built in Rust.
/// WARNING Do not use this program against sensitive infrastructure since the
/// specified server may not be able to handle this many socket connections at once.
/// - Discord https://discord.gg/GFrQsGy
/// - GitHub https://github.com/RustScan/RustScan
pub struct Opts {
    /// A list of comma separated CIDRs, IPs, or hosts to be scanned.
    #[structopt(short, long, use_delimiter = true)]
    pub addresses: Vec<String>,

    /// A list of comma separed ports to be scanned. Example: 80,443,8080.
    #[structopt(short, long, use_delimiter = true)]
    pub ports: Option<Vec<u16>>,

    /// A range of ports with format start-end. Example: 1-1000.
    #[structopt(short, long, conflicts_with = "ports", parse(try_from_str = parse_range))]
    pub range: Option<PortRange>,

    /// Whether to ignore the configuration file or not.
    #[structopt(short, long)]
    pub no_config: bool,

    /// Greppable mode. Only output the ports. No Nmap. Useful for grep or outputting to a file.
    #[structopt(short, long)]
    pub greppable: bool,

    /// Accessible mode. Turns off features which negatively affect screen readers.
    #[structopt(long)]
    pub accessible: bool,

    /// The batch size for port scanning, it increases or slows the speed of
    /// scanning. Depends on the open file limit of your OS.  If you do 65535
    /// it will do every port at the same time. Although, your OS may not
    /// support this.
    #[structopt(short, long, default_value = "4500")]
    pub batch_size: u16,

    /// The timeout in milliseconds before a port is assumed to be closed.
    #[structopt(short, long, default_value = "1500")]
    pub timeout: u32,

    /// The number of tries before a port is assumed to be closed.
    /// If set to 0, rustscan will correct it to 1.
    #[structopt(long, default_value = "1")]
    pub tries: u8,

    /// Automatically ups the ULIMIT with the value you provided.
    #[structopt(short, long)]
    pub ulimit: Option<rlimit::RawRlim>,

    /// The order of scanning to be performed. The "serial" option will
    /// scan ports in ascending order while the "random" option will scan
    /// ports randomly.
    #[structopt(long, possible_values = &ScanOrder::variants(), case_insensitive = true, default_value = "serial")]
    pub scan_order: ScanOrder,

    /// Level of scripting required for the run.
    #[structopt(long, possible_values = &ScriptsRequired::variants(), case_insensitive = true, default_value = "default")]
    pub scripts: ScriptsRequired,

    /// Use the top 1000 ports.
    #[structopt(long)]
    pub top: bool,

    /// The Script arguments to run.
    /// To use the argument -A, end RustScan's args with '-- -A'.
    /// Example: 'rustscan -T 1500 -a 127.0.0.1 -- -A -sC'.
    /// This command adds -Pn -vvv -p $PORTS automatically to nmap.
    /// For things like --script '(safe and vuln)' enclose it in quotations marks \"'(safe and vuln)'\"")
    #[structopt(last = true)]
    pub command: Vec<String>,
}

#[cfg(not(tarpaulin_include))]
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
            addresses, greppable, accessible, batch_size, timeout, tries, scan_order, scripts,
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

        // Only use top ports when the user asks for them
        if self.top && config.ports.is_some() {
            let mut ports: Vec<u16> = Vec::with_capacity(config.ports.clone().unwrap().len());
            for entry in config.ports.clone().unwrap().keys() {
                ports.push(entry.parse().unwrap())
            }
            self.ports = Some(ports);
        }

        merge_optional!(range, ulimit);
    }
}

/// Struct used to deserialize the options specified within our config file.
/// These will be further merged with our command line arguments in order to
/// generate the final Opts struct.
#[cfg(not(tarpaulin_include))]
#[derive(Debug, Deserialize)]
pub struct Config {
    addresses: Option<Vec<String>>,
    ports: Option<HashMap<String, u16>>,
    range: Option<PortRange>,
    greppable: Option<bool>,
    accessible: Option<bool>,
    batch_size: Option<u16>,
    timeout: Option<u32>,
    tries: Option<u8>,
    ulimit: Option<rlimit::RawRlim>,
    scan_order: Option<ScanOrder>,
    command: Option<Vec<String>>,
    scripts: Option<ScriptsRequired>,
}

#[cfg(not(tarpaulin_include))]
impl Config {
    /// Reads the configuration file with TOML format and parses it into a
    /// Config struct.
    ///
    /// # Format
    ///
    /// addresses = ["127.0.0.1", "127.0.0.1"]
    /// ports = [80, 443, 8080]
    /// greppable = true
    /// scan_order: "Serial"
    ///
    pub fn read() -> Self {
        let mut home_dir = match dirs::home_dir() {
            Some(dir) => dir,
            None => panic!("Could not infer config file path."),
        };
        home_dir.push(".rustscan.toml");

        let mut content = String::new();
        if home_dir.exists() {
            content = match fs::read_to_string(home_dir) {
                Ok(content) => content,
                Err(_) => String::new(),
            }
        }

        let config: Config = match toml::from_str(&content) {
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
    use super::{Config, Opts, PortRange, ScanOrder, ScriptsRequired};
    impl Config {
        fn default() -> Self {
            Self {
                addresses: Some(vec!["127.0.0.1".to_owned()]),
                ports: None,
                range: None,
                greppable: Some(true),
                batch_size: Some(25_000),
                timeout: Some(1_000),
                tries: Some(1),
                ulimit: None,
                command: Some(vec!["-A".to_owned()]),
                accessible: Some(true),
                scan_order: Some(ScanOrder::Random),
                scripts: None,
            }
        }
    }

    impl Opts {
        pub fn default() -> Self {
            Self {
                addresses: vec![],
                ports: None,
                range: None,
                greppable: true,
                batch_size: 0,
                timeout: 0,
                tries: 0,
                ulimit: None,
                command: vec![],
                accessible: false,
                scan_order: ScanOrder::Serial,
                no_config: true,
                top: false,
                scripts: ScriptsRequired::Default,
            }
        }
    }

    #[test]
    fn opts_no_merge_when_config_is_ignored() {
        let mut opts = Opts::default();
        let config = Config::default();

        opts.merge(&config);

        assert_eq!(opts.addresses, vec![] as Vec<String>);
        assert_eq!(opts.greppable, true);
        assert_eq!(opts.accessible, false);
        assert_eq!(opts.timeout, 0);
        assert_eq!(opts.command, vec![] as Vec<String>);
        assert_eq!(opts.scan_order, ScanOrder::Serial);
    }

    #[test]
    fn opts_merge_required_arguments() {
        let mut opts = Opts::default();
        let config = Config::default();

        opts.merge_required(&config);

        assert_eq!(opts.addresses, config.addresses.unwrap());
        assert_eq!(opts.greppable, config.greppable.unwrap());
        assert_eq!(opts.timeout, config.timeout.unwrap());
        assert_eq!(opts.command, config.command.unwrap());
        assert_eq!(opts.accessible, config.accessible.unwrap());
        assert_eq!(opts.scan_order, config.scan_order.unwrap());
        assert_eq!(opts.scripts, ScriptsRequired::Default)
    }

    #[test]
    fn opts_merge_optional_arguments() {
        let mut opts = Opts::default();
        let mut config = Config::default();
        config.range = Some(PortRange {
            start: 1,
            end: 1_000,
        });
        config.ulimit = Some(1_000);

        opts.merge_optional(&config);

        assert_eq!(opts.range, config.range);
        assert_eq!(opts.ulimit, config.ulimit);
    }
}

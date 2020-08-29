use structopt::{clap::arg_enum, StructOpt};

arg_enum! {
    #[derive(Debug, StructOpt)]
    pub enum ScanOrder {
        Serial,
        Random,
    }
}

#[derive(Debug)]
pub struct PortRange {
    pub start: u16,
    pub end: u16,
}

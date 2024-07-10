use std::collections::BTreeMap;
use std::fs::File;

use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::{env, i32, u16, u8};

pub fn f_btree() -> BTreeMap<Vec<u16>, Vec<u8>> {
    // TODO fix file being in same dir thing
    let current_dir = env::current_dir().expect("cant find curr dir");
    let mut file_path = PathBuf::from(current_dir);
    file_path.push("./nmap-payloads");

    let mut data = String::new();
    let file = File::open(&file_path).expect("File not found.");
    let mut file_buf = BufReader::new(file);
    file_buf
        .read_to_string(&mut data)
        .expect("unable to read file");

    let mut fp_map: BTreeMap<i32, String> = BTreeMap::new();

    let mut count = 0;
    let mut capturing = false;
    let mut curr = String::new();

    for line in data.trim().split("\n") {
        if line.contains("#") || line.is_empty() {
            continue;
        }

        if line.starts_with("udp") {
            if !curr.is_empty() {
                fp_map.insert(count, curr);
                curr = String::new();
            }
            capturing = true;
            count += 1;
        }

        if capturing {
            if !curr.is_empty() {
                curr.push(' ');
            }
            curr.push_str(line);
        }
    }

    let pb_linenr = ports_v(&fp_map);
    let payb_linenr = payloads_v(&fp_map);
    let ppm = port_payload_map(pb_linenr, payb_linenr);

    ppm
}

fn ports_v(fp_map: &BTreeMap<i32, String>) -> BTreeMap<i32, Vec<u16>> {
    let mut pb_linenr: BTreeMap<i32, Vec<u16>> = BTreeMap::new();
    let mut port_list: Vec<u16> = Vec::new();

    for (&line_nr, ports) in fp_map {
        if ports.contains("udp ") {
            let remain = &ports[4..];
            let mut start = remain.split(" ");

            let ports = start.next().unwrap();
            let port_segments: Vec<&str> = ports.split(",").collect();

            for segment in port_segments {
                if segment.contains("-") {
                    let range: Vec<&str> = segment.trim().split("-").collect();
                    let start = range[0].parse::<u16>().unwrap();
                    let end = range[1].parse::<u16>().unwrap();

                    for port in start..end {
                        port_list.push(port);
                    }
                } else if !segment.is_empty() {
                    let port: u16 = segment.parse().unwrap();
                    port_list.push(port);
                }
            }
        }

        pb_linenr.insert(line_nr, port_list.clone());
        port_list.clear();
    }

    pb_linenr
}

fn payloads_v(fp_map: &BTreeMap<i32, String>) -> BTreeMap<i32, Vec<u8>> {
    let mut payb_linenr: BTreeMap<i32, Vec<u8>> = BTreeMap::new();

    for (&line_nr, data) in fp_map {
        if data.contains("\"") {
            let start = data.find("\"").expect("payload opening \" not found");
            let payloads = &data[start + 1..];
            payb_linenr.insert(line_nr, parser(&payloads.trim()));
        }
    }

    payb_linenr
}

// I think this should return a vec of u8 instead then we just insert it everytime
fn parser(payload: &str) -> Vec<u8> {
    let payload = payload.trim_matches('"');
    let mut tmp_str = String::new();
    let mut bytes: Vec<u8> = Vec::new();

    for (idx, char) in payload.chars().enumerate() {
        if char == '\\' && payload.chars().nth(idx + 1) == Some('x') {
            continue;
        } else if char.is_digit(16) {
            tmp_str.push(char);
            if tmp_str.len() == 2 {
                bytes.push(u8::from_str_radix(&tmp_str, 16).unwrap());
                tmp_str.clear();
            }
        }
    }

    bytes
}

fn port_payload_map(
    pb_linenr: BTreeMap<i32, Vec<u16>>,
    payb_linenr: BTreeMap<i32, Vec<u8>>,
) -> BTreeMap<Vec<u16>, Vec<u8>> {
    let mut ppm_fin: BTreeMap<Vec<u16>, Vec<u8>> = BTreeMap::new();

    for (port_linenr, ports) in pb_linenr {
        for (pay_linenr, payloads) in &payb_linenr {
            if pay_linenr == &port_linenr {
                ppm_fin.insert(ports.to_vec(), payloads.to_vec());
            }
        }
    }

    ppm_fin
}

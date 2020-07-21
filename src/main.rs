use async_std::io;
use async_std::net::TcpStream;
use clap::{App, Arg};
use colored::*;
use std::process::Command;
use std::time::Duration;
use std::{
    net::{IpAddr, SocketAddr, Shutdown},
    str::FromStr, io::ErrorKind,
};
use async_std::prelude::*;
use futures::stream::FuturesUnordered;
use futures::executor::block_on;
use std::process;
/// Faster Nmap scanning with Rust
fn main() {
    // NMAP top 1k ports
    // let NMAP_1000 = vec!(1,3,4,6,7,9,13,17,19,20,21,22,23,24,25,26,30,32,33,37,42,43,49,53,70,79,80,81,82,83,84,85,88,89,90,99,100,106,109,110,111,113,119,125,135,139,143,144,146,161,163,179,199,211,212,222,254,255,256,259,264,280,301,306,311,340,366,389,406,407,416,417,425,427,443,444,445,458,464,465,481,497,500,512,513,514,515,524,541,543,544,545,548,554,555,563,587,593,616,617,625,631,636,646,648,666,667,668,683,687,691,700,705,711,714,720,722,726,749,765,777,783,787,800,801,808,843,873,880,888,898,900,901,902,903,911,912,981,987,990,992,993,995,999,1000,1001,1002,1007,1009,1010,1011,1021,1022,1023,1024,1025,1026,1027,1028,1029,1030,1031,1032,1033,1034,1035,1036,1037,1038,1039,1040,1041,1042,1043,1044,1045,1046,1047,1048,1049,1050,1051,1052,1053,1054,1055,1056,1057,1058,1059,1060,1061,1062,1063,1064,1065,1066,1067,1068,1069,1070,1071,1072,1073,1074,1075,1076,1077,1078,1079,1080,1081,1082,1083,1084,1085,1086,1087,1088,1089,1090,1091,1092,1093,1094,1095,1096,1097,1098,1099,1100,1102,1104,1105,1106,1107,1108,1110,1111,1112,1113,1114,1117,1119,1121,1122,1123,1124,1126,1130,1131,1132,1137,1138,1141,1145,1147,1148,1149,1151,1152,1154,1163,1164,1165,1166,1169,1174,1175,1183,1185,1186,1187,1192,1198,1199,1201,1213,1216,1217,1218,1233,1234,1236,1244,1247,1248,1259,1271,1272,1277,1287,1296,1300,1301,1309,1310,1311,1322,1328,1334,1352,1417,1433,1434,1443,1455,1461,1494,1500,1501,1503,1521,1524,1533,1556,1580,1583,1594,1600,1641,1658,1666,1687,1688,1700,1717,1718,1719,1720,1721,1723,1755,1761,1782,1783,1801,1805,1812,1839,1840,1862,1863,1864,1875,1900,1914,1935,1947,1971,1972,1974,1984,1998,1999,2000,2001,2002,2003,2004,2005,2006,2007,2008,2009,2010,2013,2020,2021,2022,2030,2033,2034,2035,2038,2040,2041,2042,2043,2045,2046,2047,2048,2049,2065,2068,2099,2100,2103,2105,2106,2107,2111,2119,2121,2126,2135,2144,2160,2161,2170,2179,2190,2191,2196,2200,2222,2251,2260,2288,2301,2323,2366,2381,2382,2383,2393,2394,2399,2401,2492,2500,2522,2525,2557,2601,2602,2604,2605,2607,2608,2638,2701,2702,2710,2717,2718,2725,2800,2809,2811,2869,2875,2909,2910,2920,2967,2968,2998,3000,3001,3003,3005,3006,3007,3011,3013,3017,3030,3031,3052,3071,3077,3128,3168,3211,3221,3260,3261,3268,3269,3283,3300,3301,3306,3322,3323,3324,3325,3333,3351,3367,3369,3370,3371,3372,3389,3390,3404,3476,3493,3517,3527,3546,3551,3580,3659,3689,3690,3703,3737,3766,3784,3800,3801,3809,3814,3826,3827,3828,3851,3869,3871,3878,3880,3889,3905,3914,3918,3920,3945,3971,3986,3995,3998,4000,4001,4002,4003,4004,4005,4006,4045,4111,4125,4126,4129,4224,4242,4279,4321,4343,4443,4444,4445,4446,4449,4550,4567,4662,4848,4899,4900,4998,5000,5001,5002,5003,5004,5009,5030,5033,5050,5051,5054,5060,5061,5080,5087,5100,5101,5102,5120,5190,5200,5214,5221,5222,5225,5226,5269,5280,5298,5357,5405,5414,5431,5432,5440,5500,5510,5544,5550,5555,5560,5566,5631,5633,5666,5678,5679,5718,5730,5800,5801,5802,5810,5811,5815,5822,5825,5850,5859,5862,5877,5900,5901,5902,5903,5904,5906,5907,5910,5911,5915,5922,5925,5950,5952,5959,5960,5961,5962,5963,5987,5988,5989,5998,5999,6000,6001,6002,6003,6004,6005,6006,6007,6009,6025,6059,6100,6101,6106,6112,6123,6129,6156,6346,6389,6502,6510,6543,6547,6565,6566,6567,6580,6646,6666,6667,6668,6669,6689,6692,6699,6779,6788,6789,6792,6839,6881,6901,6969,7000,7001,7002,7004,7007,7019,7025,7070,7100,7103,7106,7200,7201,7402,7435,7443,7496,7512,7625,7627,7676,7741,7777,7778,7800,7911,7920,7921,7937,7938,7999,8000,8001,8002,8007,8008,8009,8010,8011,8021,8022,8031,8042,8045,8080,8081,8082,8083,8084,8085,8086,8087,8088,8089,8090,8093,8099,8100,8180,8181,8192,8193,8194,8200,8222,8254,8290,8291,8292,8300,8333,8383,8400,8402,8443,8500,8600,8649,8651,8652,8654,8701,8800,8873,8888,8899,8994,9000,9001,9002,9003,9009,9010,9011,9040,9050,9071,9080,9081,9090,9091,9099,9100,9101,9102,9103,9110,9111,9200,9207,9220,9290,9415,9418,9485,9500,9502,9503,9535,9575,9593,9594,9595,9618,9666,9876,9877,9878,9898,9900,9917,9929,9943,9944,9968,9998,9999,10000,10001,10002,10003,10004,10009,10010,10012,10024,10025,10082,10180,10215,10243,10566,10616,10617,10621,10626,10628,10629,10778,11110,11111,11967,12000,12174,12265,12345,13456,13722,13782,13783,14000,14238,14441,14442,15000,15002,15003,15004,15660,15742,16000,16001,16012,16016,16018,16080,16113,16992,16993,17877,17988,18040,18101,18988,19101,19283,19315,19350,19780,19801,19842,20000,20005,20031,20221,20222,20828,21571,22939,23502,24444,24800,25734,25735,26214,27000,27352,27353,27355,27356,27715,28201,30000,30718,30951,31038,31337,32768,32769,32770,32771,32772,32773,32774,32775,32776,32777,32778,32779,32780,32781,32782,32783,32784,32785,33354,33899,34571,34572,34573,35500,38292,40193,40911,41511,42510,44176,44442,44443,44501,45100,48080,49152,49153,49154,49155,49156,49157,49158,49159,49160,49161,49163,49165,49167,49175,49176,49400,49999,50000,50001,50002,50003,50006,50300,50389,50500,50636,50800,51103,51493,52673,52822,52848,52869,54045,54328,55055,55056,55555,55600,56737,56738,57294,57797,58080,60020,60443,61532,61900,62078,63331,64623,64680,65000,65129,65389);

    let matches = App::new("RustScan")
        .author("Bee https://github.com/brandonskerritt")
        .about("Fast Port Scanner built in Rust")
        .version("0.01")
        // IP address is a required argument
        .arg(Arg::with_name("i")
            .required(true)
            .index(1)
            .long("ip-address")
            .help("The IP address to scan"))
        .arg(Arg::with_name("t")
            .short("t")
            .long("threads")
            .takes_value(true)
            .default_value("250")
            .help("How many threads do you want to use? Default 1000"))
        .arg(Arg::with_name("T")
            .short("T")
            .long("timeout")
            .takes_value(true)
            .default_value("250")
            .help("The timeout before a port is assumed to be close. Default 250, but can go up to 1000."))
        /*.arg(Arg::with_name("n")
            .short("n")
            .long("nmap-1000")
            .takes_value(true)
            .help("Use the nmap top 1k ports"))*/
        .get_matches();

    print_opening();

    let ip = matches.value_of("i").unwrap_or("None");
    let threads_str: &str = matches.value_of("t").unwrap_or("None");
    // let nmap_arg = matches.value_of("n").unwrap_or("None");
    let threads: usize = threads_str.parse::<usize>().unwrap();
    // gets timeout duration
    let duration_timeout = Duration::from_millis(
        matches
            .value_of("T")
            .unwrap_or("None")
            .parse::<u64>()
            .unwrap(),
    );
    // let ports = if !(nmap_arg == "None") { NMAP_1000.iter() } else {(1..65536)};

    // validates the IP address and turns it into an IpAddr type
    let addr = IpAddr::from_str(&ip).expect("IPADDR must be a valid IPv4 or IPv6 address");

    // increases thread pool

    // collect_into_vec is faster than into_vec
    let mut ports_full: Vec<bool> = vec![false; 65536];

    // performs the scan using rayon
    // 65535 + 1 because of 0 indexing
    // TODO let the user decide max port number
    let test = run_batched("192.168.0.1".to_string(), 1, 65535, Duration::from_millis(1500),  8500);
    let reports_fullsult = block_on(test);
    println!("{:?}", reports_fullsult);
    panic!("ending");

    // prints ports and places them into nmap string
    let mut nmap_str_ports = Vec::new();

    // makes vector of open ports
    for (i, item) in ports_full.iter().enumerate() {
        if item == &true {
            // appends it to port
            nmap_str_ports.push((i + 1).to_string());
        }
    }

    // if no ports are found, suggest running with less threads
    /*
    22/tcp   open  ssh     syn-ack
    53/tcp   open  domain  syn-ack
    80/tcp   open  http    syn-ack
    1900/tcp open  upnp    syn-ack*/
    if nmap_str_ports.is_empty() {
        panic!("{} Looks like I didn't find any open ports. This is usually caused by too many threads. \n*I used {} threads, consider lowering to {} with {} or a comfortable number lfor your system. \n Alternatively, increase the timeout if your ping is high. Rustscan -T 1500 for 1.5 second timeout.", "ERROR".red(), threads_str, (threads / 2).to_string().green(), "'rustscan -t <thread_nums> <ip address>'".green());
    }

    // Tells the user we are now switching to Nmap
    println!(
        "{} -A -p {:?} -vvv {}",
        "Starting nmap".blue(),
        nmap_str_ports,
        ip
    );

    // nmap port style is 80,443. Comma seperated with no spaces.
    let ports_str = nmap_str_ports.join(",");

    // Runs the nmap command and spawns it as a process.
    Command::new("nmap")
        .arg("-A")
        .arg("-Pn")
        .arg("-sV")
        .arg("-p")
        .arg(ports_str)
        .arg("-vvv")
        .arg(ip)
        .spawn()
        .expect("failed to execute process");
}
/*
    let string_list = vec![addr.to_string(), port.to_string()].join(":");
    let server: SocketAddr = string_list.parse().expect("Unable to parse socket address");

    */

pub async fn run_batched(
    host: String,
    port_start: u32,
    port_end: u32,
    timeout: Duration,
    batch: u32,
) -> Vec<SocketAddr> {
    // run the scans in batches
    let mut begin = port_start;
    let mut end = begin + batch;
    let mut all_addrs = Vec::new();

    while end <= port_end {
        let mut batch_addrs = execute(host.clone(), begin, end, timeout).await;
        all_addrs.append(&mut batch_addrs);
        begin = end+1;
        end += batch;
    }
    all_addrs
}
async fn execute(
    host: String,
    port_start: u32,
    port_end: u32,
    timeout: Duration,
) -> Vec<SocketAddr> {
    // runs a scan against a range of ports
    let mut ftrs = FuturesUnordered::new();
    for port in port_start..port_end {
        ftrs.push(try_connect(host.clone(), port, timeout));
    }

    let mut open_addrs: Vec<SocketAddr> = Vec::new();
    while let Some(result) = ftrs.next().await {
        match result {
            Ok(addr) => open_addrs.push(addr),
            Err(_) => {}
        }
    }
    open_addrs
}

async fn try_connect(host: String, port: u32, timeout: Duration) -> io::Result<SocketAddr> {
    let addr = host.to_string() + ":" + &port.to_string();
    match addr.parse() {
        Ok(sock_addr) => match connect(sock_addr, timeout).await {
            Ok(stream_result) => {
                match stream_result.shutdown(Shutdown::Both) {
                    _ => {}
                }
                println!("Sock is open {}", sock_addr);
                Ok(sock_addr)
            }
            Err(e) => match e.kind() {
                ErrorKind::Other => {
                    eprintln!("{:?}", e); // in case we get too many open files
                    Err(e)
                }
                _ => Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
            },
        },
        Err(e) => {
            eprintln!("Unable to convert to socket address {:?}", e);
            Err(io::Error::new(io::ErrorKind::Other, e.to_string()))
        }
    }
}


async fn connect(addr: SocketAddr, timeout: Duration) -> io::Result<TcpStream> {
    let stream = io::timeout(timeout, async move { TcpStream::connect(addr).await }).await?;
    Ok(stream)
}

fn print_opening() {
    let s = "
     _____           _    _____                 
    |  __ \\         | |  / ____|                
    | |__) |   _ ___| |_| (___   ___ __ _ _ __  
    |  _  / | | / __| __|\\___ \\ / __/ _` | '_ \\ 
    | | \\ \\ |_| \\__ \\ |_ ____) | (_| (_| | | | |
    |_|  \\_\\__,_|___/\\__|_____/ \\___\\__,_|_| |_|
    Faster nmap scanning with rust.";
    println!(
        "{} \n {} \n {}",
        s.green(),
        "Automated Decryption Tool - https://github.com/ciphey/ciphey".red(),
        "Creator https://github.com/brandonskerritt".green()
    );
}

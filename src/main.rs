mod utils;

// use std::net::{Ipv4Addr, IpAddr};
use std::time::Duration;
use std::path::Path;
use std::fs;
use clap::{AppSettings, Clap};
use rand::random;
use ping;
// use anyhow::{Result, anyhow};
// use std::fmt::Display;

/// this is a tool for ip ping batch testing
#[derive(Clap)]
#[clap(version = "1.0", author = "zzlion. <lixiong_zz@163.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// concurrent num
    #[clap(short, long, default_value = "4")]
    thread_num: u8,
    /// ip separate with ip or one ip file per line
    ips: String,
}

fn check_ip(ip: &str) {
    // fn to_anyhow(e: impl Display) -> anyhow::Error {
    //     anyhow!(e.to_string())
    // }
    // let ip = Ipv4Addr::from_str(ip).unwrap();
    let ret = ping::ping(ip.parse().unwrap(), Some(Duration::from_secs(1)), Some(166), Some(3), Some(5), Some(&random()));
    // .map_err(to_anyhow);
    match ret {
        Ok(_) => println!("{} yes", ip),
        Err(_) => println!("{} no", ip)
    }
}

fn check_is_file(file_name: &str) -> bool {
    let file_path = format!("./{}", file_name);
    let path = Path::new(file_path.as_str());
    let parent = path.parent().and_then(|p| p.is_dir().then(|| p));
    match parent {
        Some(_) => true,
        None => false,
    }
}

fn read_file(ips: &str) -> Vec<String> {
    let file_path = format!("./{}", ips);
    // let file_name= file_name.as_str();
    // let file_path = Path::new(file_name);
    let result = fs::read_to_string(file_path).unwrap();
    result.split("\r\n").map(|s| s.to_string()).collect()
}

fn parse_ip(ips: &str) -> Vec<String> {
    if check_is_file(ips) {
        read_file(ips)
    } else {
        ips.split(",").map(|s| s.to_string()).collect()
    }
}


fn main() {
    // let opts: Opts = Opts::parse();
    // let a = "220.181.38.148";
    // check_ip(a);
    // let file_name = "text";
    // let ret = check_is_file(file_name);
    // println!("ret is {}", ret);

    let vec = parse_ip("test");
    println!("{:?}", vec);
}
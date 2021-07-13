// use std::net::{Ipv4Addr, IpAddr};
use std::time::Duration;
use std::path::;
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
    os.

}

fn parse_ip(ips: &str) -> Vec<T>{

}


fn main() {
    let opts: Opts = Opts::parse();
    let a = "220.181.38.148";
    check_ip(a);
}
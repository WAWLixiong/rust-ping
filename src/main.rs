mod utils;

use std::time::Duration;
use std::path::Path;
use std::fs;
use clap::{AppSettings, Clap};
use rand::random;
use ping;

/// this is a tool for ip ping batch testing
#[derive(Clap)]
#[clap(version = "1.0", author = "zzlion. <lixiong_zz@163.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// concurrent num
    #[clap(short, long, default_value = "4")]
    thread_num: usize,
    /// ip separate with ip or one ip file per line
    ips: String,
}

fn ping_ip(ip: &str){
    let ret = ping::ping(ip.parse().unwrap(), Some(Duration::from_secs(1)), Some(166), Some(3), Some(5), Some(&random()));
    match ret {
        Ok(_) => println!("{} yes", ip),
        Err(_) => println!("{} no", ip)
    }
    // todo: 在线程池内检测时结果会不准确， 需要研究下ping的机制
}

fn check_is_file(file_name: &str) -> bool {
    let file_path = format!("./{}", file_name);
    Path::new(file_path.as_str()).exists()
}

fn read_file(ips: &str) -> Vec<String> {
    let file_path = format!("./{}", ips);
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

fn parse_thread_num(thread_nums: usize, ip_nums: usize) -> usize {
    let mut nums = thread_nums;
    if ip_nums < thread_nums {
        nums = ip_nums
    };
    if nums < 1 {
        nums = 1
    };
    if nums > 20 {
        nums = 20
    }
    eprintln!("Concurrent num is {}", nums);
    nums
}

fn main() {
    let opts: Opts = Opts::parse();
    let ips = opts.ips;
    let ips = parse_ip(&ips);
    let ip_nums = ips.len();
    let size = parse_thread_num(opts.thread_num, ip_nums);
    let threadpool = utils::threadpool::ThreadPool::new(size);

    for ip in ips.into_iter() {
        threadpool.execute(move || ping_ip(ip.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        ping_ip("11.11.11.11");
    }
}

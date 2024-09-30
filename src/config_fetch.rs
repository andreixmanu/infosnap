use std::fmt::Display;
use serde::Deserialize;
use std::fs;
use toml;
use serde_derive;
use crate::data::OsInfo;
use std::io::{Write};

#[derive(Deserialize, Debug)]
struct Config{
    title: String,
    info: Info
}

#[derive(Deserialize, Debug)]
struct Info
{
    kernel: bool,
    hostname: bool,
    type_: bool,
    uptime: bool,
    cpu: bool,
    gpu: bool,
    local_ip: bool,
    used_memory: bool,
    total_memory: bool,
}

impl Display for Info
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "Kernel: {}\n", self.kernel)?;
        write!(f, "Hostname: {}\n", self.hostname)?;
        write!(f, "Type: {}\n", self.type_)?;
        write!(f, "Uptime: {}\n", self.uptime)?;
        write!(f, "CPU: {}\n", self.cpu)?;
        write!(f, "GPU: {}\n", self.gpu)?;
        write!(f, "Local IP: {}\n", self.local_ip)?;
        //write!(f, "Public IP: {}\n", self.public_ip)?;
        write!(f, "Used Memory: {}MB\n", self.used_memory)?;
        write!(f, "Total Memory: {}MB\n", self.total_memory)
    }
}

impl Iterator for Info
{
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.kernel {
            self.kernel = false;
            return Some("kernel");
        }
        if self.hostname {
            self.hostname = false;
            return Some("hostname");
        }
        if self.type_ {
            self.type_ = false;
            return Some("type");
        }
        if self.uptime {
            self.uptime = false;
            return Some("uptime");
        }
        if self.cpu {
            self.cpu = false;
            return Some("cpu");
        }
        if self.gpu {
            self.gpu = false;
            return Some("gpu");
        }
        if self.local_ip {
            self.local_ip = false;
            return Some("local_ip");
        }
        // if self.public_ip {
        //     self.public_ip = false;
        //     return Some("public_ip");
        // }
        if self.used_memory {
            self.used_memory = false;
            return Some("used_memory");
        }
        if self.total_memory {
            self.total_memory = false;
            return Some("total_memory");
        }
        None
    }
}

pub fn print_config(os_info: OsInfo) {
    let mut config = fetch_config();

    for field in config.info{
        match field {
            "kernel" => println!("Kernel: {}", os_info.kernel),
            "hostname" => println!("Hostname: {}", os_info.hostname),
            "type" => println!("Type: {}", os_info.type_),
            "uptime" => println!("Uptime: {}", os_info.uptime),
            "cpu" => println!("CPU: {}", os_info.cpu),
            "gpu" => println!("GPU: {}", os_info.gpu),
            "local_ip" => println!("Local IP: {}", os_info.local_ip),
            "used_memory" => println!("Used Memory: {}MB", os_info.used_memory),
            "total_memory" => println!("Total Memory: {}MB", os_info.total_memory),
            _ => { println!("Unknown field: {}", field); }
        }
    }
}

fn fetch_config() -> Config
{
    let toml_str = fs::read_to_string("config.toml").expect("Failed to read TOML file");
    let config: Config = toml::from_str(&toml_str).expect("Failed to parse TOML");
    config
}


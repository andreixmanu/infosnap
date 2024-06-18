use std::error::Error;
use std::fmt::Display;
use serde::Deserialize;
use std::fs;
use toml;
use serde_derive;
use crate::data::OsInfo;
use crossterm::{
    ExecutableCommand,
    cursor::{MoveTo, Hide, Show},
    terminal::{Clear, ClearType},
    style::{Print, PrintStyledContent, Stylize},
};
use std::io::{stdout, Write};

#[derive(Deserialize, Debug)]
struct Config
{
    title : String,
    info : Info,
    ascii : Ascii
}
impl Display for Config
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        //write!(f, "Title: {}\n", self.title)?;
        write!(f, "Info: {}\n", self.info)?;
        write!(f, "Ascii: {}", self.ascii)
    }
}

#[derive(Deserialize, Debug)]
struct Info
{
    kernel: bool,
    hostname: bool,
    type_: bool,
    uptime: bool,
    cpu: bool,
    // pub cpu_freq : bool,
    gpu: bool,
    local_ip: bool,
    // public_ip: bool,
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
        write!(f, "Used Memory: {}\n", self.used_memory)?;
        write!(f, "Total Memory: {}\n", self.total_memory)
    }
}


#[derive(Deserialize, Debug)]
struct Ascii
{
    distro : String,
    size : String
}
impl Display for Ascii
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "Distro: {}\n", self.distro)?;
        write!(f, "Size: {}\n", self.size)
    }
}

pub fn print_config(os_info: OsInfo)
{
    let config = fetch_config();

    // associate each field with its corresponding value
    let info_fields = [
        (&config.info.kernel, os_info.kernel),
        (&config.info.hostname, os_info.hostname),
        (&config.info.type_, os_info.type_),
        (&config.info.uptime, os_info.uptime.to_string()),
        (&config.info.cpu, os_info.cpu),
        (&config.info.gpu, os_info.gpu),
        (&config.info.local_ip, os_info.local_ip),
        (&config.info.used_memory, os_info.used_memory.to_string()),
        (&config.info.total_memory, os_info.total_memory.to_string()),
    ];

    let ascii = &config.ascii;

    for (field, os_info) in &info_fields {
        if **field {
            println!("{}", os_info);
        }
    }
}

fn fetch_config() -> Config
{
    let toml_str = fs::read_to_string("config.toml").expect("Failed to read TOML file");
    let config: Config = toml::from_str(&toml_str).expect("Failed to parse TOML");
    return config;
}


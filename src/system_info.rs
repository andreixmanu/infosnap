// extern crate sys_info;
extern crate sysinfo;
extern crate os_info;
extern crate local_ip_address;
extern crate public_ip_address;

use local_ip_address::local_ip;
use std::process::Command;
use std::fs;
use std::fmt::Display;
use Vec;
use sys_info::hostname;
use sysinfo::System;
use crate::data::*; // used for uptime

pub fn get_uptime() -> Uptime
{
    let mut uptime : Uptime = Uptime // initialize to 0
    {
        days: 0,
        hours: 0,
        minutes: 0,
        seconds: 0,
    };

    let info = System::uptime();   // returns seconds
    uptime.days = info / 86400;
    uptime.hours = info / 3600;
    uptime.minutes = (info % 3600) / 60;
    uptime.seconds = info % 60;

    return uptime;
}

fn get_distro_name() -> String
{
    let os_release_content = fs::read_to_string("/etc/os-release").expect("Failed to read /etc/os-release");
    let os_release_split = os_release_content.split("\n");
    for line in os_release_split
    {
        if line.starts_with("PRETTY_NAME")
        {
            let pretty_name = line.split("=").collect::<Vec<&str>>()[1];
            let pretty_name = pretty_name.replace("\"", ""); // remove quotes
            return pretty_name.to_string();
        }
    }
    String::new() // return an empty string if no match is found
}

fn get_cpu_info () -> String
{
    let cpu_info = fs::read_to_string("/proc/cpuinfo").expect("Failed to read /proc/cpuinfo");
    let cpu_info_split = cpu_info.split("\n");
    for line in cpu_info_split
    {
        if line.starts_with("model name")
        {
            let model_name = line.split(":").collect::<Vec<&str>>()[1];
            // remove leading whitespace
            let model_name = model_name.trim();
            return model_name.to_string();
        }
    }
    String::new() // return an empty string if no match is found
}

fn get_kernel_version() -> String
{
    let output = Command::new("uname")
        .arg("-r")
        .output()
        .expect("Failed to get kernel version");
    let kernel_version = String::from_utf8(output.stdout).expect("Failed to convert kernel version to string");
    return kernel_version.trim().to_string();
}

fn get_gpu_info() -> String
{
    // execute command lspci -nnk | grep VGA and take everything after "]: "
    let output = Command::new("lspci")
        .arg("-nnk")
        .output()
        .expect("Failed to get GPU info");
    let gpu_info = String::from_utf8(output.stdout).expect("Failed to convert GPU info to string");
    let gpu_info_split = gpu_info.split("\n");
    for line in gpu_info_split
    {
        if line.contains("VGA")
        {
            let gpu_info = line.split("]: ").collect::<Vec<&str>>()[1];
            return gpu_info.to_string();
        }
    }
    return String::new(); // return an empty string if no match is found
}



pub fn get_os_info() -> OsInfo
{
    let mut sys = System::new_all();
    sys.refresh_all();

    let res = OsInfo
    {
        kernel: get_kernel_version(),
        hostname : hostname().expect("Failed to get hostname"),
        type_ : get_distro_name(),
        uptime : get_uptime(),
        cpu : get_cpu_info(),
        gpu : get_gpu_info(),
        local_ip : local_ip().expect("Failed to get local IP").to_string(),
        total_memory : (sys.total_memory() / 1048576) as f32, // bytes -> MB
        used_memory : (sys.used_memory() / 1048576) as f32, // bytes -> MB
    };
    return res;
}

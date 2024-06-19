extern crate colored;
use crate::system_info::{ get_os_info};

// mod ascii;
mod system_info;
mod image;
mod data;
mod config_fetch;
mod ascii;


fn main() {
    let os_info = get_os_info();
    let ascii_art = ascii::ascii_art::arch_small();
    print!("{}", ascii_art);
    config_fetch::print_config(os_info);
}

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
    //print!("{}", ascii_art);
    config_fetch::print_config(os_info);
}

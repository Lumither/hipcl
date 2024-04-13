use std::env;
#[cfg(unix)]
use std::os::unix::net::SocketAddr;

#[cfg(windows)]
use uds_windows::SocketAddr;

use crate::ems::load::get_entity_list;

mod ems;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config_path = match args.get(1) {
        None => "./config.toml",
        Some(config_path) => config_path,
    };

    let socket_addr = match args.get(2) {
        None => SocketAddr::from_pathname("./test.socket"),
        Some(socket_addr) => SocketAddr::from_pathname(socket_addr),
    };

    let entity_handlers =
        get_entity_list(config_path, socket_addr.expect("failed to build socket"));
}

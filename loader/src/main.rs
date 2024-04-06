use std::env;

use crate::ems::load::get_entity_list;

mod ems;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config_path = match args.get(1) {
        None => "./config.toml",
        Some(config_path) => config_path,
    };

    let entity_handlers = get_entity_list(config_path);
}

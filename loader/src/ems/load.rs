use std::fs;

use crate::ems::Entity;

pub fn get_entity_list(config_path: &str) -> Vec<Entity> {
    // parse toml file
    let config_file = fs::read_to_string(config_path)
        .unwrap_or_else(|_| panic!("failed to open the file: {}", config_path));
    let config_file: toml::Value = config_file
        .parse()
        .unwrap_or_else(|_| panic!("invalid toml format file: {}", config_path));

    let entity_list: Vec<String> = if let Some(entities) = config_file.get("entity") {
        if let Some(entity_list) = entities["list"].as_array() {
            entity_list
                .iter()
                .map(|entity| entity.as_str().unwrap().to_string())
                .collect()
        } else {
            vec![entities["list"].as_str().unwrap().to_string()]
        }
    } else {
        panic!("`entity`::list field not found")
    };

    // build entity

    todo!()
}

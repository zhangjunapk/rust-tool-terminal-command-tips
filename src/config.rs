use std::fs;
use yaml_rust::{Yaml, YamlLoader};

pub fn load_yaml() -> Vec<Yaml> {
    fs::read_to_string(
        "config.yaml",
    )
    .and_then(|s| Ok(YamlLoader::load_from_str(&s).unwrap()))
    .unwrap_or_else(|f| vec![])
}

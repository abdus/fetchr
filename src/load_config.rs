use serde::Deserialize;
use serde_json;
use std::{env, fs};

#[derive(Deserialize, Debug)]
pub struct Conf {
    pub key: String,
    pub display_name: String,
}

pub fn get_conf_from_file() -> Vec<Conf> {
    let fetchr_conf = env::var("FETCHR_CONF");
    let conf_path = match fetchr_conf {
        Ok(file_path) => Some(file_path),
        Err(_) => {
            let path: Option<String> = match env::var("HOME") {
                Ok(p) => Some(format!("{}/.config/fetchr/config.json", p)),
                Err(_) => None,
            };

            path
        }
    };

    let fetchr_conf = match conf_path {
        Some(path) => {
            let config_file_content = fs::read_to_string(path).expect("Failed to read Config File");
            let config = serde_json::from_str::<Vec<Conf>>(&config_file_content).expect(
                "Failed to parse config. Please make sure that the config file is a valid JSON",
            );

            config
        }
        None => get_default_conf(),
    };

    fetchr_conf
}

pub fn get_default_conf() -> Vec<Conf> {
    let conf_object: Vec<Conf> = vec![
        Conf {
            key: String::from("mem"),
            display_name: String::from("Memory"),
        },
        Conf {
            key: String::from("cpu"),
            display_name: String::from("CPU"),
        },
    ];

    conf_object
}

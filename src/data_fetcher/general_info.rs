use serde::{Deserialize, Serialize};
use std::env;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneralInfo {
    pub os_name: String,
    pub os_pretty_name: String,
    pub hostname: String,
    pub ansi_color: String,
    pub username: String,
}

impl GeneralInfo {
    pub fn new() -> Self {
        GeneralInfo {
            os_name: "".to_owned(),
            username: "".to_owned(),
            ansi_color: "".to_owned(),
            os_pretty_name: "".to_owned(),
            hostname: fs::read_to_string("/proc/sys/kernel/hostname")
                .expect("Failed to read /proc/sys/kernel/hostname")
                .trim()
                .to_owned(),
        }
    }
}

pub fn get_general_info() -> GeneralInfo {
    let mut gnrl_info = GeneralInfo::new();
    let os_release_file =
        fs::read_to_string("/etc/os-release").expect("could not read /etc/os-release");
    let lines = os_release_file.trim().split("\n");

    for line in lines {
        if line.starts_with("PRETTY_NAME=") {
            gnrl_info.os_pretty_name = line[13..line.len() - 1].to_owned();
        } else if line.starts_with("NAME=") {
            gnrl_info.os_name = line[6..line.len() - 1].to_owned();
        } else if line.starts_with("ANSI_COLOR=") {
            gnrl_info.ansi_color = line[12..line.len() - 1].to_owned();
        }
    }

    gnrl_info.username = env::var("USER").expect("Failed to read $USER env variable");

    gnrl_info
}

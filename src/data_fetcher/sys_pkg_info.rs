use crate::utils::distro_id;
use serde::{Deserialize, Serialize};
use std::{fs, process};

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageInfo {
    pub count: i64,
}

pub fn get_sys_pkg_info() -> PackageInfo {
    let distro_id = &distro_id::get_distro_id()[..];

    let count = match distro_id {
        "arch" => fs::read_dir("/var/lib/pacman/local")
            .expect("Failed to read files from /var/lib/pacman/local")
            .count() as i64,

        "debian" | "ubuntu" => {
            let count = String::from_utf8(
                process::Command::new("dpkg")
                    .arg("--list")
                    .output()
                    .unwrap()
                    .stdout,
            )
            .unwrap_or("".to_owned())
            .split("\n")
            .collect::<Vec<&str>>()[4..]
                .len() as i64;

            count
        }
        _ => 0,
    };

    PackageInfo { count }
}

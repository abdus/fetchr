use crate::utils::distro_id;
use serde::{Deserialize, Serialize};
use std::fs;

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
        _ => 0,
    };

    PackageInfo { count }
}

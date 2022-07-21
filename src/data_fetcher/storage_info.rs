use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct DiskInfo {
    pub partition_name: String,
    pub available_in_kb: i64,
    pub used_in_kb: i64,
    pub mounted_on: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageInfo {
    pub total_in_kb: i64,
    pub used_in_kb: i64,
    pub free_in_kb: i64,
}

impl StorageInfo {
    pub fn new() -> Self {
        StorageInfo {
            total_in_kb: 0,
            used_in_kb: 0,
            free_in_kb: 0,
        }
    }
}

pub fn get_storage_info() -> StorageInfo {
    let mut disks: Vec<DiskInfo> = Vec::new();
    let mut storage = StorageInfo::new();

    let df_output = String::from_utf8(
        Command::new("df")
            .args(["-k", "/"])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
    .trim()
    .to_owned();
    let mut df_output_lines: Vec<&str> = df_output.split("\n").collect();

    df_output_lines.remove(0);

    for line in df_output_lines.iter() {
        let chunks = line.split_whitespace().collect::<Vec<&str>>();
        let partition_name = chunks.get(0).unwrap().to_owned().to_owned();
        let used_in_kb = chunks.get(2).unwrap().to_owned().to_owned();
        let available_in_kb = chunks.get(3).unwrap().to_owned().to_owned();
        let mounted_on = chunks.get(5).unwrap().to_owned().to_owned();

        disks.push(DiskInfo {
            partition_name,
            available_in_kb: available_in_kb.parse::<i64>().unwrap(),
            used_in_kb: used_in_kb.parse::<i64>().unwrap(),
            mounted_on,
        });
    }

    for disk in disks.iter() {
        storage.total_in_kb = storage.total_in_kb + (disk.used_in_kb + disk.available_in_kb);
        storage.used_in_kb = storage.used_in_kb + disk.used_in_kb;
        storage.free_in_kb = storage.total_in_kb - storage.used_in_kb;
    }

    return storage;
}

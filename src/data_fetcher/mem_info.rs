use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct MemInfo {
    pub mem_available_in_kb: i64,
    pub mem_total_in_kb: i64,
    pub mem_free_in_kb: i64,
}

pub fn get_mem_info() -> MemInfo {
    let file = fs::read_to_string("/proc/meminfo").unwrap().to_owned();

    let mem_available_regex = Regex::new(r"MemAvailable:\s*([0-9]*)").unwrap();
    let mem_available_regex_regex_result = mem_available_regex.captures(&file).unwrap();
    let mem_available = &mem_available_regex_regex_result
        .get(1)
        .unwrap()
        .as_str()
        .to_owned();

    let memtotal_regex = Regex::new(r"MemTotal:\s*([0-9]*)").unwrap();
    let memtotal_regex_result = memtotal_regex.captures(&file).unwrap();
    let mem_total = &memtotal_regex_result.get(1).unwrap().as_str().to_owned();

    let mem_free_regex = Regex::new(r"MemFree:\s*([0-9]*)").unwrap();
    let mem_free_regex_regex_result = mem_free_regex.captures(&file).unwrap();
    let mem_free = &mem_free_regex_regex_result
        .get(1)
        .unwrap()
        .as_str()
        .to_owned();

    return MemInfo {
        mem_available_in_kb: mem_available.parse::<i64>().unwrap(),
        mem_total_in_kb: mem_total.parse::<i64>().unwrap(),
        mem_free_in_kb: mem_free.parse::<i64>().unwrap(),
    };
}

use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct CpuInfo {
    pub model_name: String,
    pub core_count: u8,
    pub thread_count: u8,
    pub max_freq_in_ghz: f32,
    pub min_freq_in_ghz: f32,
}

pub fn get_cpu_info() -> CpuInfo {
    let file_content = fs::read_to_string("/proc/cpuinfo").unwrap().to_owned();
    let lines_vec: Vec<&str> = file_content.split("\n").collect();

    let cpu_model_line = lines_vec.get(4).unwrap().to_string();
    let cpu_model = cpu_model_line.split(":").last();

    let cores_line = lines_vec.get(12).unwrap().to_string();
    let cores_count: String = cores_line
        .split(":")
        .last()
        .unwrap_or("0")
        .trim()
        .to_string();

    let thread_line = lines_vec.get(10).unwrap().to_string();
    let thread_count: String = thread_line
        .split(":")
        .last()
        .unwrap_or("0")
        .trim()
        .to_string();

    let mut cpu_frquencies: Vec<f32> = Vec::new();

    for line in lines_vec.iter() {
        if line.to_lowercase().starts_with("cpu mhz") {
            let zero = String::from("0");
            let chunks = line
                .split(":")
                .map(str::trim)
                .map(str::to_string)
                .collect::<Vec<String>>();

            let freq_str = chunks.last().unwrap_or(&zero);
            let frequency = freq_str.parse::<f32>().unwrap_or(0.0);

            //println!("{}", frequency / 1000.0);
            cpu_frquencies.push((frequency / 1000.0) as f32);
        }
    }

    let mut cpu_info = CpuInfo {
        max_freq_in_ghz: std::f32::MIN,
        min_freq_in_ghz: std::f32::MAX,
        model_name: cpu_model.unwrap_or("").trim().to_string(),
        core_count: cores_count.parse::<u8>().unwrap(),
        thread_count: thread_count.parse::<u8>().unwrap(),
    };

    for cf in cpu_frquencies.iter() {
        if cpu_info.max_freq_in_ghz < cf.to_owned() {
            cpu_info.max_freq_in_ghz = to_precision(cf.to_owned(), 1) as f32;
        }

        if cpu_info.min_freq_in_ghz > cf.to_owned() {
            cpu_info.min_freq_in_ghz = to_precision(cf.to_owned(), 1) as f32;
        }
    }

    cpu_info
}

fn to_precision(num: f32, precision: u32) -> f32 {
    let pow = 10_i64.pow(precision) as f32;
    (num * pow).round() / pow
}

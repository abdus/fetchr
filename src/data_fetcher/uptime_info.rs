use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct UptimeInfo {
    pub idle_sec: f64,
    pub uptime_sec: f64,
    pub idle_human_time: String,
    pub uptime_human_time: String,
}

fn get_human_time(time_sec: f64) -> String {
    let hour = (time_sec / (60.0 * 60.0)) as i64;
    let min = ((time_sec % (60.0 * 60.0)) / 60.0) as i64;

    return format!("{}h {}m", hour, min);
}

pub fn get_uptime_info() -> UptimeInfo {
    let uptime_content = fs::read_to_string("/proc/uptime")
        .unwrap()
        .trim()
        .to_owned();
    let segments: Vec<&str> = uptime_content.split_whitespace().collect();
    let idle_sec = segments.last().unwrap().parse::<f64>().unwrap_or(0.0);
    let uptime_sec = segments.first().unwrap().parse::<f64>().unwrap_or(0.0);

    UptimeInfo {
        idle_sec: idle_sec,
        uptime_sec: uptime_sec,
        idle_human_time: get_human_time(idle_sec),
        uptime_human_time: get_human_time(uptime_sec),
    }
}

use std::fs;

#[derive(Debug)]
pub struct CpuInfo {
    pub model_name: String,
    pub core_count: u8,
    pub thread_count: u8,
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

    return CpuInfo {
        model_name: cpu_model.unwrap_or("").trim().to_string(),
        core_count: cores_count.parse::<u8>().unwrap(),
        thread_count: thread_count.parse::<u8>().unwrap(),
    };
}

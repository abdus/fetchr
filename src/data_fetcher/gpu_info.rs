use regex::Regex;
use serde::{Deserialize, Serialize};
use std::process;

#[derive(Debug, Serialize, Deserialize)]
pub struct GpuInfo {
    pub model_name: String,
    pub kernel_driver: String,
    pub kernel_module: String,
}

#[cfg(target_os = "linux")]
pub fn get_gpu_info() -> Vec<GpuInfo> {
    let mut gpu_information: Vec<GpuInfo> = Vec::new();
    let lspci_output =
        String::from_utf8(process::Command::new("lspci").output().unwrap().stdout).unwrap();

    let gpu_id_regex = Regex::new(r"(([0-9:.]+)\s*VGA)").unwrap();
    let gpu_id_regex_result = gpu_id_regex.captures_iter(&lspci_output[..]);

    let kernel_driver_regex = Regex::new(r"Kernel driver in use:\S*(.*\S*)").unwrap();
    let kernel_module_regex = Regex::new(r"Kernel modules:\S*(.*\S*)").unwrap();
    let model_name_regex = Regex::new(r"[0-9]*\S*VGA compatible controller:\S*(.*\S*)").unwrap();

    for gpu_id in gpu_id_regex_result {
        let gpu_id = &gpu_id[2].trim().to_owned();

        // shadow variable. the original one is in the outer scope
        let lspci_output = String::from_utf8(
            process::Command::new("lspci")
                .args(["-v", "-s", gpu_id])
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap();

        let kernel_driver_result = kernel_driver_regex.captures(&lspci_output[..]).unwrap();
        let kernel_driver = kernel_driver_result
            .get(1)
            .unwrap()
            .as_str()
            .trim()
            .to_owned();

        let kernel_module_result = kernel_module_regex.captures(&lspci_output[..]).unwrap();
        let kernel_module = kernel_module_result
            .get(1)
            .unwrap()
            .as_str()
            .trim()
            .to_owned();

        let model_name_result = model_name_regex.captures(&lspci_output[..]).unwrap();
        let model_name = model_name_result.get(1).unwrap().as_str().trim().to_owned();

        gpu_information.push(GpuInfo {
            model_name,
            kernel_driver,
            kernel_module,
        })
    }

    return gpu_information;
}

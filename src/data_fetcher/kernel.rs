use std::fs;

#[derive(Debug)]
pub struct KernelInfo {
    pub full_name: String,
    pub short_name: String,
    pub short_name_with_version: String,
}

pub fn get_kernel() -> KernelInfo {
    let kernel = fs::read_to_string("/proc/version").unwrap().to_owned();
    let kernel_full = kernel.split("#").collect::<Vec<&str>>();
    let kernel_short = kernel.split(" ").collect::<Vec<&str>>();

    return KernelInfo {
        full_name: kernel_full.first().unwrap().to_string(),
        short_name: kernel_short.first().unwrap().to_string(),
        short_name_with_version: format!(
            "{} {}",
            kernel_short.first().unwrap(),
            kernel_short.get(2).unwrap()
        ),
    };
}

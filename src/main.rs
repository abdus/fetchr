use serde::{Deserialize, Serialize};
//use serde_json;
use chrono::{self, Timelike};

mod data_fetcher;
mod template;
mod utils;

#[derive(Debug, Serialize, Deserialize)]
struct SystemInfo {
    mem_info: data_fetcher::mem_info::MemInfo,
    kernel_info: data_fetcher::kernel::KernelInfo,
    shell_info: data_fetcher::shell::ShellInfo,
    cpu_info: data_fetcher::cpu_info::CpuInfo,
    //gpu_info: Vec<data_fetcher::gpu_info::GpuInfo>,
    storage_info: data_fetcher::storage_info::StorageInfo,
    package_info: data_fetcher::sys_pkg_info::PackageInfo,
    general_info: data_fetcher::general_info::GeneralInfo,
    uptime_info: data_fetcher::uptime_info::UptimeInfo,
}

fn main() {
    let keys_to_show: Vec<&str> = vec!["mem", "cpu", "gpu", "kernel", "disk", "pkg", "shell"];
    let mut longest_key_len: u8 = 0;

    for key in keys_to_show.iter() {
        let len = key.len() as u8;

        if len > longest_key_len {
            longest_key_len = len;
        }
    }

    let system_info = SystemInfo {
        mem_info: data_fetcher::mem_info::get_mem_info(),
        kernel_info: data_fetcher::kernel::get_kernel(),
        shell_info: data_fetcher::shell::get_shell_info(),
        cpu_info: data_fetcher::cpu_info::get_cpu_info(),
        //gpu_info: data_fetcher::gpu_info::get_gpu_info(),
        storage_info: data_fetcher::storage_info::get_storage_info(),
        package_info: data_fetcher::sys_pkg_info::get_sys_pkg_info(),
        general_info: data_fetcher::general_info::get_general_info(),
        uptime_info: data_fetcher::uptime_info::get_uptime_info(),
    };

    //let json = serde_json::to_string_pretty(&system_info).unwrap();
    let mut term_lines: Vec<template::TermLine> = Vec::new();

    println!(
        "\x1b[{}m{}",
        system_info.general_info.ansi_color,
        utils::logo::get_logo(&utils::distro_id::get_distro_id()[..])
    );

    let time = chrono::offset::Local::now();
    println!(
        "\n     {}@{} [ {:0>2}:{:0>2} {} ]\n",
        system_info.general_info.username,
        system_info.general_info.hostname,
        time.hour12().1,
        time.minute(),
        if time.hour12().0 { "PM" } else { "AM" }
    );

    // println!("{json}");
    term_lines.push(template::TermLine {
        key: " ╭─────────────╮".to_owned(),
        value: format!(""),
        color: utils::colorize::Colors::Green,
    });

    term_lines.push(template::TermLine {
        key: " │ MEM FREE      ".to_owned(),
        value: format!(
            "{} / {} MiB",
            system_info.mem_info.mem_available_in_kb / 1024,
            system_info.mem_info.mem_total_in_kb / 1024
        ),
        color: utils::colorize::Colors::Orange,
    });

    term_lines.push(template::TermLine {
        key: " │ CPU           ".to_owned(),
        value: format!("{} Cores", system_info.cpu_info.core_count),
        color: utils::colorize::Colors::Red,
    });

    //let mut gpu_drivers_str = String::new();

    //for (idx, gpu) in system_info.gpu_info.iter().enumerate() {
    //if idx != 0 {
    //gpu_drivers_str.push_str(", ")
    //};

    //gpu_drivers_str.push_str(&gpu.kernel_driver);
    //}

    //term_lines.push(template::TermLine {
    //key: " │ GPU DRIVERS   ".to_owned(),
    //value: format!("{}", gpu_drivers_str),
    //color: utils::colorize::Colors::Cyan,
    //});

    term_lines.push(template::TermLine {
        key: " │ KERNEL        ".to_owned(),
        value: format!("{}", system_info.kernel_info.short_name_with_version),
        color: utils::colorize::Colors::Magenta,
    });

    term_lines.push(template::TermLine {
        key: " │ DISK FREE     ".to_owned(),
        value: format!(
            "{} / {} GiB",
            system_info.storage_info.free_in_kb / (1024 * 1024),
            system_info.storage_info.total_in_kb / (1024 * 1024)
        ),
        color: utils::colorize::Colors::LightGray,
    });

    term_lines.push(template::TermLine {
        key: " │ PACKAGES      ".to_owned(),
        value: format!("{}", system_info.package_info.count),
        color: utils::colorize::Colors::Green,
    });

    term_lines.push(template::TermLine {
        key: " │ SHELL         ".to_owned(),
        value: format!("{}", system_info.shell_info.name),
        color: utils::colorize::Colors::Blue,
    });

    term_lines.push(template::TermLine {
        key: " │ UPTIME        ".to_owned(),
        value: format!("{}", system_info.uptime_info.uptime_human_time),
        color: utils::colorize::Colors::Red,
    });

    term_lines.push(template::TermLine {
        key: " │ IDLE          ".to_owned(),
        value: format!("{}", system_info.uptime_info.idle_human_time),
        color: utils::colorize::Colors::Cyan,
    });

    term_lines.push(template::TermLine {
        key: " ╰─────────────╯".to_owned(),
        value: format!(""),
        color: utils::colorize::Colors::Red,
    });

    template::write_to_term(&term_lines);
}

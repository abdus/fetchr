use serde::{Deserialize, Serialize};
//use serde_json;
use chrono::{self, Timelike};

mod data_fetcher;
mod load_config;
mod template;
mod utils;

pub trait VecExtended<T>: AsMut<Vec<T>> {
    fn prepend(&mut self, elem: T) {
        let mut vec = vec![elem];
        self.as_mut().splice(..0, vec.drain(..));
    }
}

impl<T> VecExtended<T> for Vec<T> {}

fn main() {
    let config = load_config::get_conf_from_file();
    let mut term_lines: Vec<template::TermLine> = Vec::new();

    let general_info = data_fetcher::general_info::get_general_info();

    println!(
        "\x1b[{}m{}",
        general_info.ansi_color,
        utils::logo::get_logo(&utils::distro_id::get_distro_id()[..])
    );

    let time = chrono::offset::Local::now();
    println!(
        "\n     {}@{} [ {:0>2}:{:0>2} {} ]\n",
        general_info.username,
        general_info.hostname,
        time.hour12().1,
        time.minute(),
        if time.hour12().0 { "PM" } else { "AM" }
    );

    let uptime_info = data_fetcher::uptime_info::get_uptime_info(); // just so that I don't call it repeatedly
    let mut longest_display_name_len = 0;

    for item in config.iter() {
        let len = item.display_name.len() + 3;
        longest_display_name_len = if len > longest_display_name_len {
            len
        } else {
            longest_display_name_len
        }
    }

    for item in config.iter() {
        let key = &item.key;
        let display_name = &format!(
            " {} : ",
            align_string(
                &item.display_name,
                longest_display_name_len,
                Alignment::LEFT,
                Some('·')
            )
        );

        match &key[..] {
            "mem" => {
                let mem_info = data_fetcher::mem_info::get_mem_info();
                term_lines.push(template::TermLine {
                    key: display_name.to_owned(),
                    value: format!(
                        "{} / {} MiB",
                        mem_info.mem_available_in_kb / 1024,
                        mem_info.mem_total_in_kb / 1024
                    ),
                    color: utils::colorize::Colors::Orange,
                });
            }

            "cpu" => {
                let cpu_info = data_fetcher::cpu_info::get_cpu_info();
                term_lines.push(template::TermLine {
                    key: display_name.to_owned(),
                    value: format!(
                        "{} [{}] @ {} GHz",
                        cpu_info.core_count, cpu_info.thread_count, cpu_info.max_freq_in_ghz
                    ),
                    color: utils::colorize::Colors::Red,
                });
            }

            "kernel" => {
                term_lines.push(template::TermLine {
                    key: display_name.to_owned(),
                    value: format!(
                        "{}",
                        data_fetcher::kernel::get_kernel().short_name_with_version
                    ),
                    color: utils::colorize::Colors::Magenta,
                });
            }

            "storage" => {
                let storage_info = data_fetcher::storage_info::get_storage_info();
                term_lines.push(template::TermLine {
                    key: display_name.to_owned(),
                    value: format!(
                        "{} / {} GiB",
                        storage_info.free_in_kb / (1024 * 1024),
                        storage_info.total_in_kb / (1024 * 1024)
                    ),
                    color: utils::colorize::Colors::LightGray,
                });
            }

            "pkgs" => {
                term_lines.push(template::TermLine {
                    key: display_name.to_owned(),
                    value: format!("{}", data_fetcher::sys_pkg_info::get_sys_pkg_info().count),
                    color: utils::colorize::Colors::Green,
                });
            }

            "shell" => {
                term_lines.push(template::TermLine {
                    key: display_name.to_owned(),
                    value: format!("{}", data_fetcher::shell::get_shell_info().name),
                    color: utils::colorize::Colors::Blue,
                });
            }

            "uptime" => {
                term_lines.push(template::TermLine {
                    key: display_name.to_owned(),
                    value: format!("{}", uptime_info.uptime_human_time),
                    color: utils::colorize::Colors::Red,
                });
            }

            "idle" => {
                term_lines.push(template::TermLine {
                    key: display_name.to_owned(),
                    value: format!("{}", uptime_info.idle_human_time),
                    color: utils::colorize::Colors::Red,
                });
            }

            _ => {}
        }
    }

    //let json = serde_json::to_string_pretty(&system_info).unwrap();

    // println!("{json}");

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
        key: String::new(),
        value: String::new(),
        color: utils::colorize::Colors::Green,
    });

    template::write_to_term(&term_lines);
}

enum Alignment {
    LEFT,
    RIGHT,
    CENTER,
}

fn align_string(text: &str, size: usize, alignment: Alignment, pad_with: Option<char>) -> String {
    let text_len = text.len();
    let repeat_count = size - text_len;
    let pad_char = match pad_with {
        Some(s) => s.to_string(),
        None => ' '.to_string(),
    };

    let aligned_str = match alignment {
        Alignment::RIGHT => {
            let padded = format!("{} {text}", pad_char.repeat(repeat_count));
            padded
        }

        Alignment::LEFT => {
            let padded = format!("{text} {}", pad_char.repeat(repeat_count));
            padded
        }

        Alignment::CENTER => {
            let pad_count: (usize, usize) = if repeat_count % 2 == 0 {
                // even. add half to right and left
                (repeat_count / 2, repeat_count / 2)
            } else {
                // odd. add half + 1 to left and half to right
                ((repeat_count / 2), (repeat_count / 2) + 1)
            };

            let padded = format!(
                "{} {text} {}",
                pad_char.repeat(pad_count.0),
                pad_char.repeat(pad_count.1)
            );

            padded
        }
    };

    aligned_str
}

#[derive(Debug, Serialize, Deserialize)]
struct SystemInfo {
    mem_info: data_fetcher::mem_info::MemInfo,
    kernel_info: data_fetcher::kernel::KernelInfo,
    shell_info: data_fetcher::shell::ShellInfo,
    cpu_info: data_fetcher::cpu_info::CpuInfo,
    gpu_info: Vec<data_fetcher::gpu_info::GpuInfo>,
    storage_info: data_fetcher::storage_info::StorageInfo,
    package_info: data_fetcher::sys_pkg_info::PackageInfo,
    general_info: data_fetcher::general_info::GeneralInfo,
    uptime_info: data_fetcher::uptime_info::UptimeInfo,
}

fn get_json() -> SystemInfo {
    SystemInfo {
        mem_info: data_fetcher::mem_info::get_mem_info(),
        kernel_info: data_fetcher::kernel::get_kernel(),
        shell_info: data_fetcher::shell::get_shell_info(),
        cpu_info: data_fetcher::cpu_info::get_cpu_info(),
        gpu_info: data_fetcher::gpu_info::get_gpu_info(),
        storage_info: data_fetcher::storage_info::get_storage_info(),
        package_info: data_fetcher::sys_pkg_info::get_sys_pkg_info(),
        general_info: data_fetcher::general_info::get_general_info(),
        uptime_info: data_fetcher::uptime_info::get_uptime_info(),
    }
}

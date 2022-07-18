mod data_fetcher;
mod utils;

fn main() {
    let mem_info = data_fetcher::mem_info::get_mem_info();
    let kernel_info = data_fetcher::kernel::get_kernel();
    let shell_info = data_fetcher::shell::get_shell_info();
    let cpu_info = data_fetcher::cpu_info::get_cpu_info();
    let gpu_info = data_fetcher::gpu_info::get_gpu_info();

    println!(
        "{:#?}\n\n{:#?}\n\n{:#?}\n\n{:#?}\n\n{:#?}",
        mem_info, kernel_info, shell_info, cpu_info, gpu_info
    );
}

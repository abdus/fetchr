mod data_fetcher;
mod utils;

fn main() {
    let mem_info = data_fetcher::mem_info::get_mem_info();
    let kernel_info = data_fetcher::kernel::get_kernel();
    let shell_info = data_fetcher::shell::get_shell_info();

    println!("{:#?}\n{:#?}\n{:#?}", mem_info, kernel_info, shell_info);
}

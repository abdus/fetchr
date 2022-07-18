use std::env;

#[derive(Debug)]
pub struct ShellInfo {
    pub name: String,
    pub path: String,
}

pub fn get_shell_info() -> ShellInfo {
    let shell_path = env::var("SHELL").unwrap().to_owned();
    let shell_path_splitted = shell_path.split("/");
    let mut shell_path_sgmnts: Vec<String> = Vec::new();

    for sgmnt in shell_path_splitted {
        shell_path_sgmnts.push(sgmnt.to_string())
    }

    return ShellInfo {
        name: shell_path_sgmnts.last().unwrap().clone(),
        path: shell_path,
    };
}

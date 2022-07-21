use std::fs;

pub fn get_distro_id() -> String {
    let mut os_id: String = "".to_owned();
    let file_content =
        fs::read_to_string("/etc/os-release").expect("Failed to read /etc/os-release");
    let lines = file_content.trim().split("\n").collect::<Vec<&str>>();

    for line in lines {
        if line.starts_with("ID=") {
            os_id = line[3..].trim().to_owned();
        }
    }

    os_id
}

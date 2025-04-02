use std::fs;

pub fn cpuinfo() -> String {
    let mut cpu = String::new();
    match fs::read_to_string("/proc/cpuinfo") {
        Ok(output) => {
            for line in output.split("\n") {
                if line.trim().starts_with("model name") {
                    cpu = line
                        .split(":")
                        .nth(1)
                        .map(|s| s.trim())
                        .unwrap_or("error")
                        .to_string();
                    break;
                }
            }
        }
        Err(e) => {
            println!("Warning \n {}", e);
        }
    }
    cpu
}

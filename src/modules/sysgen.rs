use std::process::Command;

pub struct Sysgen {
    pub distro: String,
    pub host: String,
    pub kernel: String,
    pub mobo: String,
}

pub fn sys_check() -> Sysgen {
    let mut distro = String::new();
    let mut host = String::new();
    let mut kernel = String::new();
    let mut mobo = String::new();

    match Command::new("hostnamectl").output() {
        Ok(output) => match String::from_utf8(output.stdout) {
            Ok(hostname) => {
                for line in hostname.split("\n") {
                    if line.trim().starts_with("Static") {
                        host = line
                            .split(":")
                            .nth(1)
                            .map(|s| s.trim())
                            .unwrap_or("error")
                            .to_string();
                    }
                    if line.trim().starts_with("Operating") {
                        distro = line
                            .split(':')
                            .nth(1)
                            .map(|s| s.trim())
                            .unwrap_or("error")
                            .to_string();
                    }
                    if line.trim().starts_with("Kernel") {
                        kernel = line
                            .split(":")
                            .nth(1)
                            .map(|s| s.trim())
                            .unwrap_or("error")
                            .to_string();
                    }
                    if line.trim().starts_with("Hardware Model") {
                        mobo = line
                            .split(":")
                            .nth(1)
                            .map(|s| s.trim())
                            .unwrap_or("error")
                            .to_string();
                    }
                }
            }
            Err(e) => {
                println!("Warning: \n {e}")
            }
        },
        Err(e) => {
            println!("Warning: \n {e}")
        }
    }
    Sysgen {
        distro,
        host,
        kernel,
        mobo,
    }
}

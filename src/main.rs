use std::env;

mod modules {
    pub mod cpu;
    pub mod sysgen;
}

fn main() {
    let system_general_info = modules::sysgen::sys_check();

    let username = env::var("USER").unwrap_or("N/A".to_string());

    println!("Os: {}", system_general_info.distro);
    println!("Host: {}@{}", username, system_general_info.host);
    println!("Kernel: {}", system_general_info.kernel);
    println!("Device: {}", system_general_info.mobo);
    println!("CPU: {}", modules::cpu::cpuinfo())
}

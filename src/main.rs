mod modules {
    pub mod cpu;
    pub mod de;
    pub mod mem;
    pub mod shell;
    pub mod sysgen;
    pub mod user;
}

use modules::de;
use modules::shell;
use modules::user;

fn main() {
    let system_general_info = modules::sysgen::sys_check();
    let memory = modules::mem::memory();

    println!("Os: {}", system_general_info.distro);
    println!("Host: {}@{}", user::user_name(), system_general_info.host);
    println!("DE/WM: {}", de::get_de());
    println!("Shell: {}", shell::get_shell());
    println!("Kernel: {}", system_general_info.kernel);
    println!(
        "Memory: {:.2} GiB / {:.2} GiB",
        memory.mem_used, memory.mem_full
    );
    println!("Device: {}", system_general_info.mobo);
    println!("CPU: {}", modules::cpu::cpuinfo());
}

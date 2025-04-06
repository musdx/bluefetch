use modules::de;
use modules::user;
mod modules {
    pub mod cpu;
    pub mod de;
    pub mod mem;
    pub mod sysgen;
    pub mod user;
}

fn main() {
    let system_general_info = modules::sysgen::sys_check();
    let memory = modules::mem::memory();

    println!("Os: {}", system_general_info.distro);
    println!("Host: {}@{}", user::user_name(), system_general_info.host);
    println!("DE/WM: {}", de::get_de());
    println!("Kernel: {}", system_general_info.kernel);
    println!(
        "Memory: {:.2} GiB / {:.2} GiB",
        memory.mem_used, memory.mem_full
    );
    println!("Device: {}", system_general_info.mobo);
    println!("CPU: {}", modules::cpu::cpuinfo());
}

use modules::user;

mod modules {
    pub mod cpu;
    pub mod mem;
    pub mod sysgen;
    pub mod user;
}

fn main() {
    let system_general_info = modules::sysgen::sys_check();
    let memory = modules::mem::memory();

    println!("Os: {}", system_general_info.distro);
    println!("Host: {}@{}", user::user_name(), system_general_info.host);
    println!("Kernel: {}", system_general_info.kernel);
    println!("Memory: {:.2}/{:.2}", memory.mem_used, memory.mem_full);
    println!("Device: {}", system_general_info.mobo);
    println!("CPU: {}", modules::cpu::cpuinfo())
}

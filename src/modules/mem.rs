use std::fs;

pub fn memory() {
    let mut mem_full: u64 = 0;
    let mut mem_free: u64 = 0;

    match fs::read_to_string("/proc/meminfo") {
        Ok(ouput) => {
            for line in ouput.split(":") {
                if line.trim().starts_with("MemTotal") {}
            }
        }
        Err(e) => {}
    }
}

use std::{f64, fs};

pub struct Sysmem {
    pub mem_full: f64,
    pub mem_used: f64,
}

pub fn memory() -> Sysmem {
    let mut mem_full: f64 = 0.0;
    let mut mem_free: f64 = 0.0;

    match fs::read_to_string("/proc/meminfo") {
        Ok(ouput) => {
            for line in ouput.split("\n") {
                if line.trim().starts_with("MemTotal:") {
                    mem_full = line
                        .split(":")
                        .nth(1)
                        .unwrap()
                        .trim()
                        .split_whitespace()
                        .next()
                        .unwrap()
                        .parse::<f64>()
                        .unwrap()
                        / (1024.0 * 1024.0);
                } else if line.trim().starts_with("MemAvailable:") {
                    mem_free = line
                        .split(":")
                        .nth(1)
                        .unwrap()
                        .trim()
                        .split_whitespace()
                        .next()
                        .unwrap()
                        .parse::<f64>()
                        .unwrap()
                        / (1024.0 * 1024.0);
                    break;
                }
            }
        }
        Err(e) => {
            println!("{e}")
        }
    }
    let mem_used: f64 = mem_full - mem_free;

    Sysmem { mem_full, mem_used }
}

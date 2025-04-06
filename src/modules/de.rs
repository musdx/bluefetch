use std::env;

pub fn get_de() -> String {
    env::var("XDG_SESSION_DESKTOP")
        .ok()
        .or_else(|| env::var("DESKTOP_SESSION").ok())
        .or_else(|| env::var("XDG_CURRENT_DESKTOP").ok())
        .or_else(|| env::var("CURRENT_DESKTOP").ok())
        .or_else(|| env::var("SESSION_DESKTOP").ok())
        .unwrap_or("N/A".to_string())
}

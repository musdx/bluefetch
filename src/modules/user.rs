use std::env;

pub fn user_name() -> String {
    env::var("USER").unwrap_or("N/A".to_string())
}

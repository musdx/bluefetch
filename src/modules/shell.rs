use std::env;

pub fn get_shell() -> String {
    let mut shell = String::new();
    let shellpure = env::var("SHELL").unwrap_or_else(|_| String::from("unknown"));
    if shellpure.contains(&"zsh") {
        shell = String::from("zsh")
    } else if shellpure.contains(&"bash") {
        shell = String::from("bash")
    } else if shellpure.contains(&"fish") {
        shell = String::from("fish")
    } else if shellpure.contains(&"dash") {
        shell = String::from("dash")
    } else if shellpure.contains(&"ksh") {
        shell = String::from("ksh")
    } else {
        shell = String::from("N/A")
    }
    shell
}

use std::path::PathBuf;

fn main() {
    let envs = ["SERVER_HOST", "SERVER_PORT", "DB_PATH", "WEB_ORIGIN"];
    if PathBuf::from("../../.env").exists() {
        println!("cargo::rerun-if-changed=../../.env");
    }
    for env in envs {
        println!("cargo::rerun-if-env-changed={env}");
        if let Ok(value) = std::env::var(env) {
            println!("cargo::rustc-env={env}={value}");
        }
    }
}

use std::path::PathBuf;

fn main() {
    if PathBuf::from("../../.env").exists() {
        println!("cargo::rerun-if-changed=../../.env");
    }
}

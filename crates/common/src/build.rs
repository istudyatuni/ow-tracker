use std::path::PathBuf;

const ENV_PATH: &str = "../../";

pub fn cargo_track_dotenv() {
    let env_file = PathBuf::from(ENV_PATH).join(".env.example");
    assert!(env_file.exists(), ".env.example should exist");

    let envs = std::fs::read_to_string(env_file).unwrap();
    let envs = envs
        .lines()
        .filter(|line| !line.starts_with("#"))
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.split_once("=").unwrap().0)
        .collect::<Vec<_>>();

    let env_file = PathBuf::from(ENV_PATH).join(".env");
    if env_file.exists() {
        println!("cargo::rerun-if-changed={}", env_file.display());
    }
    for env in envs {
        println!("cargo::rerun-if-env-changed={env}");
        if let Ok(value) = std::env::var(env) {
            println!("cargo::rustc-env={env}={value}");
        }
    }
}

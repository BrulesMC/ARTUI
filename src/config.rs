use indexmap::IndexMap;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

pub type Profile = IndexMap<String, Value>;
pub type Config = IndexMap<String, Profile>;

fn real_home_dir() -> PathBuf {
    // If invoked via sudo, resolve invoking user's home
    if let Ok(sudo_user) = std::env::var("SUDO_USER") {
        if let Ok(output) = std::process::Command::new("getent")
            .arg("passwd")
            .arg(&sudo_user)
            .output()
        {
            if output.status.success() {
                if let Some(line) = String::from_utf8_lossy(&output.stdout).lines().next() {
                    if let Some(home) = line.split(':').nth(5) {
                        return PathBuf::from(home.trim());
                    }
                }
            }
        }
    }

    // Normal execution
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
}

pub fn config_path() -> PathBuf {
    let mut p = real_home_dir();
    p.push(".config");
    p.push("ar.json");
    p
}

// Load the config from path
pub fn load_config() -> Config {
    let path = config_path();

    if !path.exists() {
        return Config::new();
    }

    let data = fs::read_to_string(path).unwrap_or_else(|_| "{}".into());
    serde_json::from_str(&data).unwrap_or_else(|_| Config::new())
}

pub fn save_config(cfg: &Config) {
    let path = config_path();

    // Prevent root-from-sudo writing user-owned files
    if unsafe { libc::geteuid() } == 0 && std::env::var("SUDO_USER").is_ok() {
        eprintln!("Refusing to write config as root into user home");
        return;
    }

    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let json = serde_json::to_string_pretty(cfg).unwrap();
    let _ = fs::write(path, json);
}

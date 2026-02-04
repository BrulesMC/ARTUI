use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::thread;
use std::time::Duration;
//get the user and return true if we are root
fn is_root() -> bool {
    std::env::var("USER").map(|u| u == "root").unwrap_or(false)
        || std::env::var("UID").map(|u| u == "0").unwrap_or(false)
}
fn get_real_home() -> PathBuf {
    // If running under sudo, use SUDO_USER
    if let Ok(sudo_user) = std::env::var("SUDO_USER") {
        // Look up home directory from /etc/passwd
        if let Ok(output) = std::process::Command::new("getent")
            .arg("passwd")
            .arg(&sudo_user)
            .output()
        {
            if output.status.success() {
                if let Some(line) = String::from_utf8_lossy(&output.stdout).lines().next() {
                    if let Some(home) = line.split(':').nth(5) {
                        return PathBuf::from(home);
                    }
                }
            }
        }
    }
    // Fallback: normal HOME
    std::env::var("HOME")
        .map(PathBuf::from)
        .expect("Could not determine home directory")
}
fn main() {
    //allows args to be passed in
    let args: Vec<String> = std::env::args().collect();
    //check for right number of args
    if args.len() < 2 {
        eprintln!("Usage: arctl <ProfileName>");
        std::process::exit(1);
    }
    //assign the arg to the profile name
    let profile_name = &args[1];
    //get the config directory
    let config_path = get_real_home().join(".config/ar.json");
    //load the config into memory
    let json_data = fs::read_to_string(&config_path).unwrap_or_else(|_| {
        eprintln!("Failed to read config file at {:?}", config_path);
        std::process::exit(1);
    });

    let profiles: Value = serde_json::from_str(&json_data).expect("Invalid JSON in config");
    //load the specified profile from the json in memory
    let profile = profiles[profile_name].as_object().unwrap_or_else(|| {
        eprintln!("Profile '{}' not found in config", profile_name);
        std::process::exit(1);
    });
    //Build the cpu fan curve command
    if let Some(cpu) = profile.get("cpu") {
        let cpu_cmd = format!(
            "asusctl fan-curve --mod-profile {} --fan cpu --data \"{}\"",
            profile_name,
            cpu.as_str().unwrap()
        );
        //assign the command to run as this
        let _ = Command::new("sh").arg("-c").arg(cpu_cmd).status();
    }
    //Build the gpu fan curve command
    if let Some(gpu) = profile.get("gpu") {
        let gpu_cmd = format!(
            "asusctl fan-curve --mod-profile {} --fan gpu --data \"{}\"",
            profile_name,
            gpu.as_str().unwrap()
        );
        let _ = Command::new("sh").arg("-c").arg(gpu_cmd).status();
    }
    // Build enable command (enables ALL fan curves for this profile)
    let enable_cmd = format!(
        "asusctl fan-curve --mod-profile {} --enable-fan-curves true",
        profile_name
    );
    let _ = Command::new("sh").arg("-c").arg(enable_cmd).status();

    //Build profile launch command
    let activate_cmd = format!("asusctl profile set --battery {}", profile_name);
    let _ = Command::new("sh").arg("-c").arg(activate_cmd).status();
    let activate_cmd = format!("asusctl profile set --ac {}", profile_name);
    let _ = Command::new("sh").arg("-c").arg(activate_cmd).status();
    //Build ryzenadj command
    let mut cmd = if is_root() {
        //if we are root already then dont use sudo, could break stuff
        String::from("ryzenadj")
    } else {
        //else just add sudo in, user will be prompteed if needed
        String::from("sudo ryzenadj")
    };
    for (key, value) in profile {
        if key == "cpu" || key == "gpu" {
            continue;
        }
        //match each key to value eg "apu-slow-limit":4000
        let val_str = match value {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            _ => continue,
        };
        //convert into usable mode eg, "apu-slow-limit":4000 => --apu-slow-limit 4000
        cmd.push_str(&format!(" --{} {}", key, val_str));
    }
    //Wait incase the asusctl messed with the powertables, this is for stablity
    thread::sleep(Duration::from_millis(1000));
    //run the ryzenadj command
    let _ = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .status()
        .expect("Failed to run ryzenadj");
}

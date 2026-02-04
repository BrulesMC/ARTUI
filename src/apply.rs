use std::process::Command;
pub fn apply_profile(profile_name: &str) {
    //run the arctl command, pass in the current profile
    let status = Command::new("arctl")
        .arg(profile_name)
        .status()
        .expect("Failed to run arctl");
    //falllback and warn if the arctl does not work, will not save you from instablity or crashes
    if !status.success() {
        eprintln!("arctl failed to apply profile '{}'", profile_name);
    }
}

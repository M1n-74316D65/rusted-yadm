use dirs::config_dir;
use std::process::Command;
use std::str;

pub fn get_git_user_info() -> (String, String) {
    // Get Git user name
    let name_output = Command::new("git")
        .arg("config")
        .arg("user.name")
        .output()
        .expect("Failed to execute git command");

    let name = str::from_utf8(&name_output.stdout)
        .expect("Failed to parse output")
        .trim();

    // Get Git user email
    let email_output = Command::new("git")
        .arg("config")
        .arg("user.email")
        .output()
        .expect("Failed to execute git command");

    let email = str::from_utf8(&email_output.stdout)
        .expect("Failed to parse output")
        .trim();

    // Return a tuple of the name and email
    (name.to_string(), email.to_string())
}

pub fn folder_path() -> String {
    config_dir()
        .unwrap()
        .join("rusted-yadm")
        .join("gitrepo")
        .to_str()
        .unwrap()
        .to_string()
}

use dirs::data_local_dir;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::str;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use walkdir::WalkDir;

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
    data_local_dir()
        .unwrap()
        .join("rusted-yadm")
        .join("repository")
        .to_str()
        .unwrap()
        .to_string()
}

pub struct LoadingAnimation {
    stop_signal: Arc<AtomicBool>,
}

impl LoadingAnimation {
    pub fn new() -> Self {
        LoadingAnimation {
            stop_signal: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&self, message: &str) {
        let stop_signal = self.stop_signal.clone();
        let message = message.to_string();
        thread::spawn(move || {
            let spinner = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
            let mut i = 0;
            while !stop_signal.load(Ordering::Relaxed) {
                print!("\r{} {} ", spinner[i], message);
                io::stdout().flush().unwrap();
                thread::sleep(Duration::from_millis(100));
                i = (i + 1) % spinner.len();
            }
            print!("\r");
            io::stdout().flush().unwrap();
        });
    }

    pub fn stop(&self) {
        self.stop_signal.store(true, Ordering::Relaxed);
        thread::sleep(Duration::from_millis(150)); // Give time for the animation to stop
    }
}

pub fn copy_files_to_home() -> Result<(), Box<dyn std::error::Error>> {
    let folder_path = folder_path();

    let home_dir = dirs::home_dir().ok_or("Could not get home directory")?;
    let repo_path = Path::new(folder_path.as_str());
    let mut skipped_files = Vec::new();

    for entry in WalkDir::new(repo_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            !e.path()
                .strip_prefix(repo_path)
                .unwrap()
                .starts_with(".git")
        })
    {
        let path = entry.path();
        if path.is_file() {
            let relative_path = path.strip_prefix(repo_path)?;
            let destination = home_dir.join(relative_path);

            if let Some(parent) = destination.parent() {
                if let Err(e) = fs::create_dir_all(parent) {
                    eprintln!("Failed to create directory {:?}: {}", parent, e);
                    continue;
                }
            }

            match fs::copy(path, &destination) {
                Ok(_) => println!("Copied: {:?}", relative_path),
                Err(e) => {
                    if e.kind() == io::ErrorKind::PermissionDenied {
                        skipped_files.push(relative_path.to_path_buf());
                        eprintln!("Skipped (permission denied): {:?}", relative_path);
                    } else {
                        eprintln!("Failed to copy {:?}: {}", relative_path, e);
                    }
                }
            }
        }
    }

    if !skipped_files.is_empty() {
        println!("\nThe following files were skipped due to permission issues:");
        for file in skipped_files {
            println!("  {:?}", file);
        }
        println!("\nYou may need to manually copy these files or run the program with elevated permissions.");
    }

    Ok(())
}

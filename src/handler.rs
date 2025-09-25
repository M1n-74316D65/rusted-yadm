use crate::git;
use crate::utils;
use git2::{Repository, Signature, Status};
use std::fs;
use std::io::Write;
use std::path::Path;
use walkdir::WalkDir;

pub fn list() -> Result<(), Box<dyn std::error::Error>> {
    let _repo = git::get_repo()?;
    let repo_path = utils::folder_path();

    let mut files = Vec::new();

    // Walk through the repository directory
    for entry in WalkDir::new(&repo_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            !e.path()
                .strip_prefix(&repo_path)
                .unwrap()
                .starts_with(".git")
        })
    {
        let path = entry.path();
        if path.is_file() {
            if let Ok(relative_path) = path.strip_prefix(&repo_path) {
                files.push(relative_path.to_string_lossy().into_owned());
            }
        }
    }

    if files.is_empty() {
        println!("No files tracked in repository");
        return Ok(());
    }

    println!("Tracked files:");
    for file in files.iter() {
        println!("  {}", file);
    }

    Ok(())
}

pub fn status() -> Result<(), Box<dyn std::error::Error>> {
    let repo = git::get_repo()?;

    let statuses = repo.statuses(None)?;

    if statuses.is_empty() {
        println!("Working tree clean");
        return Ok(());
    }

    println!("Changes in working directory:");
    for status in statuses.iter() {
        let path = status.path().unwrap_or("unknown");
        let status_bits = status.status();

        let status_str = if status_bits.contains(Status::WT_NEW) {
            "A"
        } else if status_bits.contains(Status::WT_MODIFIED) {
            "M"
        } else if status_bits.contains(Status::WT_DELETED) {
            "D"
        } else if status_bits.contains(Status::INDEX_NEW) {
            "A  (staged)"
        } else if status_bits.contains(Status::INDEX_MODIFIED) {
            "M  (staged)"
        } else if status_bits.contains(Status::INDEX_DELETED) {
            "D  (staged)"
        } else {
            "?"
        };

        println!("  {}  {}", status_str, path);
    }

    Ok(())
}

pub fn init(url: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let repo_path = utils::folder_path();

    // Check if repository already exists
    if git::repo_exists() {
        return Err("Repository already exists. Use --force with clone to overwrite.".into());
    }

    // Create repository directory
    fs::create_dir_all(&repo_path)?;

    // Initialize git repository
    let repo = Repository::init(&repo_path)?;

    // Create basic .gitignore
    let gitignore_path = Path::new(&repo_path).join(".gitignore");
    let mut gitignore_file = fs::File::create(gitignore_path)?;
    gitignore_file.write_all(b"# Common files to ignore\n*.tmp\n*.log\n.DS_Store\n")?;

    // Add and commit .gitignore
    let mut index = repo.index()?;
    index.add_path(Path::new(".gitignore"))?;
    index.write()?;

    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    let (name, email) = utils::get_git_user_info();
    let signature = Signature::now(&name, &email)?;

    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Initial commit - add .gitignore",
        &tree,
        &[],
    )?;

    // Add remote origin if URL provided
    if let Some(remote_url) = url {
        let _remote = repo.remote("origin", remote_url)?;
        println!("Added remote origin: {}", remote_url);
    }

    println!("Initialized empty yadm repository in {}", repo_path);
    Ok(())
}

pub fn clone(url: &str, force: bool) -> Result<(), Box<dyn std::error::Error>> {
    let result = git::clone(url, utils::folder_path().as_str(), force);

    match result {
        Ok(_) => {
            println!("Repo cloned successfully");

            let copy_result = utils::copy_files_to_home();

            match copy_result {
                Ok(_) => println!("Files copied to home directory successfully"),
                Err(e) => eprintln!("Failed to copy files: {}", e),
            }

            Ok(())
        }
        Err(e) => {
            eprintln!("Repo clone failed: {}", e);
            Err(e)
        }
    }
}

pub fn clone_ssh(url: &str, force: bool) -> Result<(), Box<dyn std::error::Error>> {
    let result = git::clone_ssh(url, utils::folder_path().as_str(), force);

    match result {
        Ok(_) => {
            println!("Repo cloned successfully");

            let copy_result = utils::copy_files_to_home();

            match copy_result {
                Ok(_) => println!("Files copied to home directory successfully"),
                Err(e) => eprintln!("Failed to copy files: {}", e),
            }

            Ok(())
        }
        Err(e) => {
            eprintln!("Repo clone failed: {}", e);
            Err(e)
        }
    }
}

pub fn add(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let repo = git::get_repo()?;
    let mut index = repo.index()?;

    // Normalize the source file path
    let source_path = utils::normalize_path(file_path)?;
    println!("Source path: {:?}", source_path);

    // Check if the source file exists
    if !source_path.exists() {
        return Err(format!("File does not exist: {:?}", source_path).into());
    }

    // Get the repository's working directory
    let repo_dir = repo.workdir().ok_or("Could not get repository directory")?;

    // Get relative path for repository storage
    let relative_path = if let Some(stripped) = file_path.strip_prefix("~/") {
        Path::new(stripped)
    } else if file_path.starts_with('/') {
        // For absolute paths, store relative to home directory
        let home_dir = dirs::home_dir().ok_or("Could not get home directory")?;
        source_path
            .strip_prefix(home_dir)
            .map_err(|_| "File is not in home directory")?
    } else {
        // For relative paths, store as-is
        Path::new(file_path)
    };

    // Construct the destination path in the repository
    let dest_path = repo_dir.join(relative_path);

    println!("Destination path: {:?}", dest_path);

    // Create parent directories if they don't exist
    if let Some(parent) = dest_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Copy the file or symlink from home directory to the repository
    if source_path.is_symlink() {
        // Handle symlinks - read the target and create symlink in repo
        if let Ok(target) = fs::read_link(&source_path) {
            #[cfg(unix)]
            {
                std::os::unix::fs::symlink(&target, &dest_path)?;
            }
            #[cfg(not(unix))]
            {
                std::os::windows::fs::symlink_file(&target, &dest_path)?;
            }
            println!("Added symlink: {} -> {}", file_path, target.display());
        } else {
            return Err(format!("Failed to read symlink target: {:?}", source_path).into());
        }
    } else {
        // Handle regular files
        fs::copy(&source_path, &dest_path)?;
    }

    // Add the file to the index
    index.add_path(Path::new(relative_path))?;

    // Write the index to disk
    index.write()?;

    println!("Added file: {}", file_path);
    Ok(())
}

pub fn commit(message: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Run pre-commit hook validation
    git::run_pre_commit_hook()?;

    let (name, email) = utils::get_git_user_info();
    let repo = git::get_repo()?;

    let mut index = repo.index()?;
    let tree_id = index.write_tree()?;

    let tree = repo.find_tree(tree_id)?;

    // Get the current HEAD commit, or create an initial commit.
    let parent_commit = match repo.head() {
        Ok(head) => Some(repo.find_commit(head.target().unwrap())?),
        Err(_) => None,
    };

    // Create a signature for the commit.
    let signature = Signature::now(&name, &email)?;

    // Create the commit.
    match parent_commit {
        Some(parent) => {
            repo.commit(
                Some("HEAD"), // the refname for the HEAD
                &signature,   // the author of the commit
                &signature,   // the committer of the commit
                message,      // the commit message
                &tree,        // the tree object
                &[&parent],   // parents of the commit
            )?;
        }
        None => {
            repo.commit(
                Some("HEAD"),     // the refname for the HEAD
                &signature,       // the author of the commit
                &signature,       // the committer of the commit
                "Initial commit", // the commit message
                &tree,            // the tree object
                &[],              // no parents, this is the first commit
            )?;
        }
    }

    Ok(())
}

pub fn fetch() -> Result<(), Box<dyn std::error::Error>> {
    git::fetch()?;
    Ok(())
}

pub fn pull(branch: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    git::pull(branch)?;
    Ok(())
}

pub fn push(branch: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    git::push(branch)?;
    Ok(())
}

pub fn hook_install() -> Result<(), Box<dyn std::error::Error>> {
    git::install_hooks()?;
    Ok(())
}

pub fn hook_uninstall() -> Result<(), Box<dyn std::error::Error>> {
    git::uninstall_hooks()?;
    Ok(())
}

pub fn hook_pre_commit() -> Result<(), Box<dyn std::error::Error>> {
    git::run_pre_commit_hook()?;
    Ok(())
}

pub fn config(
    key: Option<&str>,
    value: Option<&str>,
    list: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    use crate::config::Config;

    let mut config = Config::load()?;

    if list {
        // List all configuration values
        let config_items = config.list();
        if config_items.is_empty() {
            println!("No configuration values set");
        } else {
            println!("Configuration values:");
            for (k, v) in config_items {
                println!("  {} = {}", k, v);
            }
        }
    } else if let Some(k) = key {
        match value {
            Some(val) => {
                // Set configuration value
                config.set(k, val)?;
                config.save()?;
                println!("Set {} = {}", k, val);
            }
            None => {
                // Get configuration value
                match config.get(k)? {
                    Some(val) => println!("{} = {}", k, val),
                    None => println!("{} is not set", k),
                }
            }
        }
    } else {
        println!("Either specify a key or use --list to show all configuration values");
    }

    Ok(())
}

pub fn alt(dry_run: bool, force: bool) -> Result<(), Box<dyn std::error::Error>> {
    use crate::config::Config;

    let repo = git::get_repo()?;
    let config = Config::load()?;
    let repo_path = utils::folder_path();

    // Get system information for alternate file selection
    let os = std::env::consts::OS;
    let hostname = get_hostname()?;
    let user = std::env::var("USER").unwrap_or_else(|_| "unknown".to_string());
    let class = config.local.class.as_deref().unwrap_or("default");

    println!("Processing alternate files...");
    println!("  OS: {}", os);
    println!("  Hostname: {}", hostname);
    println!("  User: {}", user);
    println!("  Class: {}", class);

    let mut processed_count = 0;

    // Walk through the repository directory
    for entry in WalkDir::new(&repo_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            !e.path()
                .strip_prefix(&repo_path)
                .unwrap()
                .starts_with(".git")
        })
    {
        let path = entry.path();

        // Check if this is an alternate file (contains ##)
        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            if file_name.contains("##") {
                if let Ok(relative_path) = path.strip_prefix(&repo_path) {
                    if process_alternate_file(
                        &repo,
                        relative_path,
                        os,
                        &hostname,
                        &user,
                        class,
                        dry_run,
                        force,
                    )? {
                        processed_count += 1;
                    }
                }
            }
        }
    }

    if processed_count == 0 {
        println!("No alternate files found to process");
    } else {
        println!("Processed {} alternate file(s)", processed_count);
    }

    Ok(())
}

fn process_alternate_file(
    repo: &Repository,
    alt_path: &Path,
    os: &str,
    hostname: &str,
    user: &str,
    class: &str,
    dry_run: bool,
    force: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    use std::fs;

    let file_name = alt_path.file_name().unwrap().to_str().unwrap();
    let parent_dir = alt_path.parent().unwrap();

    // Parse the alternate file pattern
    let base_name = if let Some(pos) = file_name.find("##") {
        &file_name[..pos]
    } else {
        return Ok(false);
    };

    // Extract the condition part
    let condition = &file_name[file_name.find("##").unwrap() + 2..];

    // Check if this alternate file matches our system
    let should_process = match condition {
        "os" => true,
        "hostname" => true,
        "user" => true,
        "class" => true,
        cond if cond.starts_with("os.") => &cond[3..] == os,
        cond if cond.starts_with("hostname.") => &cond[10..] == hostname,
        cond if cond.starts_with("user.") => &cond[5..] == user,
        cond if cond.starts_with("class.") => &cond[6..] == class,
        _ => false,
    };

    if !should_process {
        return Ok(false);
    }

    // Construct the target path
    let target_path = parent_dir.join(base_name);
    let repo_workdir = repo.workdir().unwrap();
    let alt_full_path = repo_workdir.join(alt_path);
    let target_full_path = repo_workdir.join(&target_path);

    // Check if target already exists
    if target_full_path.exists() && !force {
        println!(
            "Skipping: {:?} (already exists, use --force to override)",
            target_path
        );
        return Ok(false);
    }

    if dry_run {
        println!("Would process: {:?} -> {:?}", alt_path, target_path);
    } else {
        // Copy the alternate file to the target location
        if let Some(parent) = target_full_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::copy(&alt_full_path, &target_full_path)?;
        println!("Processed: {:?} -> {:?}", alt_path, target_path);

        // Add the target file to git index
        let mut index = repo.index()?;
        index.add_path(&target_path)?;
        index.write()?;
    }

    Ok(true)
}

pub fn encrypt(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    use aes::cipher::{KeyIvInit, StreamCipher};
    use aes::Aes256;
    use ctr::Ctr128BE;
    use rpassword;
    use std::fs;
    use std::io::Read;
    use std::io::Write;

    type Aes256Ctr128BE = Ctr128BE<Aes256>;

    // Normalize the file path
    let source_path = utils::normalize_path(file_path)?;

    // Check if the source file exists
    if !source_path.exists() {
        return Err(format!("File does not exist: {:?}", source_path).into());
    }

    // Read the file content
    let mut file = fs::File::open(&source_path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;

    // Get encryption password from user
    println!("Enter encryption password:");
    let password = rpassword::read_password()?;

    if password.is_empty() {
        return Err("Password cannot be empty".into());
    }

    // Derive key from password (simple hash for demo - in production use proper KDF)
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let key = hasher.finalize();

    // Generate random nonce
    let nonce = rand::random::<[u8; 16]>();

    // Create cipher
    let mut cipher =
        Aes256Ctr128BE::new_from_slices(&key, &nonce).map_err(|_| "Failed to create cipher")?;

    // Encrypt the content
    let mut encrypted_content = content.clone();
    cipher.apply_keystream(&mut encrypted_content);

    // Combine nonce and encrypted content
    let mut final_content = Vec::new();
    final_content.extend_from_slice(&nonce);
    final_content.extend_from_slice(&encrypted_content);

    // Create encrypted file path
    let encrypted_path = source_path.with_extension(format!(
        "{}.enc",
        source_path
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or("")
    ));

    // Write encrypted content to file
    let mut encrypted_file = fs::File::create(&encrypted_path)?;
    encrypted_file.write_all(&final_content)?;

    println!("File encrypted successfully: {:?}", encrypted_path);
    Ok(())
}

pub fn decrypt(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    use aes::cipher::{KeyIvInit, StreamCipher};
    use aes::Aes256;
    use ctr::Ctr128BE;
    use rpassword;
    use std::fs;
    use std::io::Read;
    use std::io::Write;

    type Aes256Ctr128BE = Ctr128BE<Aes256>;

    // Normalize the file path
    let source_path = utils::normalize_path(file_path)?;

    // Check if the source file exists
    if !source_path.exists() {
        return Err(format!("File does not exist: {:?}", source_path).into());
    }

    // Read the encrypted file content
    let mut file = fs::File::open(&source_path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;

    // Extract nonce (first 16 bytes) and encrypted content
    if content.len() < 16 {
        return Err("Invalid encrypted file: too short".into());
    }

    let nonce = &content[0..16];
    let encrypted_content = &content[16..];

    // Get decryption password from user
    println!("Enter decryption password:");
    let password = rpassword::read_password()?;

    if password.is_empty() {
        return Err("Password cannot be empty".into());
    }

    // Derive key from password (same as encryption)
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let key = hasher.finalize();

    // Create cipher
    let mut cipher =
        Aes256Ctr128BE::new_from_slices(&key, nonce).map_err(|_| "Failed to create cipher")?;

    // Decrypt the content (CTR mode is symmetric)
    let mut decrypted_content = encrypted_content.to_vec();
    cipher.apply_keystream(&mut decrypted_content);

    // Create decrypted file path (remove .enc extension)
    let decrypted_path = source_path.with_extension({
        let ext = source_path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        if let Some(stripped) = ext.strip_suffix(".enc") {
            stripped
        } else {
            ext
        }
    });

    // Write decrypted content to file
    let mut decrypted_file = fs::File::create(&decrypted_path)?;
    decrypted_file.write_all(&decrypted_content)?;

    println!("File decrypted successfully: {:?}", decrypted_path);
    Ok(())
}

pub fn bootstrap(
    config_path: Option<&str>,
    force: bool,
    yes: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting bootstrap process...");

    // Check if yadm repository already exists
    if git::repo_exists() && !force {
        if !yes {
            println!("Yadm repository already exists.");
            println!("Do you want to continue and potentially overwrite existing files? [y/N]");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            if input.trim().to_lowercase() != "y" {
                println!("Bootstrap cancelled.");
                return Ok(());
            }
        }
    }

    // Load or create bootstrap configuration
    let bootstrap_config = if let Some(config) = config_path {
        load_bootstrap_config(config)?
    } else {
        // Create default bootstrap configuration
        create_default_bootstrap_config()?
    };

    println!("Bootstrap configuration loaded:");
    if let Some(ref repo) = bootstrap_config.repository {
        println!("  Repository: {}", repo);
    }
    if let Some(ref branch) = bootstrap_config.branch {
        println!("  Branch: {}", branch);
    }
    println!("  Files to bootstrap: {}", bootstrap_config.files.len());

    // Step 1: Initialize or clone repository
    if !git::repo_exists() {
        println!("Initializing yadm repository...");
        init(None)?;
    }

    // Step 2: Add remote if specified
    if let Some(ref remote_url) = bootstrap_config.remote {
        println!("Adding remote origin: {}", remote_url);
        let repo = git::get_repo()?;
        let remote_exists = repo.find_remote("origin").is_ok();
        if !remote_exists {
            repo.remote("origin", remote_url)?;
        }
    }

    // Step 3: Clone or fetch repository if URL is provided
    if let Some(ref repo_url) = bootstrap_config.repository {
        if repo_url.starts_with("git@") || repo_url.starts_with("ssh://") {
            println!("Cloning from SSH repository...");
            clone_ssh(repo_url, force)?;
        } else {
            println!("Cloning from HTTPS repository...");
            clone(repo_url, force)?;
        }
    }

    // Step 4: Checkout specific branch if specified
    if let Some(ref branch) = bootstrap_config.branch {
        println!("Checking out branch: {}", branch);
        let repo = git::get_repo()?;
        let obj = repo.revparse_single(&format!("origin/{}", branch))?;
        repo.checkout_tree(&obj, None)?;
        repo.set_head(&format!("refs/heads/{}", branch))?;
    }

    // Step 5: Process alternate files
    println!("Processing alternate files...");
    alt(false, force)?;

    // Step 6: Bootstrap specific files
    println!("Bootstrapping files...");
    for file_config in &bootstrap_config.files {
        bootstrap_file(file_config, force, yes)?;
    }

    // Step 7: Run post-bootstrap commands
    if let Some(ref commands) = bootstrap_config.post_commands {
        println!("Running post-bootstrap commands...");
        for command in commands {
            run_bootstrap_command(command)?;
        }
    }

    println!("Bootstrap completed successfully!");
    Ok(())
}

fn load_bootstrap_config(config_path: &str) -> Result<BootstrapConfig, Box<dyn std::error::Error>> {
    use std::fs;

    // Check if config_path is a URL
    if config_path.starts_with("http://") || config_path.starts_with("https://") {
        // Download configuration from URL
        let response = reqwest::blocking::get(config_path)?;
        let content = response.text()?;
        let config: BootstrapConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    } else {
        // Load configuration from local file
        let content = fs::read_to_string(config_path)?;
        let config: BootstrapConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }
}

fn create_default_bootstrap_config() -> Result<BootstrapConfig, Box<dyn std::error::Error>> {
    Ok(BootstrapConfig {
        repository: None,
        remote: None,
        branch: None,
        files: Vec::new(),
        post_commands: None,
    })
}

fn bootstrap_file(
    file_config: &BootstrapFile,
    force: bool,
    yes: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;

    let source_path = utils::normalize_path(&file_config.source)?;
    let target_path = utils::normalize_path(&file_config.target)?;

    // Check if target already exists
    if target_path.exists() && !force {
        if !yes {
            println!("File already exists: {:?}", target_path);
            println!("Overwrite? [y/N]");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            if input.trim().to_lowercase() != "y" {
                println!("Skipping file: {:?}", file_config.source);
                return Ok(());
            }
        }
    }

    // Create parent directories if they don't exist
    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Copy the file
    fs::copy(&source_path, &target_path)?;
    println!("Bootstrapped: {:?} -> {:?}", source_path, target_path);

    // Set permissions if specified
    if let Some(ref perms) = file_config.permissions {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = fs::metadata(&target_path)?.permissions();
        permissions.set_mode(*perms);
        fs::set_permissions(&target_path, permissions)?;
        println!("Set permissions: {:o} on {:?}", perms, target_path);
    }

    Ok(())
}

fn run_bootstrap_command(command: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::process::Command;

    println!("Running command: {}", command);

    let shell = if cfg!(target_os = "windows") {
        "cmd"
    } else {
        "sh"
    };
    let flag = if cfg!(target_os = "windows") {
        "/C"
    } else {
        "-c"
    };

    let output = Command::new(shell)
        .args(&[flag, command])
        .output()
        .map_err(|e| format!("Failed to execute command '{}': {}", command, e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(format!(
            "Command failed: {}\nStdout: {}\nStderr: {}",
            command, stdout, stderr
        )
        .into());
    }

    println!("Command completed successfully");
    Ok(())
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct BootstrapConfig {
    repository: Option<String>,
    remote: Option<String>,
    branch: Option<String>,
    files: Vec<BootstrapFile>,
    post_commands: Option<Vec<String>>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct BootstrapFile {
    source: String,
    target: String,
    permissions: Option<u32>,
}

fn get_hostname() -> Result<String, Box<dyn std::error::Error>> {
    use std::process::Command;

    let output = Command::new("hostname")
        .output()
        .map_err(|_| "Failed to get hostname")?;

    if output.status.success() {
        let hostname = String::from_utf8(output.stdout)?.trim().to_string();
        Ok(hostname)
    } else {
        Ok("unknown".to_string())
    }
}

use crate::config::Config;
use crate::utils::folder_path;
use git2::{build::RepoBuilder, Cred, FetchOptions, PushOptions, RemoteCallbacks, Repository};
use std::env;
use std::fs;
use std::path::Path;

fn get_ssh_credentials() -> Result<Cred, git2::Error> {
    let config = Config::load().unwrap_or_default();

    if config.ssh.agent {
        // Try SSH agent first if enabled
        if let Ok(cred) = Cred::ssh_key_from_agent("git") {
            return Ok(cred);
        }
    }

    // Fall back to SSH key from config or default
    let ssh_key_path = config
        .ssh
        .key
        .unwrap_or_else(|| format!("{}/.ssh/id_rsa", env::var("HOME").unwrap()));

    Cred::ssh_key("git", None, Path::new(&ssh_key_path), None)
}

pub fn clone(url: &str, path: &str, force: bool) -> Result<(), Box<dyn std::error::Error>> {
    if Path::new(path).exists() {
        if force {
            println!("Force flag set. Removing existing directory.");
            fs::remove_dir_all(path)?;
        } else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "Directory already exists. Use --force to overwrite.",
            )));
        }
    }
    Repository::clone(url, path)?;
    Ok(())
}

pub fn clone_ssh(url: &str, path: &str, force: bool) -> Result<(), Box<dyn std::error::Error>> {
    if Path::new(path).exists() {
        if force {
            println!("Force flag set. Removing existing directory.");
            fs::remove_dir_all(path)?;
        } else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "Directory already exists. Use --force to overwrite.",
            )));
        }
    }

    // Prepare callbacks.
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, _username_from_url, _allowed_types| get_ssh_credentials());

    // Prepare fetch options.
    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callbacks);

    // Prepare builder.
    let mut builder = RepoBuilder::new();
    builder.fetch_options(fo);

    // Clone the project.
    builder.clone(url, Path::new(path))?;
    Ok(())
}

pub fn open(path: &str) -> Repository {
    // Remove this line: get_repo();

    match Repository::open(path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    }
}

pub fn get_repo() -> Result<Repository, Box<dyn std::error::Error>> {
    let folder_path = folder_path();
    let repo_path = folder_path.as_str();

    if !Path::new(repo_path).exists() {
        return Err("Repository does not exist. Use 'clone' or 'init' to create one.".into());
    }

    match Repository::open(repo_path) {
        Ok(repo) => Ok(repo),
        Err(e) => Err(format!("Failed to open repository: {}", e).into()),
    }
}

pub fn repo_exists() -> bool {
    let folder_path = folder_path();
    Path::new(&folder_path).exists()
}

pub fn push(branch: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let repo = get_repo()?;

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, _username_from_url, _allowed_types| get_ssh_credentials());

    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(callbacks);

    let mut remote = repo.find_remote("origin")?;

    // Get the branch name to push
    let branch_name = match branch {
        Some(b) => b,
        None => {
            let head = repo.head()?;
            let shorthand = head
                .shorthand()
                .ok_or("Failed to get branch name")?
                .to_string();
            Box::leak(shorthand.into_boxed_str())
        }
    };

    // Push the specified branch to the remote
    remote.push(
        &[&format!("refs/heads/{0}:refs/heads/{0}", branch_name)],
        Some(&mut push_options),
    )?;

    Ok(())
}

pub fn fetch() -> Result<(), Box<dyn std::error::Error>> {
    let repo = get_repo()?;

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, _username_from_url, _allowed_types| get_ssh_credentials());

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    let mut remote = repo.find_remote("origin")?;
    remote.fetch(
        &["refs/heads/*:refs/heads/*"],
        Some(&mut fetch_options),
        None,
    )?;

    Ok(())
}

pub fn pull(branch: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let repo = get_repo()?;

    // Fetch first
    fetch()?;

    // Get the branch name to pull
    let branch_name = match branch {
        Some(b) => b,
        None => {
            let head = repo.head()?;
            let shorthand = head
                .shorthand()
                .ok_or("Failed to get branch name")?
                .to_string();
            Box::leak(shorthand.into_boxed_str())
        }
    };

    // Get the remote reference
    let remote_branch_name = format!("origin/{}", branch_name);
    let remote_ref = repo.find_reference(&remote_branch_name)?;
    let remote_commit = repo.reference_to_annotated_commit(&remote_ref)?;

    // Merge remote commit into HEAD
    let mut head_ref = repo.head()?;
    let _head_commit = repo.reference_to_annotated_commit(&head_ref)?;

    let (analysis, _preference) = repo.merge_analysis(&[&remote_commit])?;

    if analysis.contains(git2::MergeAnalysis::ANALYSIS_UP_TO_DATE) {
        println!("Already up to date.");
    } else if analysis.contains(git2::MergeAnalysis::ANALYSIS_FASTFORWARD) {
        // Fast-forward merge
        head_ref.set_target(remote_commit.id(), "Fast-forward merge")?;
        println!("Fast-forward merge successful.");
    } else if analysis.contains(git2::MergeAnalysis::ANALYSIS_NORMAL) {
        return Err("Merge requires conflict resolution (not implemented)".into());
    }

    // Clean up merge state
    repo.cleanup_state()?;

    Ok(())
}

pub fn install_hooks() -> Result<(), Box<dyn std::error::Error>> {
    let repo = get_repo()?;
    let config = Config::load()?;

    if !config.git.hooks.pre_commit {
        return Ok(());
    }

    let git_dir = repo.path();
    let hooks_dir = git_dir.join("hooks");

    // Create hooks directory if it doesn't exist
    fs::create_dir_all(&hooks_dir)?;

    // Create pre-commit hook
    let pre_commit_hook = hooks_dir.join("pre-commit");
    let hook_content = generate_pre_commit_hook();

    fs::write(&pre_commit_hook, hook_content)?;

    // Make hook executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&pre_commit_hook)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&pre_commit_hook, perms)?;
    }

    println!("Git hooks installed successfully");
    Ok(())
}

pub fn uninstall_hooks() -> Result<(), Box<dyn std::error::Error>> {
    let repo = get_repo()?;
    let git_dir = repo.path();
    let hooks_dir = git_dir.join("hooks");

    let pre_commit_hook = hooks_dir.join("pre-commit");

    if pre_commit_hook.exists() {
        fs::remove_file(&pre_commit_hook)?;
        println!("Git hooks uninstalled successfully");
    } else {
        println!("No git hooks found to uninstall");
    }

    Ok(())
}

pub fn run_pre_commit_hook() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load()?;

    if !config.git.hooks.pre_commit {
        return Ok(());
    }

    let repo = get_repo()?;

    // Get staged files
    let statuses = repo.statuses(None)?;
    let staged_files: Vec<String> = statuses
        .iter()
        .filter(|s| {
            let status = s.status();
            status.contains(git2::Status::INDEX_NEW)
                || status.contains(git2::Status::INDEX_MODIFIED)
                || status.contains(git2::Status::INDEX_DELETED)
        })
        .filter_map(|s| s.path().map(|p| p.to_string()))
        .collect();

    if staged_files.is_empty() {
        return Ok(());
    }

    println!("Running pre-commit validation...");

    // Validate symlinks if enabled
    if config.git.hooks.validate_symlinks {
        validate_symlinks(&repo, &staged_files)?;
    }

    // Validate permissions if enabled
    if config.git.hooks.validate_permissions {
        validate_permissions(&repo, &staged_files)?;
    }

    // Validate encrypted files if enabled
    if config.git.hooks.validate_encrypted {
        validate_encrypted_files(&repo, &staged_files)?;
    }

    println!("Pre-commit validation passed");
    Ok(())
}

pub fn generate_pre_commit_hook() -> String {
    r#"#!/bin/sh
# yadm pre-commit hook

# Find yadm executable
YADM_EXEC=""
for path in "$HOME/.cargo/bin/yadm" "/usr/local/bin/yadm" "/usr/bin/yadm"; do
    if [ -x "$path" ]; then
        YADM_EXEC="$path"
        break
    fi
done

if [ -z "$YADM_EXEC" ]; then
    echo "yadm executable not found in PATH"
    exit 1
fi

# Run yadm pre-commit validation
"$YADM_EXEC" hook pre-commit

# Exit with the same code as yadm
exit $?
"#
    .to_string()
}

pub fn validate_symlinks(
    repo: &Repository,
    files: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    let workdir = repo.workdir().ok_or("Could not get repository workdir")?;

    for file in files {
        let file_path = workdir.join(file);

        if file_path.exists() && file_path.is_symlink() {
            // Check if symlink target exists
            match fs::read_link(&file_path) {
                Ok(target) => {
                    if !target.exists() {
                        return Err(format!(
                            "Broken symlink detected: {} -> {}",
                            file,
                            target.display()
                        )
                        .into());
                    }
                }
                Err(e) => {
                    return Err(format!("Invalid symlink {}: {}", file, e).into());
                }
            }
        }
    }

    Ok(())
}

pub fn validate_permissions(
    repo: &Repository,
    files: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    let workdir = repo.workdir().ok_or("Could not get repository workdir")?;

    for file in files {
        let file_path = workdir.join(file);

        if file_path.exists() && !file_path.is_symlink() {
            // Check if file is readable and writable by user
            let metadata = fs::metadata(&file_path)?;
            let permissions = metadata.permissions();

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mode = permissions.mode();

                // Check if file has appropriate permissions (not world-writable)
                if mode & 0o002 != 0 {
                    return Err(format!(
                        "File has world-writable permissions: {} (mode: {:o})",
                        file, mode
                    )
                    .into());
                }
            }
        }
    }

    Ok(())
}

pub fn validate_encrypted_files(
    repo: &Repository,
    files: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    let workdir = repo.workdir().ok_or("Could not get repository workdir")?;

    for file in files {
        if file.ends_with(".enc") {
            let file_path = workdir.join(file);

            if file_path.exists() {
                // Basic validation: check if encrypted file has minimum size
                let metadata = fs::metadata(&file_path)?;
                if metadata.len() < 32 {
                    // At least 16 bytes nonce + some data
                    return Err(format!("Encrypted file appears to be too small: {}", file).into());
                }
            }
        }
    }

    Ok(())
}

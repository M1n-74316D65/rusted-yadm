use crate::utils::folder_path;
use git2::{build::RepoBuilder, Cred, PushOptions, RemoteCallbacks, Repository};
use std::env;
use std::fs;
use std::path::Path;

pub fn clone(url: &str, path: &str, force: bool) -> Result<(), Box<dyn std::error::Error>> {
    if force && Path::new(path).exists() {
        let _ = fs::remove_dir_all(path);
    }
    Repository::clone(url, path)?;
    Ok(())
}

pub fn clone_ssh(url: &str, path: &str, force: bool) -> Result<(), Box<dyn std::error::Error>> {
    if force && Path::new(path).exists() {
        fs::remove_dir_all(path)?;
    }

    // Prepare callbacks.
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key(
            username_from_url.unwrap(),
            None,
            Path::new(&format!("{}/.ssh/id_rsa", std::env::var("HOME").unwrap())),
            None,
        )
    });

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

pub fn get_repo() -> Repository {
    let folder_path = folder_path();
    let repo_path = folder_path.as_str();

    open(repo_path)
}

pub fn push() -> Result<(), git2::Error> {
    let repo = get_repo();

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key(
            username_from_url.unwrap_or("git"),
            None,
            std::path::Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
            None,
        )
    });

    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(callbacks);

    let mut remote = repo.find_remote("origin")?;

    // Get the name of the current branch
    let head = repo.head()?;
    let branch_name = head
        .shorthand()
        .ok_or(git2::Error::from_str("Failed to get branch name"))?;

    // Push the current branch to the remote
    remote.push(
        &[&format!("refs/heads/{0}:refs/heads/{0}", branch_name)],
        Some(&mut push_options),
    )?;

    Ok(())
}

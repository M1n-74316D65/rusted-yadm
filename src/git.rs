use crate::utils::folder_path;
use git2::{build::RepoBuilder, Cred, PushOptions, RemoteCallbacks, Repository};
use std::env;
use std::fs;
use std::path::Path;

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

pub fn pull() -> Result<(), git2::Error> {
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

    let mut fetch_options = git2::FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    let mut remote = repo.find_remote("origin")?;

    // Get the name of the current branch
    let head = repo.head()?;
    let branch_name = head
        .shorthand()
        .ok_or(git2::Error::from_str("Failed to get branch name"))?;

    // Fetch the current branch from the remote
    remote.fetch(&[branch_name], Some(&mut fetch_options), None)?;

    // Get the fetched commit
    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;

    // Perform the merge
    let (merge_analysis, _merge_preference) = repo.merge_analysis(&[&fetch_commit])?;

    if merge_analysis.is_fast_forward() {
        // Fast-forward merge
        let refname = format!("refs/heads/{}", branch_name);
        let mut reference = repo.find_reference(&refname)?;
        reference.set_target(fetch_commit.id(), "Fast-forward")?;
        repo.set_head(&refname)?;
        repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
    } else {
        // Normal merge
        repo.merge(&[&fetch_commit], None, None)?;
    }

    Ok(())
}

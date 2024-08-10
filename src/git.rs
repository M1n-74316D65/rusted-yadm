use crate::utils::folder_path;
use git2::{Cred, RemoteCallbacks, Repository};
use std::env;
use std::path::Path;
pub fn clone(url: &str, path: &str) -> Repository {
    let repo = match Repository::clone(url, path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };

    repo
}

pub fn clone_ssh(url: &str, path: &str) -> Repository {
    // Prepare callbacks.
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key(
            username_from_url.unwrap(),
            None,
            Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
            None,
        )
    });

    // Prepare fetch options.
    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callbacks);

    // Prepare builder.
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);

    // Clone the project.
    let repo = builder.clone(url, Path::new(path));

    repo.unwrap()
}

pub fn open(path: &str) -> Repository {
    get_repo();
    let repo = match Repository::open(path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    repo
}

pub fn get_repo() -> Repository {
    let folder_path = folder_path();
    let repo_path = folder_path.as_str();

    open(&repo_path)
}

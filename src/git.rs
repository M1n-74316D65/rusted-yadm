use dirs::config_dir;
use git2::Repository;

pub fn clone(url: &str, path: &str) -> Repository {
    let repo = match Repository::clone(url, path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };
    repo
}

pub fn open(path: &str) -> Repository {
    get_repo();
    let repo = match Repository::open(path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    repo
}

fn folder_path() -> String {
    config_dir()
        .unwrap()
        .join("rusted-yadm")
        .join("gitrepo")
        .to_str()
        .unwrap()
        .to_string()
}

pub fn get_repo() -> Repository {
    let folder_path = folder_path();
    let repo_path = folder_path.as_str();
    open(&repo_path)
}

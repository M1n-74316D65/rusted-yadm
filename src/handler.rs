use crate::git;
use crate::utils;
use git2::{IndexAddOption, Signature};

pub fn clone(url: &str) {
    git::clone(url, utils::folder_path().as_str());
}

pub fn clone_ssh(url: &str) {
    git::clone_ssh(url, utils::folder_path().as_str());
}

pub fn add(file_path: &str) -> Result<(), git2::Error> {
    let repo = git::get_repo();

    // Get the index (staging area).
    let mut index = repo.index()?;

    // Add file to the index.
    index.add_all([&file_path].iter(), IndexAddOption::DEFAULT, None)?;
    index.write()?;

    // Write the index to the tree (create the tree object).
    index.write_tree()?;

    Ok(())
}

pub fn commit(message: &str) -> Result<(), git2::Error> {
    let (name, email) = utils::get_git_user_info();
    let repo = git::get_repo();

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

pub fn push() {
    let repo = git::get_repo();

    let mut remote = repo.find_remote("origin").unwrap();
    remote.push(&["refs/heads/master"], None).unwrap();
}

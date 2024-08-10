use crate::git;
use git2::{IndexAddOption, Signature};
use std::process::Command;
use std::str;

fn get_git_user_info() -> (String, String) {
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

pub fn clone(url: &str) {
    git::clone(url, "dwadaw");
}

pub fn add(file_path: &str) -> Result<(), git2::Error> {
    let repo = git::get_repo();

    // Get the index (staging area).
    let mut index = repo.index()?;

    // Add file to the index.
    index.add_all([&file_path].iter(), IndexAddOption::DEFAULT, None)?;
    index.write()?;

    // Write the index to the tree (create the tree object).
    let tree_id = index.write_tree()?;
    let _tree = repo.find_tree(tree_id)?;

    Ok(())
}

pub fn commit(message: &str) -> Result<(), git2::Error> {
    let (name, email) = get_git_user_info();
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

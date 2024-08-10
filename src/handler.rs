use crate::git;
use crate::utils;
use git2::Signature;
use std::fs;
use std::path::Path;

pub fn clone(url: &str, force: bool) -> Result<(), Box<dyn std::error::Error>> {
    let loading = utils::LoadingAnimation::new();
    loading.start("Cloning repository...");

    let result = git::clone(url, utils::folder_path().as_str(), force);
    loading.stop();

    match result {
        Ok(_) => {
            println!("Repo cloned successfully");

            let loading = utils::LoadingAnimation::new();
            loading.start("Copying files to home directory...");

            let copy_result = utils::copy_files_to_home();
            loading.stop();

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
    let loading = utils::LoadingAnimation::new();
    loading.start("Cloning repository via SSH...");

    let result = git::clone_ssh(url, utils::folder_path().as_str(), force);
    loading.stop();

    match result {
        Ok(_) => {
            println!("Repo cloned successfully");

            let loading = utils::LoadingAnimation::new();
            loading.start("Copying files to home directory...");

            let copy_result = utils::copy_files_to_home();
            loading.stop();

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
    let repo = git::get_repo();
    let mut index = repo.index()?;

    // Get the home directory
    let home_dir = dirs::home_dir().ok_or("Could not get home directory")?;

    // Construct the full path of the source file
    let source_path = home_dir.join(file_path.trim_start_matches("~/"));

    println!("Source path: {:?}", source_path);

    // Check if the source file exists
    if !source_path.exists() {
        return Err(format!("File does not exist: {:?}", source_path).into());
    }

    // Get the repository's working directory
    let repo_dir = repo.workdir().ok_or("Could not get repository directory")?;

    // Construct the destination path in the repository
    let dest_path = repo_dir.join(file_path.trim_start_matches("~/"));

    println!("Destination path: {:?}", dest_path);

    // Create parent directories if they don't exist
    if let Some(parent) = dest_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Copy the file from home directory to the repository
    fs::copy(&source_path, &dest_path)?;

    // Add the file to the index
    index.add_path(Path::new(file_path.trim_start_matches("~/")))?;

    // Write the index to disk
    index.write()?;

    println!("Added file: {}", file_path);
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
    match git::push() {
        Ok(_) => println!("Push successful"),
        Err(e) => eprintln!("Push failed: {}", e),
    }
}

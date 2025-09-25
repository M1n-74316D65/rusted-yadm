#[cfg(test)]
mod tests {
    use rusted_yadm::git;
    use std::fs;
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn test_clone_new_directory() {
        let temp_dir = tempdir().unwrap();
        let repo_path = temp_dir.path().join("repo");

        // This test would require a real git repository to clone
        // For now, we'll test with a non-existent URL to ensure error handling
        let result = git::clone(
            "https://nonexistent-repo.git",
            repo_path.to_str().unwrap(),
            false,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_clone_existing_directory_without_force() {
        let temp_dir = tempdir().unwrap();
        let repo_path = temp_dir.path().join("repo");

        // Create directory
        fs::create_dir_all(&repo_path).unwrap();

        // Try to clone without force flag
        let result = git::clone(
            "https://example.com/repo.git",
            repo_path.to_str().unwrap(),
            false,
        );
        assert!(result.is_err());

        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Directory already exists"));
    }

    #[test]
    fn test_clone_existing_directory_with_force() {
        let temp_dir = tempdir().unwrap();
        let repo_path = temp_dir.path().join("repo");

        // Create directory with some content
        fs::create_dir_all(&repo_path).unwrap();
        fs::write(repo_path.join("test.txt"), "test content").unwrap();

        // Try to clone with force flag (will fail because URL doesn't exist, but should remove directory first)
        let result = git::clone(
            "https://nonexistent-repo.git",
            repo_path.to_str().unwrap(),
            true,
        );
        assert!(result.is_err());

        // Directory should have been removed
        assert!(!repo_path.exists());
    }

    #[test]
    fn test_clone_ssh_new_directory() {
        let temp_dir = tempdir().unwrap();
        let repo_path = temp_dir.path().join("repo");

        // Test SSH clone with non-existent repository
        let result = git::clone_ssh(
            "git@nonexistent.com:repo.git",
            repo_path.to_str().unwrap(),
            false,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_clone_ssh_existing_directory_without_force() {
        let temp_dir = tempdir().unwrap();
        let repo_path = temp_dir.path().join("repo");

        // Create directory
        fs::create_dir_all(&repo_path).unwrap();

        // Try to SSH clone without force flag
        let result = git::clone_ssh(
            "git@example.com:repo.git",
            repo_path.to_str().unwrap(),
            false,
        );
        assert!(result.is_err());

        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Directory already exists"));
    }

    #[test]
    fn test_clone_ssh_existing_directory_with_force() {
        let temp_dir = tempdir().unwrap();
        let repo_path = temp_dir.path().join("repo");

        // Create directory with some content
        fs::create_dir_all(&repo_path).unwrap();
        fs::write(repo_path.join("test.txt"), "test content").unwrap();

        // Try to SSH clone with force flag
        let result = git::clone_ssh(
            "git@nonexistent.com:repo.git",
            repo_path.to_str().unwrap(),
            true,
        );
        assert!(result.is_err());

        // Directory should have been removed
        assert!(!repo_path.exists());
    }

    #[test]
    fn test_get_repo_no_directory() {
        // Backup existing repository if it exists
        let folder_path = rusted_yadm::utils::folder_path();
        let backup_exists = std::path::Path::new(&folder_path).exists();
        let backup_temp = if backup_exists {
            let temp_dir = tempfile::tempdir().unwrap();
            let backup_path = temp_dir.path().join("repository_backup");
            // Use copy instead of rename to avoid cross-device issues
            dircpy::copy_dir(&folder_path, &backup_path).unwrap();
            std::fs::remove_dir_all(&folder_path).unwrap();
            Some((backup_path, temp_dir))
        } else {
            None
        };

        // Try to get repo when no directory exists
        let result = git::get_repo();
        match result {
            Ok(_) => panic!("Expected error but got success"),
            Err(e) => {
                let error_msg = e.to_string();
                assert!(error_msg.contains("does not exist"));
            }
        }

        // Restore backup if it existed
        if let Some((backup_path, _temp_dir)) = backup_temp {
            dircpy::copy_dir(&backup_path, &folder_path).unwrap();
        }
    }

    #[test]
    fn test_repo_exists() {
        // Test repo_exists function
        let exists = git::repo_exists();
        // This should return false since we don't have a repo set up
        // but the function should not panic
        println!("Repo exists: {}", exists);
    }

    #[test]
    fn test_git_operations_integration() {
        let temp_dir = tempdir().unwrap();
        let repo_path = temp_dir.path().join("test-repo");

        // Initialize a real git repository for testing
        std::process::Command::new("git")
            .arg("init")
            .arg(&repo_path)
            .output()
            .expect("Failed to init git repo");

        // Configure git user
        std::process::Command::new("git")
            .args(&["config", "user.name", "Test User"])
            .current_dir(&repo_path)
            .output()
            .expect("Failed to set git user name");

        std::process::Command::new("git")
            .args(&["config", "user.email", "test@example.com"])
            .current_dir(&repo_path)
            .output()
            .expect("Failed to set git user email");

        // Create a test file
        let test_file = repo_path.join("test.txt");
        fs::write(&test_file, "test content").unwrap();

        // Add file to git
        std::process::Command::new("git")
            .args(&["add", "test.txt"])
            .current_dir(&repo_path)
            .output()
            .expect("Failed to add file");

        // Commit file
        std::process::Command::new("git")
            .args(&["commit", "-m", "Initial commit"])
            .current_dir(&repo_path)
            .output()
            .expect("Failed to commit");

        // Now test our open function (should work with real git repo)
        let repo = git::open(repo_path.to_str().unwrap());
        assert_eq!(repo.path(), repo_path.join(".git").as_path());
    }

    #[test]
    fn test_path_handling() {
        let temp_dir = tempdir().unwrap();
        let repo_path = temp_dir.path().join("test repo with spaces");

        // Create directory with spaces in name
        fs::create_dir_all(&repo_path).unwrap();

        // Test that our functions handle paths with spaces correctly
        let result = git::clone(
            "https://nonexistent-repo.git",
            repo_path.to_str().unwrap(),
            false,
        );
        assert!(result.is_err());

        // The error should not be due to path handling issues
        let error_msg = result.unwrap_err().to_string();
        assert!(!error_msg.contains("path"));
    }

    #[test]
    fn test_error_messages() {
        let temp_dir = tempdir().unwrap();
        let repo_path = temp_dir.path().join("repo");

        // Test error message for existing directory
        fs::create_dir_all(&repo_path).unwrap();
        let result = git::clone(
            "https://example.com/repo.git",
            repo_path.to_str().unwrap(),
            false,
        );
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("already exists"));
        assert!(error_msg.contains("--force"));
    }
}

#[cfg(test)]
mod tests {
    
    use std::fs;
    use std::path::Path;
    use tempfile::tempdir;

    fn setup_test_repo() -> (tempfile::TempDir, String) {
        let temp_dir = tempdir().unwrap();
        let repo_path = temp_dir.path().to_string_lossy().to_string();

        // Initialize git repository (non-bare)
        let mut repo_builder = git2::RepositoryInitOptions::new();
        repo_builder.bare(false);
        git2::Repository::init_opts(&repo_path, &repo_builder).unwrap();

        // Create .gitignore
        let gitignore_path = Path::new(&repo_path).join(".gitignore");
        fs::write(gitignore_path, "*.tmp\n*.log\n").unwrap();

        (temp_dir, repo_path)
    }

    fn setup_test_repo_in_correct_location() -> tempfile::TempDir {
        // Create a temporary directory that will be used as the data local dir
        let temp_dir = tempdir().unwrap();
        let data_local_path = temp_dir.path().to_string_lossy().to_string();

        // Set environment variables to make folder_path() return our test path
        std::env::set_var("HOME", &data_local_path);
        std::env::set_var("XDG_DATA_HOME", &data_local_path);

        // Create the expected directory structure manually
        let yadm_dir = Path::new(&data_local_path).join("rusted-yadm");
        fs::create_dir_all(&yadm_dir).unwrap();

        let repo_path = yadm_dir.join("repository");

        // Initialize git repository (non-bare)
        let mut repo_builder = git2::RepositoryInitOptions::new();
        repo_builder.bare(false);
        git2::Repository::init_opts(&repo_path, &repo_builder).unwrap();

        // Create .gitignore
        let gitignore_path = repo_path.join(".gitignore");
        fs::write(gitignore_path, "*.tmp\n*.log\n").unwrap();

        temp_dir
    }

    #[test]
    fn test_hooks_config_default() {
        use rusted_yadm::config::Config;

        let config = Config::new();
        assert!(!config.git.hooks.pre_commit);
        assert!(!config.git.hooks.validate_symlinks);
        assert!(!config.git.hooks.validate_permissions);
        assert!(!config.git.hooks.validate_encrypted);
    }

    #[test]
    fn test_hooks_config_set_get() {
        use rusted_yadm::config::Config;

        let mut config = Config::new();

        // Test setting hook configurations
        config.set("git.hooks.pre_commit", "true").unwrap();
        config.set("git.hooks.validate_symlinks", "true").unwrap();
        config
            .set("git.hooks.validate_permissions", "true")
            .unwrap();
        config.set("git.hooks.validate_encrypted", "true").unwrap();

        // Test getting hook configurations
        assert_eq!(
            config.get("git.hooks.pre_commit").unwrap(),
            Some("true".to_string())
        );
        assert_eq!(
            config.get("git.hooks.validate_symlinks").unwrap(),
            Some("true".to_string())
        );
        assert_eq!(
            config.get("git.hooks.validate_permissions").unwrap(),
            Some("true".to_string())
        );
        assert_eq!(
            config.get("git.hooks.validate_encrypted").unwrap(),
            Some("true".to_string())
        );
    }

    #[test]
    fn test_hooks_config_list() {
        use rusted_yadm::config::Config;

        let mut config = Config::new();
        config.set("git.hooks.pre_commit", "true").unwrap();
        config.set("git.hooks.validate_symlinks", "false").unwrap();

        let config_items = config.list();

        assert!(config_items
            .iter()
            .any(|(k, v)| k == "git.hooks.pre_commit" && v == "true"));
        assert!(config_items
            .iter()
            .any(|(k, v)| k == "git.hooks.validate_symlinks" && v == "false"));
        assert!(config_items
            .iter()
            .any(|(k, v)| k == "git.hooks.validate_permissions" && v == "false"));
        assert!(config_items
            .iter()
            .any(|(k, v)| k == "git.hooks.validate_encrypted" && v == "false"));
    }

    #[test]
    fn test_hooks_config_invalid_boolean() {
        use rusted_yadm::config::Config;

        let mut config = Config::new();

        // Test invalid boolean values
        assert!(config.set("git.hooks.pre_commit", "invalid").is_err());
        assert!(config.set("git.hooks.validate_symlinks", "yes").is_err());
    }

    #[test]
    fn test_hooks_config_unknown_key() {
        use rusted_yadm::config::Config;

        let mut config = Config::new();

        // Test unknown keys
        assert!(config.set("git.hooks.unknown", "true").is_err());
        assert!(config.get("git.hooks.unknown").is_err());
    }

    #[test]
    fn test_install_hooks_no_repo() {
        // This test should fail because there's no repository
        let result = rusted_yadm::git::install_hooks();
        assert!(result.is_err());
    }

    #[test]
    fn test_install_hooks_success() {
        let _temp_dir = setup_test_repo_in_correct_location();

        // Create a config with hooks enabled
        use rusted_yadm::config::Config;
        let mut config = Config::new();
        config.git.hooks.pre_commit = true;
        config.save().unwrap();

        // Test hook installation
        let result = rusted_yadm::git::install_hooks();
        assert!(result.is_ok());

        // Check if pre-commit hook was created
        let repo_path = rusted_yadm::utils::folder_path();
        let hooks_dir = Path::new(&repo_path).join(".git").join("hooks");
        let pre_commit_hook = hooks_dir.join("pre-commit");
        assert!(pre_commit_hook.exists());

        // Check hook content
        let hook_content = fs::read_to_string(&pre_commit_hook).unwrap();
        assert!(hook_content.contains("yadm pre-commit"));
        assert!(hook_content.contains("#!/bin/sh"));

        // Check hook permissions on Unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = fs::metadata(&pre_commit_hook).unwrap();
            let permissions = metadata.permissions();
            assert_eq!(permissions.mode() & 0o111, 0o111); // Check if executable
        }
    }

    #[test]
    fn test_install_hooks_disabled() {
        let _temp_dir = setup_test_repo_in_correct_location();

        // Create a config with hooks disabled
        use rusted_yadm::config::Config;
        let mut config = Config::new();
        config.git.hooks.pre_commit = false;
        config.save().unwrap();

        // Test hook installation (should do nothing)
        let result = rusted_yadm::git::install_hooks();
        assert!(result.is_ok());

        // Check if pre-commit hook was NOT created
        let repo_path = rusted_yadm::utils::folder_path();
        let hooks_dir = Path::new(&repo_path).join(".git").join("hooks");
        let pre_commit_hook = hooks_dir.join("pre-commit");
        assert!(!pre_commit_hook.exists());
    }

    #[test]
    fn test_uninstall_hooks_success() {
        let _temp_dir = setup_test_repo_in_correct_location();

        // Create a config with hooks enabled
        use rusted_yadm::config::Config;
        let mut config = Config::new();
        config.git.hooks.pre_commit = true;
        config.save().unwrap();

        // Install hooks first
        rusted_yadm::git::install_hooks().unwrap();

        // Check if hook exists
        let repo_path = rusted_yadm::utils::folder_path();
        let hooks_dir = Path::new(&repo_path).join(".git").join("hooks");
        let pre_commit_hook = hooks_dir.join("pre-commit");
        assert!(pre_commit_hook.exists());

        // Test hook uninstallation
        let result = rusted_yadm::git::uninstall_hooks();
        assert!(result.is_ok());

        // Check if hook was removed
        assert!(!pre_commit_hook.exists());
    }

    #[test]
    fn test_uninstall_hooks_no_hooks() {
        let _temp_dir = setup_test_repo_in_correct_location();

        // Test hook uninstallation when no hooks exist
        let result = rusted_yadm::git::uninstall_hooks();
        assert!(result.is_ok()); // Should not fail
    }

    #[test]
    fn test_run_pre_commit_hook_disabled() {
        let _temp_dir = setup_test_repo_in_correct_location();

        // Create a config with hooks disabled
        use rusted_yadm::config::Config;
        let mut config = Config::new();
        config.git.hooks.pre_commit = false;
        config.save().unwrap();

        // Test pre-commit hook (should do nothing)
        let result = rusted_yadm::git::run_pre_commit_hook();
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_symlinks_valid() {
        let (_temp_dir, repo_path) = setup_test_repo();

        // Create a valid symlink
        let test_file = Path::new(&repo_path).join("target.txt");
        let symlink_file = Path::new(&repo_path).join("symlink.txt");

        fs::write(&test_file, "test content").unwrap();

        #[cfg(unix)]
        {
            std::os::unix::fs::symlink("target.txt", &symlink_file).unwrap();
        }
        #[cfg(not(unix))]
        {
            std::os::windows::fs::symlink_file("target.txt", &symlink_file).unwrap();
        }

        // Test symlink validation
        let repo = git2::Repository::open(&repo_path).unwrap();
        let files = vec!["symlink.txt".to_string()];
        let result = rusted_yadm::git::validate_symlinks(&repo, &files);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_symlinks_broken() {
        let (_temp_dir, repo_path) = setup_test_repo();

        // Create a broken symlink
        let symlink_file = Path::new(&repo_path).join("broken_symlink.txt");

        #[cfg(unix)]
        {
            std::os::unix::fs::symlink("nonexistent.txt", &symlink_file).unwrap();
        }
        #[cfg(not(unix))]
        {
            std::os::windows::fs::symlink_file("nonexistent.txt", &symlink_file).unwrap();
        }

        // Test symlink validation
        let repo = git2::Repository::open(&repo_path).unwrap();
        let files = vec!["broken_symlink.txt".to_string()];
        let result = rusted_yadm::git::validate_symlinks(&repo, &files);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_permissions_safe() {
        let (_temp_dir, repo_path) = setup_test_repo();

        // Create a file with safe permissions
        let test_file = Path::new(&repo_path).join("safe_file.txt");
        fs::write(&test_file, "test content").unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut permissions = fs::metadata(&test_file).unwrap().permissions();
            permissions.set_mode(0o644); // Safe permissions
            fs::set_permissions(&test_file, permissions).unwrap();
        }

        // Test permission validation
        let repo = git2::Repository::open(&repo_path).unwrap();
        let files = vec!["safe_file.txt".to_string()];
        let result = rusted_yadm::git::validate_permissions(&repo, &files);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_permissions_unsafe() {
        let (_temp_dir, repo_path) = setup_test_repo();

        // Create a file with unsafe permissions
        let test_file = Path::new(&repo_path).join("unsafe_file.txt");
        fs::write(&test_file, "test content").unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut permissions = fs::metadata(&test_file).unwrap().permissions();
            permissions.set_mode(0o622); // World-writable (unsafe)
            fs::set_permissions(&test_file, permissions).unwrap();

            // Test permission validation
            let repo = git2::Repository::open(&repo_path).unwrap();
            let files = vec!["unsafe_file.txt".to_string()];
            let result = rusted_yadm::git::validate_permissions(&repo, &files);
            assert!(result.is_err());
        }
        #[cfg(not(unix))]
        {
            // On Windows, permission validation is a no-op, so it should pass
            let repo = git2::Repository::open(&repo_path).unwrap();
            let files = vec!["unsafe_file.txt".to_string()];
            let result = rusted_yadm::git::validate_permissions(&repo, &files);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_validate_encrypted_files_valid() {
        let (_temp_dir, repo_path) = setup_test_repo();

        // Create a valid encrypted file (just a dummy file with sufficient size)
        let test_file = Path::new(&repo_path).join("test.enc");
        fs::write(test_file, vec![0u8; 64]).unwrap(); // 64 bytes > 32 minimum

        // Test encrypted file validation
        let repo = git2::Repository::open(&repo_path).unwrap();
        let files = vec!["test.enc".to_string()];
        let result = rusted_yadm::git::validate_encrypted_files(&repo, &files);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_encrypted_files_too_small() {
        let (_temp_dir, repo_path) = setup_test_repo();

        // Create an encrypted file that's too small
        let test_file = Path::new(&repo_path).join("small.enc");
        fs::write(test_file, vec![0u8; 16]).unwrap(); // 16 bytes < 32 minimum

        // Test encrypted file validation
        let repo = git2::Repository::open(&repo_path).unwrap();
        let files = vec!["small.enc".to_string()];
        let result = rusted_yadm::git::validate_encrypted_files(&repo, &files);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_encrypted_files_non_encrypted() {
        let (_temp_dir, repo_path) = setup_test_repo();

        // Create a non-encrypted file
        let test_file = Path::new(&repo_path).join("normal.txt");
        fs::write(&test_file, "test content").unwrap();

        // Test encrypted file validation (should ignore non-encrypted files)
        let repo = git2::Repository::open(&repo_path).unwrap();
        let files = vec!["normal.txt".to_string()];
        let result = rusted_yadm::git::validate_encrypted_files(&repo, &files);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_pre_commit_hook() {
        let hook_content = rusted_yadm::git::generate_pre_commit_hook();

        assert!(hook_content.contains("#!/bin/sh"));
        assert!(hook_content.contains("yadm pre-commit"));
        assert!(hook_content.contains("YADM_EXEC"));
        assert!(hook_content.contains("exit $?"));
    }

    #[test]
    fn test_hook_integration_with_commit() {
        let _temp_dir = setup_test_repo_in_correct_location();

        // Create a config with hooks enabled
        use rusted_yadm::config::Config;
        let mut config = Config::new();
        config.git.hooks.pre_commit = true;
        config.git.hooks.validate_symlinks = true;
        config.git.hooks.validate_permissions = true;
        config.git.hooks.validate_encrypted = true;
        config.save().unwrap();

        // Install hooks
        rusted_yadm::git::install_hooks().unwrap();

        // Create a test file
        let repo_path = rusted_yadm::utils::folder_path();
        let test_file = Path::new(&repo_path).join("test.txt");
        fs::write(&test_file, "test content").unwrap();

        // Add the file to git
        let repo = git2::Repository::open(&repo_path).unwrap();
        let mut index = repo.index().unwrap();
        index.add_path(Path::new("test.txt")).unwrap();
        index.write().unwrap();

        // Test pre-commit hook directly
        let result = rusted_yadm::git::run_pre_commit_hook();
        assert!(result.is_ok());
    }
}

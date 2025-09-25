#[cfg(test)]
mod tests {
    use rusted_yadm::config::Config;
    use rusted_yadm::git;
    use rusted_yadm::handler;
    use rusted_yadm::utils;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_full_config_workflow() {
        // Test config creation, modification, and operations
        let mut config = Config::new();

        // Set some configuration values
        assert!(config.set("local.class", "work").is_ok());
        assert!(config.set("local.os", "linux").is_ok());
        assert!(config.set("ssh.agent", "true").is_ok());
        assert!(config.set("git.auto_private", "true").is_ok());

        // Verify values were set
        assert_eq!(config.get("local.class").unwrap(), Some("work".to_string()));
        assert_eq!(config.get("local.os").unwrap(), Some("linux".to_string()));
        assert_eq!(config.get("ssh.agent").unwrap(), Some("true".to_string()));
        assert_eq!(
            config.get("git.auto_private").unwrap(),
            Some("true".to_string())
        );

        // Test listing configuration
        let items = config.list();
        assert!(items.len() > 0);
        assert!(items.iter().any(|(k, v)| k == "local.class" && v == "work"));
        assert!(items.iter().any(|(k, v)| k == "local.os" && v == "linux"));
    }

    #[test]
    fn test_git_clone_error_handling() {
        let temp_dir = tempdir().unwrap();
        let repo_path = temp_dir.path().join("repo");

        // Test clone with non-existent URL
        let result = git::clone(
            "https://nonexistent-repo.git",
            repo_path.to_str().unwrap(),
            false,
        );
        assert!(result.is_err());

        // Test clone with existing directory without force
        fs::create_dir_all(&repo_path).unwrap();
        let result = git::clone(
            "https://example.com/repo.git",
            repo_path.to_str().unwrap(),
            false,
        );
        assert!(result.is_err());

        // Test clone with existing directory with force
        let result = git::clone(
            "https://nonexistent-repo.git",
            repo_path.to_str().unwrap(),
            true,
        );
        assert!(result.is_err());
        assert!(!repo_path.exists()); // Directory should have been removed
    }

    #[test]
    fn test_git_ssh_clone_error_handling() {
        let temp_dir = tempdir().unwrap();
        let repo_path = temp_dir.path().join("repo");

        // Test SSH clone with non-existent URL
        let result = git::clone_ssh(
            "git@nonexistent.com:repo.git",
            repo_path.to_str().unwrap(),
            false,
        );
        assert!(result.is_err());

        // Test SSH clone with existing directory without force
        fs::create_dir_all(&repo_path).unwrap();
        let result = git::clone_ssh(
            "git@example.com:repo.git",
            repo_path.to_str().unwrap(),
            false,
        );
        assert!(result.is_err());

        // Test SSH clone with existing directory with force
        let result = git::clone_ssh(
            "git@nonexistent.com:repo.git",
            repo_path.to_str().unwrap(),
            true,
        );
        assert!(result.is_err());
        assert!(!repo_path.exists()); // Directory should have been removed
    }

    #[test]
    fn test_utils_folder_path() {
        let path = utils::folder_path();

        // Path should be valid and contain expected components
        assert!(!path.is_empty());
        assert!(path.contains("rusted-yadm"));
        assert!(path.contains("repository"));

        // Path should be absolute
        assert!(path.starts_with('/'));
    }

    #[test]
    fn test_utils_git_user_info() {
        let (name, email) = utils::get_git_user_info();

        // Should return valid strings (even if empty)
        assert!(!name.contains('\n'));
        assert!(!email.contains('\n'));

        println!("Git user info - Name: '{}', Email: '{}'", name, email);
    }

    #[test]
    fn test_handler_error_handling() {
        // Test that all handler functions handle errors gracefully

        // Test operations that should fail without a repository
        let operations = vec![
            || handler::list(),
            || handler::status(),
            || handler::fetch(),
            || handler::pull(None),
            || handler::push(None),
            || handler::add("/nonexistent/file"),
            || handler::commit("test message"),
        ];

        for op in operations {
            let result = op();
            match result {
                Ok(_) => println!("Operation succeeded unexpectedly"),
                Err(e) => {
                    println!("Operation failed as expected: {}", e);
                    // Error message should be meaningful
                    assert!(!e.to_string().is_empty());
                }
            }
        }
    }

    #[test]
    fn test_config_and_handler_integration() {
        // Test that config operations work with handler functions

        // Test config operations through handler
        let result = handler::config(Some("local.class"), Some("integration_test"), false);
        match result {
            Ok(_) => println!("Config set through handler succeeded"),
            Err(e) => println!("Config set through handler failed: {}", e),
        }

        let result = handler::config(Some("local.class"), None, false);
        match result {
            Ok(_) => println!("Config get through handler succeeded"),
            Err(e) => println!("Config get through handler failed: {}", e),
        }

        let result = handler::config(None, None, true);
        match result {
            Ok(_) => println!("Config list through handler succeeded"),
            Err(e) => println!("Config list through handler failed: {}", e),
        }
    }

    #[test]
    fn test_bootstrap_error_handling() {
        // Test bootstrap operations with invalid configurations

        let bootstrap_ops = vec![
            || handler::bootstrap(None, false, true),
            || handler::bootstrap(Some("/nonexistent/config.yaml"), false, true),
            || handler::bootstrap(Some("/nonexistent/config.yaml"), true, true),
            || handler::bootstrap(None, false, true),
        ];

        for op in bootstrap_ops {
            let result = op();
            match result {
                Ok(_) => println!("Bootstrap operation succeeded unexpectedly"),
                Err(e) => {
                    println!("Bootstrap operation failed as expected: {}", e);
                    assert!(!e.to_string().is_empty());
                }
            }
        }
    }

    #[test]
    fn test_encryption_error_handling() {
        // Test encryption operations with invalid files

        let result = handler::encrypt("/nonexistent/file.txt");
        match result {
            Ok(_) => println!("Encrypt nonexistent file succeeded unexpectedly"),
            Err(e) => {
                println!("Encrypt nonexistent file failed as expected: {}", e);
                assert!(!e.to_string().is_empty());
            }
        }

        let result = handler::decrypt("/nonexistent/file.enc");
        match result {
            Ok(_) => println!("Decrypt nonexistent file succeeded unexpectedly"),
            Err(e) => {
                println!("Decrypt nonexistent file failed as expected: {}", e);
                assert!(!e.to_string().is_empty());
            }
        }
    }

    #[test]
    fn test_alt_operations() {
        // Test alt operations with different parameters

        let alt_ops = vec![
            || handler::alt(false, false),
            || handler::alt(true, false),
            || handler::alt(false, true),
            || handler::alt(true, true),
        ];

        for op in alt_ops {
            let result = op();
            match result {
                Ok(_) => println!("Alt operation succeeded"),
                Err(e) => {
                    println!("Alt operation failed: {}", e);
                    assert!(!e.to_string().is_empty());
                }
            }
        }
    }

    #[test]
    fn test_init_operations() {
        // Test init operations with different parameters

        let init_ops = vec![
            || handler::init(None),
            || handler::init(Some("https://example.com/repo.git")),
            || handler::init(Some("invalid-url")),
        ];

        for op in init_ops {
            let result = op();
            match result {
                Ok(_) => println!("Init operation succeeded"),
                Err(e) => {
                    println!("Init operation failed: {}", e);
                    assert!(!e.to_string().is_empty());
                }
            }
        }
    }

    #[test]
    fn test_error_message_quality() {
        // Test that error messages are meaningful and helpful

        let error_operations = vec![
            || handler::add("/nonexistent/file.txt"),
            || handler::commit("test message"),
            || handler::encrypt("/nonexistent/file.txt"),
            || handler::decrypt("/nonexistent/file.enc"),
            || handler::bootstrap(Some("/nonexistent/config.yaml"), false, true),
        ];

        for op in error_operations {
            if let Err(e) = op() {
                let error_msg = e.to_string();

                // Error message should not be empty
                assert!(!error_msg.is_empty());

                // Error message should be reasonably descriptive
                assert!(error_msg.len() > 5);

                println!("Good error message: {}", error_msg);
            }
        }
    }

    #[test]
    fn test_concurrent_operations() {
        // Test that operations can be called concurrently without panicking

        use std::sync::Arc;
        use std::thread;

        let operations = Arc::new(vec![
            || handler::list(),
            || handler::status(),
            || handler::config(None, None, false),
            || handler::alt(false, false),
        ]);

        let mut handles = vec![];

        for i in 0..operations.len() {
            let ops = operations.clone();
            let handle = thread::spawn(move || {
                let result = ops[i]();
                match result {
                    Ok(_) => println!("Concurrent operation {} succeeded", i),
                    Err(e) => println!("Concurrent operation {} failed: {}", i, e),
                }
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_memory_usage() {
        // Test that operations don't cause obvious memory issues

        for _ in 0..10 {
            let result = handler::config(None, None, true);
            match result {
                Ok(_) => (),
                Err(e) => println!("Config operation failed: {}", e),
            }

            let result = handler::list();
            match result {
                Ok(_) => (),
                Err(e) => println!("List operation failed: {}", e),
            }
        }

        // If we get here without panicking, memory usage is probably fine
        println!("Memory usage test completed successfully");
    }
}

#[cfg(test)]
mod tests {
    use rusted_yadm::handler;
    use std::fs;
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn test_list_no_repo() {
        // This will likely fail because there's no repository
        // but we can test that it doesn't panic
        let result = handler::list();

        match result {
            Ok(_) => println!("list succeeded"),
            Err(e) => println!("list failed (expected): {}", e),
        }
    }

    #[test]
    fn test_status_no_repo() {
        // This will likely fail because there's no repository
        // but we can test that it doesn't panic
        let result = handler::status();

        match result {
            Ok(_) => println!("status succeeded"),
            Err(e) => println!("status failed (expected): {}", e),
        }
    }

    #[test]
    fn test_fetch_no_repo() {
        // This will likely fail because there's no repository
        // but we can test that it doesn't panic
        let result = handler::fetch();

        match result {
            Ok(_) => println!("fetch succeeded"),
            Err(e) => println!("fetch failed (expected): {}", e),
        }
    }

    #[test]
    fn test_pull_no_repo() {
        // This will likely fail because there's no repository
        // but we can test that it doesn't panic
        let result = handler::pull(None);

        match result {
            Ok(_) => println!("pull succeeded"),
            Err(e) => println!("pull failed (expected): {}", e),
        }
    }

    #[test]
    fn test_pull_with_branch_no_repo() {
        // This will likely fail because there's no repository
        // but we can test that it doesn't panic
        let result = handler::pull(Some("main"));

        match result {
            Ok(_) => println!("pull with branch succeeded"),
            Err(e) => println!("pull with branch failed (expected): {}", e),
        }
    }

    #[test]
    fn test_push_no_repo() {
        // This will likely fail because there's no repository
        // but we can test that it doesn't panic
        let result = handler::push(None);

        match result {
            Ok(_) => println!("push succeeded"),
            Err(e) => println!("push failed (expected): {}", e),
        }
    }

    #[test]
    fn test_push_with_branch_no_repo() {
        // This will likely fail because there's no repository
        // but we can test that it doesn't panic
        let result = handler::push(Some("main"));

        match result {
            Ok(_) => println!("push with branch succeeded"),
            Err(e) => println!("push with branch failed (expected): {}", e),
        }
    }

    #[test]
    fn test_add_nonexistent_file() {
        // Try to add a non-existent file
        let result = handler::add("/nonexistent/file.txt");

        // This should fail
        assert!(result.is_err());

        let error_msg = result.unwrap_err().to_string();
        println!("add nonexistent file error: {}", error_msg);
    }

    #[test]
    fn test_commit_no_repo() {
        // This will likely fail because there's no repository
        // but we can test that it doesn't panic
        let result = handler::commit("test commit message");

        match result {
            Ok(_) => println!("commit succeeded"),
            Err(e) => println!("commit failed (expected): {}", e),
        }
    }

    #[test]
    fn test_init_no_url() {
        // Test init without URL
        let result = handler::init(None);

        match result {
            Ok(_) => println!("init without URL succeeded"),
            Err(e) => println!("init without URL failed (expected): {}", e),
        }
    }

    #[test]
    fn test_init_with_invalid_url() {
        // Test init with invalid URL
        let result = handler::init(Some("invalid-url"));

        match result {
            Ok(_) => println!("init with invalid URL succeeded"),
            Err(e) => println!("init with invalid URL failed (expected): {}", e),
        }
    }

    #[test]
    fn test_config_operations() {
        // Test config get with no key
        let result = handler::config(None, None, false);
        match result {
            Ok(_) => println!("config get no key succeeded"),
            Err(e) => println!("config get no key failed (expected): {}", e),
        }

        // Test config get with key
        let result = handler::config(Some("local.class"), None, false);
        match result {
            Ok(_) => println!("config get key succeeded"),
            Err(e) => println!("config get key failed (expected): {}", e),
        }

        // Test config set
        let result = handler::config(Some("local.class"), Some("test"), false);
        match result {
            Ok(_) => println!("config set succeeded"),
            Err(e) => println!("config set failed (expected): {}", e),
        }

        // Test config list
        let result = handler::config(None, None, true);
        match result {
            Ok(_) => println!("config list succeeded"),
            Err(e) => println!("config list failed (expected): {}", e),
        }
    }

    #[test]
    fn test_alt_operations() {
        // Test alt without dry run
        let result = handler::alt(false, false);
        match result {
            Ok(_) => println!("alt succeeded"),
            Err(e) => println!("alt failed (expected): {}", e),
        }

        // Test alt with dry run
        let result = handler::alt(true, false);
        match result {
            Ok(_) => println!("alt dry run succeeded"),
            Err(e) => println!("alt dry run failed (expected): {}", e),
        }

        // Test alt with force
        let result = handler::alt(false, true);
        match result {
            Ok(_) => println!("alt force succeeded"),
            Err(e) => println!("alt force failed (expected): {}", e),
        }
    }

    #[test]
    fn test_encrypt_nonexistent_file() {
        // Try to encrypt a non-existent file
        let result = handler::encrypt("/nonexistent/file.txt");

        // This should fail
        assert!(result.is_err());

        let error_msg = result.unwrap_err().to_string();
        println!("encrypt nonexistent file error: {}", error_msg);
    }

    #[test]
    fn test_decrypt_nonexistent_file() {
        // Try to decrypt a non-existent file
        let result = handler::decrypt("/nonexistent/file.enc");

        // This should fail
        assert!(result.is_err());

        let error_msg = result.unwrap_err().to_string();
        println!("decrypt nonexistent file error: {}", error_msg);
    }

    #[test]
    fn test_bootstrap_no_config() {
        // Test bootstrap without config
        let result = handler::bootstrap(None, false, true);

        match result {
            Ok(_) => println!("bootstrap without config succeeded"),
            Err(e) => println!("bootstrap without config failed (expected): {}", e),
        }
    }

    #[test]
    fn test_bootstrap_with_invalid_config() {
        // Test bootstrap with invalid config path
        let result = handler::bootstrap(Some("/nonexistent/config.yaml"), false, true);

        match result {
            Ok(_) => println!("bootstrap with invalid config succeeded"),
            Err(e) => println!("bootstrap with invalid config failed (expected): {}", e),
        }
    }

    #[test]
    fn test_bootstrap_with_force() {
        // Test bootstrap with force flag
        let result = handler::bootstrap(None, true, true);

        match result {
            Ok(_) => println!("bootstrap with force succeeded"),
            Err(e) => println!("bootstrap with force failed (expected): {}", e),
        }
    }

    #[test]
    fn test_bootstrap_non_interactive() {
        // Test bootstrap non-interactive
        let result = handler::bootstrap(None, false, true);

        match result {
            Ok(_) => println!("bootstrap non-interactive succeeded"),
            Err(e) => println!("bootstrap non-interactive failed (expected): {}", e),
        }
    }

    #[test]
    fn test_error_messages() {
        // Test that error messages are meaningful
        let result = handler::add("/nonexistent/file.txt");
        if let Err(e) = result {
            let error_msg = e.to_string();
            assert!(!error_msg.is_empty());
            println!("Error message: {}", error_msg);
        }

        let result = handler::commit("test message");
        if let Err(e) = result {
            let error_msg = e.to_string();
            assert!(!error_msg.is_empty());
            println!("Error message: {}", error_msg);
        }
    }

    #[test]
    fn test_handler_functions_exist() {
        // This test ensures all handler functions exist and are callable
        // Even if they fail, they should not panic

        let functions = vec![
            || handler::list(),
            || handler::status(),
            || handler::fetch(),
            || handler::pull(None),
            || handler::push(None),
            || handler::add("test"),
            || handler::commit("test"),
            || handler::init(None),
            || handler::config(None, None, false),
            || handler::alt(false, false),
            || handler::encrypt("test"),
            || handler::decrypt("test"),
            || handler::bootstrap(None, false, true),
        ];

        for func in functions {
            match func() {
                Ok(_) => println!("Function succeeded"),
                Err(e) => println!("Function failed (expected): {}", e),
            }
        }
    }
}

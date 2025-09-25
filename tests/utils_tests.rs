#[cfg(test)]
mod tests {
    use rusted_yadm::utils;
    use std::fs;
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn test_get_git_user_info() {
        let (name, email) = utils::get_git_user_info();

        // Name and email should not be empty if git is configured
        // If git is not configured, they might be empty, which is also valid
        println!("Git user: {}, email: {}", name, email);

        // At minimum, they should be valid strings
        assert!(!name.contains('\n'));
        assert!(!email.contains('\n'));
    }

    #[test]
    fn test_folder_path() {
        let path = utils::folder_path();

        // Path should not be empty
        assert!(!path.is_empty());

        // Path should be a valid string
        assert!(Path::new(&path).is_absolute());

        // Path should contain rusted-yadm components
        assert!(path.contains("rusted-yadm"));
        assert!(path.contains("repository"));
    }

    #[test]
    fn test_data_dir_path() {
        let result = utils::data_dir_path();
        assert!(result.is_ok());

        let path = result.unwrap();

        // Path should be absolute
        assert!(path.is_absolute());

        // Path should exist or be creatable
        if let Some(parent) = path.parent() {
            // Parent should exist or be creatable
            assert!(parent.exists() || parent.parent().map_or(false, |p| p.exists()));
        }
    }

    #[test]
    fn test_copy_files_to_home_no_repo() {
        // This test will likely fail because there's no repository
        // but we can test that it doesn't panic
        let result = utils::copy_files_to_home();

        // It might fail, but it should not panic
        match result {
            Ok(_) => println!("copy_files_to_home succeeded"),
            Err(e) => println!("copy_files_to_home failed (expected): {}", e),
        }
    }

    #[test]
    fn test_loading_animation_new() {
        let animation = utils::LoadingAnimation::new();
        // Just test that it can be created without panicking
        // We can't easily test the animation itself without complex threading tests
        println!("LoadingAnimation created successfully");
    }

    #[test]
    fn test_path_operations() {
        let temp_dir = tempdir().unwrap();
        let test_path = temp_dir.path().join("test").join("file.txt");

        // Test path joining
        let parent = test_path.parent().unwrap();
        assert_eq!(parent, temp_dir.path().join("test"));

        // Test file name extraction
        let file_name = test_path.file_name().unwrap();
        assert_eq!(file_name, "file.txt");

        // Test extension extraction
        let extension = test_path.extension().unwrap();
        assert_eq!(extension, "txt");
    }

    #[test]
    fn test_file_creation_and_reading() {
        let temp_dir = tempdir().unwrap();
        let test_file = temp_dir.path().join("test.txt");

        // Create file
        let content = "Hello, World!";
        fs::write(&test_file, content).unwrap();

        // Verify file exists
        assert!(test_file.exists());

        // Read file content
        let read_content = fs::read_to_string(&test_file).unwrap();
        assert_eq!(read_content, content);
    }

    #[test]
    fn test_directory_operations() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("test_dir");

        // Create directory
        fs::create_dir_all(&test_dir).unwrap();

        // Verify directory exists
        assert!(test_dir.exists());
        assert!(test_dir.is_dir());

        // Create nested directory
        let nested_dir = test_dir.join("nested");
        fs::create_dir_all(&nested_dir).unwrap();
        assert!(nested_dir.exists());
        assert!(nested_dir.is_dir());
    }

    #[test]
    fn test_symlink_operations() {
        let temp_dir = tempdir().unwrap();
        let source_file = temp_dir.path().join("source.txt");
        let symlink_file = temp_dir.path().join("symlink.txt");

        // Create source file
        let content = "test content";
        fs::write(&source_file, content).unwrap();

        // Create symlink (this might fail on some systems, so we'll handle it gracefully)
        #[cfg(unix)]
        {
            use std::os::unix::fs as unix_fs;
            let result = unix_fs::symlink(&source_file, &symlink_file);

            if result.is_ok() {
                // Verify symlink exists
                assert!(symlink_file.exists());
                assert!(symlink_file.is_symlink());

                // Verify symlink content matches source
                let symlink_content = fs::read_to_string(&symlink_file).unwrap();
                assert_eq!(symlink_content, content);
            } else {
                println!("Symlink creation failed (might be expected on some systems)");
            }
        }

        #[cfg(not(unix))]
        {
            println!("Symlink tests skipped on non-Unix systems");
        }
    }

    #[test]
    fn test_file_metadata() {
        let temp_dir = tempdir().unwrap();
        let test_file = temp_dir.path().join("test.txt");

        // Create file
        let content = "test content";
        fs::write(&test_file, content).unwrap();

        // Get metadata
        let metadata = fs::metadata(&test_file).unwrap();

        // Test metadata properties
        assert!(metadata.is_file());
        assert!(!metadata.is_dir());
        assert!(metadata.len() > 0);

        // Test permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = metadata.permissions();
            let mode = perms.mode();
            assert!(mode > 0);
        }
    }

    #[test]
    fn test_directory_listing() {
        let temp_dir = tempdir().unwrap();

        // Create some files
        fs::write(temp_dir.path().join("file1.txt"), "content1").unwrap();
        fs::write(temp_dir.path().join("file2.txt"), "content2").unwrap();
        fs::create_dir_all(temp_dir.path().join("subdir")).unwrap();

        // Read directory
        let entries: Vec<_> = fs::read_dir(temp_dir.path())
            .unwrap()
            .filter_map(|entry| entry.ok())
            .collect();

        // Should have at least our created files
        let file_names: Vec<_> = entries.iter().map(|entry| entry.file_name()).collect();

        assert!(file_names.contains(&"file1.txt".into()));
        assert!(file_names.contains(&"file2.txt".into()));
        assert!(file_names.contains(&"subdir".into()));
    }

    #[test]
    fn test_file_permissions() {
        let temp_dir = tempdir().unwrap();
        let test_file = temp_dir.path().join("test.txt");

        // Create file
        fs::write(&test_file, "test content").unwrap();

        // Test that we can read and write to the file
        let content = fs::read_to_string(&test_file).unwrap();
        assert_eq!(content, "test content");

        // Try to append to file
        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(&test_file)
            .unwrap();
        use std::io::Write;
        writeln!(file, "appended").unwrap();
        drop(file);

        // Verify appended content
        let new_content = fs::read_to_string(&test_file).unwrap();
        assert!(new_content.contains("appended"));
    }

    #[test]
    fn test_error_handling() {
        let temp_dir = tempdir().unwrap();
        let nonexistent_file = temp_dir.path().join("nonexistent.txt");

        // Try to read non-existent file
        let result = fs::read_to_string(&nonexistent_file);
        assert!(result.is_err());

        // Try to get metadata of non-existent file
        let result = fs::metadata(&nonexistent_file);
        assert!(result.is_err());

        // Try to read non-existent directory
        let result = fs::read_dir(&nonexistent_file);
        assert!(result.is_err());
    }
}

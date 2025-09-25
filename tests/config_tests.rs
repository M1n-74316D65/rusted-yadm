#[cfg(test)]
mod tests {
    use rusted_yadm::config::{Config, GitConfig, LocalConfig, SshConfig};
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_config_new() {
        let config = Config::new();
        assert!(config.local.repo.is_none());
        assert!(config.local.class.is_none());
        assert!(config.local.os.is_none());
        assert!(config.local.hostname.is_none());
        assert!(config.local.user.is_none());
        assert!(config.class.is_none());
        assert!(config.os.is_none());
        assert!(config.hostname.is_none());
        assert!(!config.ssh.agent);
        assert!(config.ssh.key.is_none());
        assert!(!config.git.auto_private);
        assert!(!config.git.auto_ignore);
        assert!(!config.git.auto_perms);
    }

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert!(config.local.repo.is_none());
        assert!(config.local.class.is_none());
        assert!(config.local.os.is_none());
        assert!(config.local.hostname.is_none());
        assert!(config.local.user.is_none());
        assert!(config.class.is_none());
        assert!(config.os.is_none());
        assert!(config.hostname.is_none());
        assert!(!config.ssh.agent);
        assert!(config.ssh.key.is_none());
        assert!(!config.git.auto_private);
        assert!(!config.git.auto_ignore);
        assert!(!config.git.auto_perms);
    }

    #[test]
    fn test_config_load_no_file() {
        // Backup existing config file if it exists
        let config_path = Config::get_config_path().unwrap();
        let backup_exists = config_path.exists();
        let backup_content = if backup_exists {
            Some(std::fs::read_to_string(&config_path).unwrap())
        } else {
            None
        };

        // Remove config file temporarily
        if backup_exists {
            std::fs::remove_file(&config_path).unwrap();
        }

        // This should work even if no config file exists
        let result = Config::load();
        assert!(result.is_ok());

        let config = result.unwrap();
        assert!(config.local.repo.is_none());
        assert!(config.local.class.is_none());

        // Restore backup if it existed
        if let Some(content) = backup_content {
            std::fs::write(&config_path, content).unwrap();
        }
    }

    #[test]
    fn test_config_get() {
        let mut config = Config::new();

        // Test getting unset values
        assert!(config.get("local.class").unwrap().is_none());
        assert!(config.get("class").unwrap().is_none());
        assert!(config.get("ssh.key").unwrap().is_none());

        // Test getting boolean values
        assert_eq!(config.get("ssh.agent").unwrap(), Some("false".to_string()));
        assert_eq!(
            config.get("git.auto_private").unwrap(),
            Some("false".to_string())
        );

        // Test unknown key
        assert!(config.get("unknown.key").is_err());
    }

    #[test]
    fn test_config_set() {
        let mut config = Config::new();

        // Test setting string values
        assert!(config.set("local.class", "work").is_ok());
        assert_eq!(config.local.class, Some("work".to_string()));

        assert!(config.set("class", "personal").is_ok());
        assert_eq!(config.class, Some("personal".to_string()));

        assert!(config.set("ssh.key", "/path/to/key").is_ok());
        assert_eq!(config.ssh.key, Some("/path/to/key".to_string()));

        // Test setting boolean values
        assert!(config.set("ssh.agent", "true").is_ok());
        assert!(config.ssh.agent);

        assert!(config.set("git.auto_private", "true").is_ok());
        assert!(config.git.auto_private);

        // Test invalid boolean value
        assert!(config.set("ssh.agent", "invalid").is_err());

        // Test unknown key
        assert!(config.set("unknown.key", "value").is_err());
    }

    #[test]
    fn test_config_list() {
        let mut config = Config::new();

        // Initially should have default boolean values
        let items = config.list();
        assert!(items.contains(&("ssh.agent".to_string(), "false".to_string())));
        assert!(items.contains(&("git.auto_private".to_string(), "false".to_string())));
        assert!(items.contains(&("git.auto_ignore".to_string(), "false".to_string())));
        assert!(items.contains(&("git.auto_perms".to_string(), "false".to_string())));

        // Set some values
        config.set("local.class", "work").unwrap();
        config.set("ssh.key", "/path/to/key").unwrap();

        let items = config.list();
        assert!(items.contains(&("local.class".to_string(), "work".to_string())));
        assert!(items.contains(&("ssh.key".to_string(), "/path/to/key".to_string())));
    }

    #[test]
    fn test_config_save_and_load() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("config.yml");

        // Create a config with some values
        let mut config = Config::new();
        config.local.class = Some("work".to_string());
        config.local.os = Some("linux".to_string());
        config.ssh.agent = true;
        config.git.auto_private = true;

        // Test serialization and deserialization directly
        let content = serde_yaml::to_string(&config).expect("Failed to serialize config");
        fs::write(&config_path, content).expect("Failed to write config file");

        // Test reading back
        let read_content = fs::read_to_string(&config_path).expect("Failed to read config file");
        let loaded_config: Config = serde_yaml::from_str(&read_content).expect("Failed to deserialize config");

        // Verify the values
        assert_eq!(loaded_config.local.class, config.local.class);
        assert_eq!(loaded_config.local.os, config.local.os);
        assert_eq!(loaded_config.ssh.agent, config.ssh.agent);
        assert_eq!(loaded_config.git.auto_private, config.git.auto_private);
    }

    #[test]
    fn test_local_config_default() {
        let local_config = LocalConfig::default();
        assert!(local_config.repo.is_none());
        assert!(local_config.class.is_none());
        assert!(local_config.os.is_none());
        assert!(local_config.hostname.is_none());
        assert!(local_config.user.is_none());
    }

    #[test]
    fn test_ssh_config_default() {
        let ssh_config = SshConfig::default();
        assert!(!ssh_config.agent);
        assert!(ssh_config.key.is_none());
    }

    #[test]
    fn test_git_config_default() {
        let git_config = GitConfig::default();
        assert!(!git_config.auto_private);
        assert!(!git_config.auto_ignore);
        assert!(!git_config.auto_perms);
    }

    #[test]
    fn test_config_debug() {
        let config = Config::new();
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("Config"));
        assert!(debug_str.contains("local"));
        assert!(debug_str.contains("ssh"));
        assert!(debug_str.contains("git"));
    }
}

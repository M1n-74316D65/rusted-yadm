use crate::utils;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub local: LocalConfig,
    pub class: Option<String>,
    pub os: Option<String>,
    pub hostname: Option<String>,
    pub ssh: SshConfig,
    pub git: GitConfig,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LocalConfig {
    pub repo: Option<String>,
    pub class: Option<String>,
    pub os: Option<String>,
    pub hostname: Option<String>,
    pub user: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SshConfig {
    pub key: Option<String>,
    pub agent: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GitConfig {
    pub auto_private: bool,
    pub auto_ignore: bool,
    pub auto_perms: bool,
    pub hooks: HooksConfig,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HooksConfig {
    pub pre_commit: bool,
    pub validate_symlinks: bool,
    pub validate_permissions: bool,
    pub validate_encrypted: bool,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path()?;

        if !config_path.exists() {
            return Ok(Self::new());
        }

        let content = fs::read_to_string(&config_path)?;
        let config: Config = serde_yaml::from_str(&content)?;

        Ok(config)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path()?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_yaml::to_string(self)?;
        fs::write(&config_path, content)?;

        Ok(())
    }

    pub fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let data_dir = utils::data_dir_path()?;
        Ok(data_dir.join("config.yml"))
    }

    pub fn get(&self, key: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        match key {
            "local.class" => Ok(self.local.class.clone()),
            "local.os" => Ok(self.local.os.clone()),
            "local.hostname" => Ok(self.local.hostname.clone()),
            "local.user" => Ok(self.local.user.clone()),
            "class" => Ok(self.class.clone()),
            "os" => Ok(self.os.clone()),
            "hostname" => Ok(self.hostname.clone()),
            "ssh.key" => Ok(self.ssh.key.clone()),
            "ssh.agent" => Ok(Some(self.ssh.agent.to_string())),
            "git.auto_private" => Ok(Some(self.git.auto_private.to_string())),
            "git.auto_ignore" => Ok(Some(self.git.auto_ignore.to_string())),
            "git.auto_perms" => Ok(Some(self.git.auto_perms.to_string())),
            "git.hooks.pre_commit" => Ok(Some(self.git.hooks.pre_commit.to_string())),
            "git.hooks.validate_symlinks" => Ok(Some(self.git.hooks.validate_symlinks.to_string())),
            "git.hooks.validate_permissions" => {
                Ok(Some(self.git.hooks.validate_permissions.to_string()))
            }
            "git.hooks.validate_encrypted" => {
                Ok(Some(self.git.hooks.validate_encrypted.to_string()))
            }
            _ => Err(format!("Unknown config key: {}", key).into()),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        match key {
            "local.class" => self.local.class = Some(value.to_string()),
            "local.os" => self.local.os = Some(value.to_string()),
            "local.hostname" => self.local.hostname = Some(value.to_string()),
            "local.user" => self.local.user = Some(value.to_string()),
            "class" => self.class = Some(value.to_string()),
            "os" => self.os = Some(value.to_string()),
            "hostname" => self.hostname = Some(value.to_string()),
            "ssh.key" => self.ssh.key = Some(value.to_string()),
            "ssh.agent" => {
                self.ssh.agent = value
                    .parse()
                    .map_err(|_| "Invalid boolean value for ssh.agent")?
            }
            "git.auto_private" => {
                self.git.auto_private = value
                    .parse()
                    .map_err(|_| "Invalid boolean value for git.auto_private")?
            }
            "git.auto_ignore" => {
                self.git.auto_ignore = value
                    .parse()
                    .map_err(|_| "Invalid boolean value for git.auto_ignore")?
            }
            "git.auto_perms" => {
                self.git.auto_perms = value
                    .parse()
                    .map_err(|_| "Invalid boolean value for git.auto_perms")?
            }
            "git.hooks.pre_commit" => {
                self.git.hooks.pre_commit = value
                    .parse()
                    .map_err(|_| "Invalid boolean value for git.hooks.pre_commit")?
            }
            "git.hooks.validate_symlinks" => {
                self.git.hooks.validate_symlinks = value
                    .parse()
                    .map_err(|_| "Invalid boolean value for git.hooks.validate_symlinks")?
            }
            "git.hooks.validate_permissions" => {
                self.git.hooks.validate_permissions = value
                    .parse()
                    .map_err(|_| "Invalid boolean value for git.hooks.validate_permissions")?
            }
            "git.hooks.validate_encrypted" => {
                self.git.hooks.validate_encrypted = value
                    .parse()
                    .map_err(|_| "Invalid boolean value for git.hooks.validate_encrypted")?
            }
            _ => return Err(format!("Unknown config key: {}", key).into()),
        }
        Ok(())
    }

    pub fn list(&self) -> Vec<(String, String)> {
        let mut config_items = Vec::new();

        if let Some(ref val) = self.local.class {
            config_items.push(("local.class".to_string(), val.clone()));
        }
        if let Some(ref val) = self.local.os {
            config_items.push(("local.os".to_string(), val.clone()));
        }
        if let Some(ref val) = self.local.hostname {
            config_items.push(("local.hostname".to_string(), val.clone()));
        }
        if let Some(ref val) = self.local.user {
            config_items.push(("local.user".to_string(), val.clone()));
        }
        if let Some(ref val) = self.class {
            config_items.push(("class".to_string(), val.clone()));
        }
        if let Some(ref val) = self.os {
            config_items.push(("os".to_string(), val.clone()));
        }
        if let Some(ref val) = self.hostname {
            config_items.push(("hostname".to_string(), val.clone()));
        }
        if let Some(ref val) = self.ssh.key {
            config_items.push(("ssh.key".to_string(), val.clone()));
        }
        config_items.push(("ssh.agent".to_string(), self.ssh.agent.to_string()));
        config_items.push((
            "git.auto_private".to_string(),
            self.git.auto_private.to_string(),
        ));
        config_items.push((
            "git.auto_ignore".to_string(),
            self.git.auto_ignore.to_string(),
        ));
        config_items.push((
            "git.auto_perms".to_string(),
            self.git.auto_perms.to_string(),
        ));
        config_items.push((
            "git.hooks.pre_commit".to_string(),
            self.git.hooks.pre_commit.to_string(),
        ));
        config_items.push((
            "git.hooks.validate_symlinks".to_string(),
            self.git.hooks.validate_symlinks.to_string(),
        ));
        config_items.push((
            "git.hooks.validate_permissions".to_string(),
            self.git.hooks.validate_permissions.to_string(),
        ));
        config_items.push((
            "git.hooks.validate_encrypted".to_string(),
            self.git.hooks.validate_encrypted.to_string(),
        ));

        config_items
    }
}

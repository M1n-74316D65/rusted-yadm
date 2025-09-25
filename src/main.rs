use clap::error::{Error, ErrorKind};
use clap::{ArgMatches, Args as _, Command, FromArgMatches, Parser, Subcommand};

mod config;
mod git;
mod handler;
mod utils;

#[derive(Parser, Debug, Clone)]
pub struct CloneArgs {
    /// Repo to clone
    pub url: String,
    /// Force clone even if directory exists
    #[arg(short, long)]
    pub force: bool,
    /// Branch to checkout
    #[arg(short, long)]
    pub branch: Option<String>,
}

#[derive(Parser, Debug, Clone)]
pub struct AddArgs {
    /// File to add
    pub file: String,
}

#[derive(Parser, Debug, Clone)]
pub struct CommitArgs {
    /// Commit message
    pub message: String,
}

#[derive(Parser, Debug, Clone)]
pub struct PushArgs {
    /// Remote branch to push to (optional)
    #[arg(short, long)]
    branch: Option<String>,
}

#[derive(Parser, Debug, Clone)]
pub struct InitArgs {
    /// Remote URL to add as origin (optional)
    #[arg(short, long)]
    pub repo: String,
    /// Branch to checkout
    #[arg(short, long)]
    pub branch: Option<String>,
    /// System class
    #[arg(short, long)]
    pub class: Option<String>,
    /// Operating system
    #[arg(short, long)]
    pub os: Option<String>,
    /// Hostname
    #[arg(long)]
    pub hostname: Option<String>,
    /// Username
    #[arg(short, long)]
    pub user: Option<String>,
    /// Force initialization
    #[arg(short, long)]
    pub force: bool,
}

#[derive(Parser, Debug, Clone)]
pub struct StatusArgs {}

#[derive(Parser, Debug, Clone)]
pub struct ListArgs {}

#[derive(Parser, Debug, Clone)]
pub struct FetchArgs {}

#[derive(Parser, Debug, Clone)]
pub struct PullArgs {
    /// Remote branch to pull from (optional)
    #[arg(short, long)]
    branch: Option<String>,
}

#[derive(Parser, Debug, Clone)]
pub struct ConfigArgs {
    /// Configuration key to get/set
    key: Option<String>,
    /// Value to set (optional - if not provided, will get the value)
    value: Option<String>,
    /// List all configuration values
    #[arg(short, long)]
    list: bool,
}

#[derive(Parser, Debug, Clone)]
pub struct AltArgs {
    /// Show what would be processed without making changes
    #[arg(short, long)]
    dry_run: bool,
    /// Force processing even if files exist
    #[arg(short, long)]
    force: bool,
}

#[derive(Parser, Debug, Clone)]
pub struct EncryptArgs {
    /// File to encrypt
    #[arg(short, long)]
    pub file: String,
    /// Output file
    #[arg(short, long)]
    pub output: Option<String>,
    /// Encryption key
    #[arg(short, long)]
    pub key: Option<String>,
}

#[derive(Parser, Debug, Clone)]
pub struct DecryptArgs {
    /// File to decrypt
    #[arg(short, long)]
    pub file: String,
    /// Output file
    #[arg(short, long)]
    pub output: Option<String>,
    /// Decryption key
    #[arg(short, long)]
    pub key: Option<String>,
}

#[derive(Parser, Debug, Clone)]
pub struct BootstrapArgs {
    /// Bootstrap configuration file or URL
    #[arg(short, long)]
    pub config: Option<String>,
    /// Bootstrap configuration URL
    #[arg(short, long)]
    pub url: Option<String>,
    /// Force bootstrap even if files exist
    #[arg(short, long)]
    pub force: bool,
    /// Skip interactive prompts
    #[arg(short, long)]
    pub non_interactive: bool,
}

#[derive(Parser, Debug, Clone)]
pub struct HookInstallArgs {}

#[derive(Parser, Debug, Clone)]
pub struct HookUninstallArgs {}

#[derive(Parser, Debug, Clone)]
pub struct HookPreCommitArgs {}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum CliSub {
    Clone(CloneArgs),
    Init(InitArgs),
    Status(StatusArgs),
    List(ListArgs),
    Fetch(FetchArgs),
    Pull(PullArgs),
    Add(AddArgs),
    Commit(CommitArgs),
    Push(PushArgs),
    Config(ConfigArgs),
    Alt(AltArgs),
    Encrypt(EncryptArgs),
    Decrypt(DecryptArgs),
    Bootstrap(BootstrapArgs),
    HookInstall(HookInstallArgs),
    HookUninstall(HookUninstallArgs),
    HookPreCommit(HookPreCommitArgs),
    Version,
    Help,
    Sync,
}

impl FromArgMatches for CliSub {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        match matches.subcommand() {
            Some(("clone", args)) => Ok(Self::Clone(CloneArgs::from_arg_matches(args)?)),
            Some(("init", args)) => Ok(Self::Init(InitArgs::from_arg_matches(args)?)),
            Some(("status", args)) => Ok(Self::Status(StatusArgs::from_arg_matches(args)?)),
            Some(("list", args)) => Ok(Self::List(ListArgs::from_arg_matches(args)?)),
            Some(("fetch", args)) => Ok(Self::Fetch(FetchArgs::from_arg_matches(args)?)),
            Some(("pull", args)) => Ok(Self::Pull(PullArgs::from_arg_matches(args)?)),
            Some(("add", args)) => Ok(Self::Add(AddArgs::from_arg_matches(args)?)),
            Some(("commit", args)) => Ok(Self::Commit(CommitArgs::from_arg_matches(args)?)),
            Some(("push", args)) => Ok(Self::Push(PushArgs::from_arg_matches(args)?)),
            Some(("config", args)) => Ok(Self::Config(ConfigArgs::from_arg_matches(args)?)),
            Some(("alt", args)) => Ok(Self::Alt(AltArgs::from_arg_matches(args)?)),
            Some(("encrypt", args)) => Ok(Self::Encrypt(EncryptArgs::from_arg_matches(args)?)),
            Some(("decrypt", args)) => Ok(Self::Decrypt(DecryptArgs::from_arg_matches(args)?)),
            Some(("bootstrap", args)) => {
                Ok(Self::Bootstrap(BootstrapArgs::from_arg_matches(args)?))
            }
            Some(("hook-install", args)) => {
                Ok(Self::HookInstall(HookInstallArgs::from_arg_matches(args)?))
            }
            Some(("hook-uninstall", args)) => Ok(Self::HookUninstall(
                HookUninstallArgs::from_arg_matches(args)?,
            )),
            Some(("hook-pre-commit", args)) => Ok(Self::HookPreCommit(
                HookPreCommitArgs::from_arg_matches(args)?,
            )),
            Some(("version", _)) => Ok(Self::Version),
            Some(("help", _)) => Ok(Self::Help),
            Some(("sync", _)) => Ok(Self::Sync),
            Some((_, _)) => Err(Error::raw(
                ErrorKind::InvalidSubcommand,
                "Invalid subcommands",
            )),
            None => Err(Error::raw(
                ErrorKind::MissingSubcommand,
                "Invalid subcommands",
            )),
        }
    }
    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
        match matches.subcommand() {
            Some(("clone", args)) => *self = Self::Clone(CloneArgs::from_arg_matches(args)?),
            Some(("init", args)) => *self = Self::Init(InitArgs::from_arg_matches(args)?),
            Some(("status", args)) => *self = Self::Status(StatusArgs::from_arg_matches(args)?),
            Some(("list", args)) => *self = Self::List(ListArgs::from_arg_matches(args)?),
            Some(("fetch", args)) => *self = Self::Fetch(FetchArgs::from_arg_matches(args)?),
            Some(("pull", args)) => *self = Self::Pull(PullArgs::from_arg_matches(args)?),
            Some(("add", args)) => *self = Self::Add(AddArgs::from_arg_matches(args)?),
            Some(("commit", args)) => *self = Self::Commit(CommitArgs::from_arg_matches(args)?),
            Some(("push", args)) => *self = Self::Push(PushArgs::from_arg_matches(args)?),
            Some(("config", args)) => *self = Self::Config(ConfigArgs::from_arg_matches(args)?),
            Some(("alt", args)) => *self = Self::Alt(AltArgs::from_arg_matches(args)?),
            Some(("encrypt", args)) => *self = Self::Encrypt(EncryptArgs::from_arg_matches(args)?),
            Some(("decrypt", args)) => *self = Self::Decrypt(DecryptArgs::from_arg_matches(args)?),
            Some(("bootstrap", args)) => {
                *self = Self::Bootstrap(BootstrapArgs::from_arg_matches(args)?)
            }
            Some(("hook-install", args)) => {
                *self = Self::HookInstall(HookInstallArgs::from_arg_matches(args)?)
            }
            Some(("hook-uninstall", args)) => {
                *self = Self::HookUninstall(HookUninstallArgs::from_arg_matches(args)?)
            }
            Some(("hook-pre-commit", args)) => {
                *self = Self::HookPreCommit(HookPreCommitArgs::from_arg_matches(args)?)
            }
            Some(("version", _)) => *self = Self::Version,
            Some(("help", _)) => *self = Self::Help,
            Some(("sync", _)) => *self = Self::Sync,
            Some((_, _)) => {
                return Err(Error::raw(
                    ErrorKind::InvalidSubcommand,
                    "Invalid subcommands",
                ))
            }
            None => (),
        };
        Ok(())
    }
}

impl Subcommand for CliSub {
    fn augment_subcommands(cmd: Command) -> Command {
        cmd.subcommand(CloneArgs::augment_args(Command::new("clone")))
            .subcommand(InitArgs::augment_args(Command::new("init")))
            .subcommand(StatusArgs::augment_args(Command::new("status")))
            .subcommand(ListArgs::augment_args(Command::new("list")))
            .subcommand(FetchArgs::augment_args(Command::new("fetch")))
            .subcommand(PullArgs::augment_args(Command::new("pull")))
            .subcommand(AddArgs::augment_args(Command::new("add")))
            .subcommand(CommitArgs::augment_args(Command::new("commit")))
            .subcommand(PushArgs::augment_args(Command::new("push")))
            .subcommand(ConfigArgs::augment_args(Command::new("config")))
            .subcommand(AltArgs::augment_args(Command::new("alt")))
            .subcommand(EncryptArgs::augment_args(Command::new("encrypt")))
            .subcommand(DecryptArgs::augment_args(Command::new("decrypt")))
            .subcommand(BootstrapArgs::augment_args(Command::new("bootstrap")))
            .subcommand(HookInstallArgs::augment_args(Command::new("hook-install")))
            .subcommand(HookUninstallArgs::augment_args(Command::new(
                "hook-uninstall",
            )))
            .subcommand(HookPreCommitArgs::augment_args(Command::new(
                "hook-pre-commit",
            )))
            .subcommand(Command::new("version"))
            
            .subcommand(Command::new("sync"))
    }
    fn augment_subcommands_for_update(cmd: Command) -> Command {
        cmd.subcommand(CloneArgs::augment_args(Command::new("clone")))
            .subcommand(InitArgs::augment_args(Command::new("init")))
            .subcommand(StatusArgs::augment_args(Command::new("status")))
            .subcommand(ListArgs::augment_args(Command::new("list")))
            .subcommand(FetchArgs::augment_args(Command::new("fetch")))
            .subcommand(PullArgs::augment_args(Command::new("pull")))
            .subcommand(AddArgs::augment_args(Command::new("add")))
            .subcommand(CommitArgs::augment_args(Command::new("commit")))
            .subcommand(PushArgs::augment_args(Command::new("push")))
            .subcommand(ConfigArgs::augment_args(Command::new("config")))
            .subcommand(AltArgs::augment_args(Command::new("alt")))
            .subcommand(EncryptArgs::augment_args(Command::new("encrypt")))
            .subcommand(DecryptArgs::augment_args(Command::new("decrypt")))
            .subcommand(BootstrapArgs::augment_args(Command::new("bootstrap")))
            .subcommand(HookInstallArgs::augment_args(Command::new("hook-install")))
            .subcommand(HookUninstallArgs::augment_args(Command::new(
                "hook-uninstall",
            )))
            .subcommand(HookPreCommitArgs::augment_args(Command::new(
                "hook-pre-commit",
            )))
            .subcommand(Command::new("version"))
            
            .subcommand(Command::new("sync"))
    }
    fn has_subcommand(name: &str) -> bool {
        matches!(
            name,
            "clone"
                | "init"
                | "status"
                | "list"
                | "fetch"
                | "pull"
                | "add"
                | "commit"
                | "push"
                | "config"
                | "alt"
                | "encrypt"
                | "decrypt"
                | "bootstrap"
                | "hook-install"
                | "hook-uninstall"
                | "hook-pre-commit"
                | "version"
                
                | "sync"
        )
    }
}

/// Paste.lol on the command line.
#[derive(Parser, Debug, Clone)]
pub struct Cli {
    #[command(subcommand)]
    subcommand: Option<CliSub>,
}

use utils::LoadingAnimation;

fn main() {
    let args = Cli::parse();
    match &args.subcommand {
        Some(CliSub::Init(args)) => {
            let loading = LoadingAnimation::new();
            loading.start("Initializing repository...");

            let result = handler::init(Some(args.repo.as_str()));

            loading.stop();

            match result {
                Ok(_) => println!("Repository initialized successfully"),
                Err(e) => eprintln!("Initialization failed: {}", e),
            }
        }
        Some(CliSub::Status(_args)) => match handler::status() {
            Ok(_) => (),
            Err(e) => eprintln!("Status failed: {}", e),
        },
        Some(CliSub::List(_args)) => match handler::list() {
            Ok(_) => (),
            Err(e) => eprintln!("List failed: {}", e),
        },
        Some(CliSub::Fetch(_args)) => {
            let loading = LoadingAnimation::new();
            loading.start("Fetching changes...");

            let result = handler::fetch();

            loading.stop();

            match result {
                Ok(_) => println!("Fetch successful"),
                Err(e) => eprintln!("Fetch failed: {}", e),
            }
        }
        Some(CliSub::Pull(args)) => {
            let loading = LoadingAnimation::new();
            loading.start("Pulling changes...");

            let result = handler::pull(args.branch.as_deref());

            loading.stop();

            match result {
                Ok(_) => println!("Pull successful"),
                Err(e) => eprintln!("Pull failed: {}", e),
            }
        }
        Some(CliSub::Clone(args)) => {
            let loading = LoadingAnimation::new();
            loading.start("Cloning repository...");

            let result = if args.url.starts_with("git@") {
                handler::clone_ssh(&args.url, args.force)
            } else {
                handler::clone(&args.url, args.force)
            };

            loading.stop();

            match result {
                Ok(_) => println!("Clone successful"),
                Err(e) => eprintln!("Clone failed: {}", e),
            }

            let loading = LoadingAnimation::new();
            loading.start("Copying files to home directory...");

            match utils::copy_files_to_home() {
                Ok(_) => println!("Files copied to home directory successfully"),
                Err(e) => eprintln!("Failed to copy files: {}", e),
            }

            loading.stop();
        }
        Some(CliSub::Add(args)) => {
            let loading = LoadingAnimation::new();
            loading.start("Adding file...");

            let result = handler::add(&args.file);

            loading.stop();

            match result {
                Ok(_) => println!("File added successfully"),
                Err(e) => eprintln!("Failed to add file: {}", e),
            }
        }
        Some(CliSub::Commit(args)) => {
            let loading = LoadingAnimation::new();
            loading.start("Committing changes...");

            let result = handler::commit(&args.message);

            loading.stop();

            match result {
                Ok(_) => println!("Changes committed successfully"),
                Err(e) => eprintln!("Failed to commit changes: {}", e),
            }
        }
        Some(CliSub::Push(args)) => {
            let loading = LoadingAnimation::new();
            loading.start("Pushing changes...");

            let result = handler::push(args.branch.as_deref());

            loading.stop();

            match result {
                Ok(_) => println!("Push successful"),
                Err(e) => eprintln!("Push failed: {}", e),
            }
        }
        Some(CliSub::Config(args)) => {
            match handler::config(args.key.as_deref(), args.value.as_deref(), args.list) {
                Ok(_) => (),
                Err(e) => eprintln!("Config failed: {}", e),
            }
        }
        Some(CliSub::Alt(args)) => match handler::alt(args.dry_run, args.force) {
            Ok(_) => (),
            Err(e) => eprintln!("Alt failed: {}", e),
        },
        Some(CliSub::Encrypt(args)) => match handler::encrypt(&args.file) {
            Ok(_) => (),
            Err(e) => eprintln!("Encrypt failed: {}", e),
        },
        Some(CliSub::Decrypt(args)) => match handler::decrypt(&args.file) {
            Ok(_) => (),
            Err(e) => eprintln!("Decrypt failed: {}", e),
        },
        Some(CliSub::Bootstrap(args)) => {
            match handler::bootstrap(args.config.as_deref(), args.force, args.non_interactive) {
                Ok(_) => (),
                Err(e) => eprintln!("Bootstrap failed: {}", e),
            }
        }
        Some(CliSub::HookInstall(_args)) => match handler::hook_install() {
            Ok(_) => println!("Hooks installed successfully"),
            Err(e) => eprintln!("Hook installation failed: {}", e),
        },
        Some(CliSub::HookUninstall(_args)) => match handler::hook_uninstall() {
            Ok(_) => println!("Hooks uninstalled successfully"),
            Err(e) => eprintln!("Hook uninstallation failed: {}", e),
        },
        Some(CliSub::HookPreCommit(_args)) => match handler::hook_pre_commit() {
            Ok(_) => (),
            Err(e) => eprintln!("Pre-commit hook failed: {}", e),
        },
        Some(CliSub::Version) => {
            println!("rusted-yadm version 0.1.1");
        }
        Some(CliSub::Help) => {
            println!("rusted-yadm - Yet Another Dotfiles Manager");
            println!("Usage: rusted-yadm <command> [options]");
            println!("Use --help for more information.");
        }
        Some(CliSub::Sync) => {
            let loading = LoadingAnimation::new();
            loading.start("Syncing repository...");

            let result = handler::pull(None);
            if result.is_ok() {
                let result = handler::push(None);
                match result {
                    Ok(_) => println!("Sync successful"),
                    Err(e) => eprintln!("Sync failed: {}", e),
                }
            } else {
                eprintln!("Sync failed: {}", result.unwrap_err());
            }

            loading.stop();
        }
        None => println!("No command provided. Use --help for usage information."),
    }
}

use clap::error::{Error, ErrorKind};
use clap::{ArgMatches, Args as _, Command, FromArgMatches, Parser, Subcommand};

mod git;
mod handler;
mod utils;

#[derive(Parser, Debug, Clone)]
struct CloneArgs {
    /// Repo to clone
    title: String,
    /// Force clone even if directory exists
    #[arg(short, long)]
    force: bool,
}

#[derive(Parser, Debug, Clone)]
struct AddArgs {
    /// File to add
    title: String,
}

#[derive(Parser, Debug, Clone)]
struct CommitArgs {
    /// Commit message
    title: String,
}

#[derive(Parser, Debug, Clone)]
struct PushArgs {
    /// Commit message
    title: String,
}

#[derive(Parser, Debug, Clone)]
struct PullArgs {
    /// Commit message
    title: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum CliSub {
    Clone(CloneArgs),
    Add(AddArgs),
    Commit(CommitArgs),
    Push(PushArgs),
    Pull(PullArgs),
}

impl FromArgMatches for CliSub {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        match matches.subcommand() {
            Some(("clone", args)) => Ok(Self::Clone(CloneArgs::from_arg_matches(args)?)),
            Some(("add", args)) => Ok(Self::Add(AddArgs::from_arg_matches(args)?)),
            Some(("commit", args)) => Ok(Self::Commit(CommitArgs::from_arg_matches(args)?)),
            Some(("push", args)) => Ok(Self::Push(PushArgs::from_arg_matches(args)?)),
            Some(("pull", args)) => Ok(Self::Pull(PullArgs::from_arg_matches(args)?)),
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
            Some(("add", args)) => *self = Self::Add(AddArgs::from_arg_matches(args)?),
            Some(("commit", args)) => *self = Self::Commit(CommitArgs::from_arg_matches(args)?),
            Some(("push", args)) => *self = Self::Push(PushArgs::from_arg_matches(args)?),
            Some(("pull", args)) => *self = Self::Pull(PullArgs::from_arg_matches(args)?),
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
            .subcommand(AddArgs::augment_args(Command::new("add")))
            .subcommand(CommitArgs::augment_args(Command::new("commit")))
            .subcommand(PushArgs::augment_args(Command::new("push")))
            .subcommand(PullArgs::augment_args(Command::new("pull")))
    }
    fn augment_subcommands_for_update(cmd: Command) -> Command {
        cmd.subcommand(CloneArgs::augment_args(Command::new("clone")))
            .subcommand(AddArgs::augment_args(Command::new("add")))
            .subcommand(CommitArgs::augment_args(Command::new("commit")))
            .subcommand(PushArgs::augment_args(Command::new("push")))
            .subcommand(PullArgs::augment_args(Command::new("pull")))
    }
    fn has_subcommand(name: &str) -> bool {
        matches!(name, "clone" | "add" | "commit" | "push")
    }
}

/// Paste.lol on the command line.
#[derive(Parser, Debug, Clone)]
struct Cli {
    #[command(subcommand)]
    subcommand: Option<CliSub>,
}

use utils::LoadingAnimation;

fn main() {
    let args = Cli::parse();
    match &args.subcommand {
        Some(CliSub::Clone(args)) => {
            let loading = LoadingAnimation::new();
            loading.start("Cloning repository...");

            let result = if args.title.starts_with("git@") {
                handler::clone_ssh(&args.title, args.force)
            } else {
                handler::clone(&args.title, args.force)
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

            let result = handler::add(&args.title);

            loading.stop();

            match result {
                Ok(_) => println!("File added successfully"),
                Err(e) => eprintln!("Failed to add file: {}", e),
            }
        }
        Some(CliSub::Commit(args)) => {
            let loading = LoadingAnimation::new();
            loading.start("Committing changes...");

            let result = handler::commit(&args.title);

            loading.stop();

            match result {
                Ok(_) => println!("Changes committed successfully"),
                Err(e) => eprintln!("Failed to commit changes: {}", e),
            }
        }
        Some(CliSub::Push(_args)) => {
            let loading = LoadingAnimation::new();
            loading.start("Pushing changes...");

            handler::push();

            loading.stop();
        }
        Some(CliSub::Pull(_args)) => {
            let loading = LoadingAnimation::new();
            loading.start("Pulling changes...");

            handler::pull();

            loading.stop();
        }
        None => println!("No command provided. Use --help for usage information."),
    }
}

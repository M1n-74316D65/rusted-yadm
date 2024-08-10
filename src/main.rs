use clap::error::{Error, ErrorKind};
use clap::{ArgMatches, Args as _, Command, FromArgMatches, Parser, Subcommand};

mod git;
mod handler;

#[derive(Parser, Debug, Clone)]
struct CloneArgs {
    /// Title of the new pastebin.
    title: String,
}

#[derive(Parser, Debug, Clone)]
struct AddArgs {
    /// Title of the new pastebin.
    title: String,
}

#[derive(Parser, Debug, Clone)]
struct CommitArgs {
    /// Title of the new pastebin.
    title: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum CliSub {
    Clone(CloneArgs),
    Add(AddArgs),
    Commit(CommitArgs),
    // Remove(RemoveArgs),
    // Download(DownloadArgs),
    // List(ListArgs),
    // View(ViewArgs),
    // Search(SearchArgs),
    // Settings(SettingsArgs),
}

impl FromArgMatches for CliSub {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        match matches.subcommand() {
            Some(("clone", args)) => Ok(Self::Clone(CloneArgs::from_arg_matches(args)?)),
            Some(("add", args)) => Ok(Self::Add(AddArgs::from_arg_matches(args)?)),
            Some(("commit", args)) => Ok(Self::Commit(CommitArgs::from_arg_matches(args)?)),
            // Some(("remove", args)) => Ok(Self::Remove(RemoveArgs::from_arg_matches(args)?)),
            // Some(("download", args)) => Ok(Self::Download(DownloadArgs::from_arg_matches(args)?)),
            // Some(("list", args)) => Ok(Self::List(ListArgs::from_arg_matches(args)?)),
            // Some(("view", args)) => Ok(Self::View(ViewArgs::from_arg_matches(args)?)),
            // Some(("search", args)) => Ok(Self::Search(SearchArgs::from_arg_matches(args)?)),
            // Some(("settings", args)) => Ok(Self::Settings(SettingsArgs::from_arg_matches(args)?)),
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
            // Some(("remove", args)) => *self = Self::Remove(RemoveArgs::from_arg_matches(args)?),
            // Some(("download", args)) => {
            //     *self = Self::Download(DownloadArgs::from_arg_matches(args)?)
            // }
            // Some(("list", args)) => *self = Self::List(ListArgs::from_arg_matches(args)?),
            // Some(("view", args)) => *self = Self::View(ViewArgs::from_arg_matches(args)?),
            // Some(("search", args)) => *self = Self::Search(SearchArgs::from_arg_matches(args)?),
            // Some(("settings", args)) => {
            //     *self = Self::Settings(SettingsArgs::from_arg_matches(args)?)
            // }
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
        // .subcommand(RemoveArgs::augment_args(Command::new("remove").alias("rm")))
        // .subcommand(DownloadArgs::augment_args(
        //     Command::new("download").alias("dl"),
        // ))
        // .subcommand(ListArgs::augment_args(Command::new("list").alias("ls")))
        // .subcommand(ViewArgs::augment_args(Command::new("view").alias("cat")))
        // .subcommand(SearchArgs::augment_args(
        //     Command::new("search").alias("find"),
        // ))
        // .subcommand(SettingsArgs::augment_args(
        //     Command::new("settings").alias("config"),
        // ))
        // .subcommand_required(true)
    }
    fn augment_subcommands_for_update(cmd: Command) -> Command {
        cmd.subcommand(CloneArgs::augment_args(Command::new("clone")))
            .subcommand(AddArgs::augment_args(Command::new("add")))
            .subcommand(CommitArgs::augment_args(Command::new("commit")))
        // .subcommand(RemoveArgs::augment_args(Command::new("rm").alias("remove")))
        // .subcommand(DownloadArgs::augment_args(
        //     Command::new("download").alias("dl"),
        // ))
        // .subcommand(ListArgs::augment_args(Command::new("list").alias("ls")))
        // .subcommand(ViewArgs::augment_args(Command::new("view").alias("cat")))
        // .subcommand(SearchArgs::augment_args(
        //     Command::new("search").alias("find"),
        // ))
        // .subcommand(SettingsArgs::augment_args(
        //     Command::new("settings").alias("config"),
        // ))
        // .subcommand_required(true)
    }
    fn has_subcommand(name: &str) -> bool {
        matches!(
            name,
            "clone" | "add" | "commit" // | "download"
                                       // | "dl"
                                       // | "list"
                                       // | "ls"
                                       // | "view"
                                       // | "cat"
                                       // | "search"
                                       // | "find"
                                       // | "settings"
                                       // | "config"
        )
    }
}

/// Paste.lol on the command line.
#[derive(Parser, Debug, Clone)]
struct Cli {
    #[command(subcommand)]
    subcommand: Option<CliSub>,
}

fn main() {
    let args = Cli::parse();
    match &args.subcommand {
        Some(CliSub::Clone(args)) => {
            handler::clone(args.title.as_str());
        }
        Some(CliSub::Add(args)) => {
            let _lol = handler::add(args.title.as_str());
        }
        Some(CliSub::Commit(args)) => {
            let _lol = handler::commit(args.title.as_str());
        }

        None => println!("No command provided"),
    }
}

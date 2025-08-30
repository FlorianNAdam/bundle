use clap::{Command, Parser};
use std::os::unix::process::CommandExt;
use std::process::Command as ProcCommand;

/// A small CLI tool to bundle subcommands
#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
struct Cli {
    #[arg(
        short, 
        long, 
        value_parser = parse_mapping,
        help = "Define command mappings in format 'name:path[:description]'. Can be specified multiple times"
    )]
    command: Vec<CommandMapping>,

    #[arg(
        short, 
        long,
        help = "The name of the main command"
    )]
    name: String,

    #[arg(
        short, 
        long,
        help = "Description for the main command"
    )]
    description: Option<String>,

    #[arg(
        short, 
        long,
        help = "Author information for the main command"
    )]
    author: Option<String>,

    #[arg(
        short = 'b', 
        long,
        help = "About information for the main command (similar to description)"
    )]
    about: Option<String>,

    #[arg(
        last = true,
        help = "Arguments passed to the main command"
    )]
    trailing: Vec<String>,
}


#[derive(Debug, Clone)]
struct CommandMapping {
    name: String,
    path: String,
    description: Option<String>,
}

fn parse_mapping(s: &str) -> Result<CommandMapping, String> {
    let parts: Vec<&str> = s.splitn(3, ':').collect();
    if parts.len() < 2 {
        return Err("Mapping must be at least name:path".into());
    }

    let description = if parts.len() == 3 {
        Some(parts[2].to_string())
    } else {
        None
    };

    Ok(CommandMapping {
        name: parts[0].to_string(),
        path: parts[1].to_string(),
        description,
    })
}

macro_rules! optional {
    ($cmd:ident, $cli:ident, $option:ident) => {
        if let Some($option) = $cli.$option {
            $cmd = $cmd.about($option);
        }
    };
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mut cmd = Command::new(&cli.name);

    optional!(cmd, cli, description);
    optional!(cmd, cli, author);
    optional!(cmd, cli, about);

    for mapping in cli.command.clone() {
        let mut subcmd = Command::new(mapping.name);
        if let Some(desc) = &mapping.description {
            subcmd = subcmd.about(desc);
        }
        cmd = cmd.subcommand(subcmd);
    }

    let mut args = cli.trailing;
    args.insert(0, cli.name);

    let matches = cmd.get_matches_from(args);
    if let Some((sub_name, _sub_matches)) = matches.subcommand() {
        if let Some(mapping) = cli.command.iter().find(|m| m.name == sub_name) {
            let error = ProcCommand::new(&mapping.path).exec();
            return Err(error.into());
        }
    }
    Ok(())
}

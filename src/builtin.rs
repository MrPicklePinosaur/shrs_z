use std::path::{Path, PathBuf};

use anyhow::anyhow;
use clap::Parser;
use regex::Regex;
use shrs::prelude::*;

use crate::ZState;

#[derive(Parser)]
struct Cli {
    /// restrict matches to subdirectories of the current directory
    #[arg(short = 'c', long)]
    subdir: bool,
    /// echo the best match, don't cd
    #[arg(short = 'e', long)]
    echo: bool,
    /// list only
    #[arg(short = 'l', long)]
    list: bool,
    /// match by rank only
    #[arg(short = 'r', long)]
    rank_only: bool,
    /// match by recent access only
    #[arg(short = 't', long)]
    recent_only: bool,
    /// remove the current directory from the datafile
    #[arg(short = 'x', long)]
    remove: bool,

    /// Regex to match
    regex: String,
}

pub fn z_builtin(
    sh: &Shell,
    mut z: StateMut<ZState>,
    mut rt: StateMut<Runtime>,
    args: &Vec<String>,
) -> anyhow::Result<CmdOutput> {
    let cli: Cli = Cli::try_parse_from(args)?;

    // Remove the current working directory from database
    if cli.remove {

        if z.database.remove(&rt.working_dir).is_some() {
            return Ok(CmdOutput::success());
        } else {
            return Ok(CmdOutput::error());
        }
    }

    // List database contents
    // TODO maybe more useful to also take a param and list potential matches?
    if cli.list {
        // TODO should sort?

        for (path, frecency) in z.database.iter() {
            println!("{}      {:?}", frecency.value(), path);
        }

        return Ok(CmdOutput::success());
    }

    // Search database
    let dir: anyhow::Result<PathBuf> = {
        // iterate through database by frecency and take first match

        let mut entries = z.database.iter().collect::<Vec<_>>();

        // Configure heuristic
        if cli.rank_only {
            entries.sort_by(|a, b| b.1.rank().cmp(&a.1.rank()));
        } else if cli.recent_only {
            entries.sort_by(|a, b| b.1.access_time().cmp(&a.1.access_time()));
        } else {
            entries.sort_by(|a, b| b.1.value().cmp(&a.1.value()));
        }

        let Ok(regex) = Regex::new(&cli.regex) else {
            return Err(anyhow!("invalid regex {}", cli.regex));
        };

        let matched_dir = entries
            .iter()
            .find(|(path, _)| regex.is_match(path.to_str().unwrap()));
        match matched_dir {
            Some((dir, _)) => Ok(dir.to_path_buf()),
            None => Err(anyhow!("no matches")),
        }
    };

    let dir = match dir {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("{e}");
            return Ok(CmdOutput::error());
        },
    };

    // If echo flag is passed, just output match but don't change directory
    if cli.echo {
        println!("{dir:?}");
        return Ok(CmdOutput::success());
    }

    set_working_dir(sh, &mut rt, &dir, true).unwrap();

    Ok(CmdOutput::success())
}

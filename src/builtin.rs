use shrs::prelude::*;
use clap::Parser;

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
    regex: Vec<String>,
}

pub struct ZBuiltin {

}

impl ZBuiltin {
    pub fn new() -> Self {
        ZBuiltin {  }
    }
}

impl BuiltinCmd for ZBuiltin {
    fn run(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        args: &[String],
    ) -> anyhow::Result<CmdOutput> {

        let cli = Cli::try_parse_from(args)?;

        Ok(CmdOutput::success())
    }
}

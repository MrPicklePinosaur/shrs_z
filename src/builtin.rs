use shrs::prelude::*;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// echo the best match, don't cd
    #[arg(short, long)]
    echo: bool,
    /// list only
    #[arg(short, long)]
    list: bool,
    /// match by rank only
    #[arg(short, long)]
    rank_only: bool,
    /// match by recent access only
    #[arg(short = 't', long)]
    recent_only: bool,
}

#[derive(Default)]
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

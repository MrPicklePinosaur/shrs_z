mod builtin;

use std::{
    collections::{hash_map::Entry, HashMap},
    path::PathBuf,
};

use chrono::{DateTime, Utc};
use shrs::{anyhow::anyhow, prelude::*};

use crate::builtin::ZBuiltin;

struct Frecency {
    /// Number of times the directory was accessed
    rank: u64,
    /// The last timestamp the directory was accessed
    access_time: i64,
}

impl Frecency {
    pub fn new() -> Self {
        let now = Utc::now().timestamp();
        Frecency {
            rank: 1,
            access_time: now,
        }
    }

    /// Access the directory, updating it's frecency
    pub fn access(&mut self) {
        self.rank += 1;
        self.access_time = Utc::now().timestamp();
    }

    /// Return the weighted frecency value
    pub fn value(&self) -> u64 {
        let dx = Utc::now().timestamp() - self.access_time;
        (10000.0 * self.rank as f64 * (3.75 / ((0.0001 * dx as f64 + 0.1) + 0.25))) as u64
    }
}

#[derive(Default)]
struct ZState {
    /// Map of path to 'frecency'
    database: HashMap<PathBuf, Frecency>,
    /// Sum of all ranks in database
    total_rank: u64,
}

// TODO entering a directory should also count as an access??

pub fn before_command_hook(
    sh: &Shell,
    sh_ctx: &mut Context,
    sh_rt: &mut Runtime,
    ctx: &BeforeCommandCtx,
) -> anyhow::Result<()> {
    // update access directory after each command thats ran
    let Some(state) = sh_ctx.state.get_mut::<ZState>() else {
        return Err(anyhow!("could not get z state"));
    };

    // maybe clean up old entries

    // insert new entry
    match state.database.entry(ctx.run_ctx.working_dir.clone()) {
        Entry::Occupied(mut entry) => {
            entry.get_mut().access();
        },
        Entry::Vacant(entry) => {
            entry.insert(Frecency::new());
        },
    }
    state.total_rank += 1;

    Ok(())
}

pub struct ZPlugin;

impl ZPlugin {
    pub fn new() -> Self {
        ZPlugin
    }
}

impl Plugin for ZPlugin {
    fn init(&self, shell: &mut ShellConfig) -> anyhow::Result<()> {
        let state = ZState::default();
        shell.state.insert(state);

        shell.builtins.insert("z", ZBuiltin::new());
        shell.hooks.insert(before_command_hook);

        Ok(())
    }
}

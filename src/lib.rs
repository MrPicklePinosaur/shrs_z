mod builtin;

use shrs::prelude::*;
use crate::builtin::ZBuiltin;

pub struct ZPlugin;

impl ZPlugin {
    pub fn new() -> Self {
        ZPlugin
    }
}

impl Plugin for ZPlugin {
    fn init(&self, shell: &mut ShellConfig) -> anyhow::Result<()> {
        shell.builtins.insert("z", ZBuiltin::default());
        Ok(())
    }
}

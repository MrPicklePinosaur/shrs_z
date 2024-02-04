use shrs::prelude::*;

pub struct MyPlugin;

impl MyPlugin {
    pub fn new() -> Self {
        MyPlugin
    }
}

impl Plugin for MyPlugin {
    fn init(&self, shell: &mut ShellConfig) -> anyhow::Result<()> {
        Ok(())
    }
}

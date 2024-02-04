use shrs::prelude::*;
use shrs_z::MyPlugin;

fn main() {
    let myshell = ShellBuilder::default()
        .with_plugin(MyPlugin::new())
        .build()
        .unwrap();

    myshell.run();
}

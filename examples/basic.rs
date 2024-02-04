use shrs::prelude::*;
use shrs_z::ZPlugin;

fn main() {
    let myshell = ShellBuilder::default()
        .with_plugin(ZPlugin::new())
        .build()
        .unwrap();

    myshell.run();
}

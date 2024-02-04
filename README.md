<div align="center">

# shrs_z

[![crates.io](https://img.shields.io/crates/v/shrs_z.svg)](https://crates.io/crates/shrs_z)
[![docs.rs](https://docs.rs/shrs_z/badge.svg)](https://docs.rs/shrs_z)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#)

</div>

This is a plugin for [shrs](https://github.com/MrPicklePinosaur/shrs).

## Using this plugin

First add this plugin to your dependencies
```toml
shrs_z = { version = "0.0.1" }
```

Then include this plugin when initializing shrs
```rust
use shrs::prelude::*;
use shrs_z::MyPlugin;

let myshell = ShellBuilder::default()
    .with_plugin(MyPlugin::new())
    .build()
    .unwrap();

```

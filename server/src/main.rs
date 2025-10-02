//! This example showcases how to use Lightyear with Bevy, to easily get replication along with prediction/interpolation working.
//!
//! There is a lot of setup code, but it's mostly to have the examples work in all possible configurations of transport.
//! (all transports are supported, as well as running the example in client-and-server or host-server mode)
//!
//!
//! Run with
//! - `cargo run -- server`
//! - `cargo run -- client -c 1`
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use crate::server::ExampleServerPlugin;
use bevy::prelude::*;
use bevy_scriptum::Script;
use bevy_scriptum::runtimes::lua::LuaScript;
use core::time::Duration;
use lightyear_examples_common::cli::{Cli, Mode};
use lightyear_examples_common::shared::FIXED_TIMESTEP_HZ;
use shared::plugin::SharedPlugin;
use shared::protocol::NPC;

mod scripting;

mod renderer;
mod server;

/// When running the example as a binary, we only support Client or Server mode.
fn main() {
    let mut cli = Cli::default();
    cli.mode = Some(Mode::Server);

    let mut app = cli.build_app(Duration::from_secs_f64(1.0 / FIXED_TIMESTEP_HZ), true);

    app.add_plugins(SharedPlugin);

    cli.spawn_connections(&mut app);

    app.add_plugins(ExampleServerPlugin);
    println!("BEGOR");
    app.add_plugins(scripting::CoreScriptApiPlugin);
    app.add_systems(Startup, server_setup);

    #[cfg(feature = "gui")]
    app.add_plugins(renderer::ExampleRendererPlugin);

    app.run();
}

fn server_setup(mut commands: Commands, assets_server: Res<AssetServer>) {
    info!("AAAA");
    commands.spawn((NPC, Name::new("John")));
    commands.spawn((NPC, Name::new("Mary")));
    commands.spawn((NPC, Name::new("Alice")));

    commands.spawn((
        Script::<LuaScript>::new(assets_server.load("game.lua")),
        Name::new("Game Script"),
    ));
}

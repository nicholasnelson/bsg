use bevy::prelude::*;
mod core;
mod gameplay;
mod input;
mod render;

use gameplay::GameplayPlugin;
use core::CorePlugin;
use input::InputPlugin;
use render::RenderPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((CorePlugin, InputPlugin, RenderPlugin, GameplayPlugin))
        .run();
}

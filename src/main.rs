use bevy::prelude::*;
mod core;
mod gameplay;
mod render;

use gameplay::GameplayPlugin;
use render::RenderPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((RenderPlugin, GameplayPlugin))
        .run();
}

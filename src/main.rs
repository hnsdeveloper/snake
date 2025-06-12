use bevy::prelude::*;
mod snake;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(snake::SnakeInitializePlugin)
        .add_plugins(snake::GameplayPlugin)
        .add_plugins(snake::GameOverPlugin)
        .run();
}

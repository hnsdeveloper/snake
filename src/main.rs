use bevy::prelude::*;
mod snake;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Snake".to_owned(),
            ..default()
        }),
        ..default()
    }))
    .add_plugins(snake::EntrancePlugin)
    .add_plugins(snake::MainPlugin)
    .add_plugins(snake::GameplayPlugin)
    .add_plugins(snake::GameOverPlugin)
    .run();
}

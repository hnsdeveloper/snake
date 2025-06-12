use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Main,
    Gameplay,
    Gameover,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameplayState {
    #[default]
    Running,
    Paused,
}

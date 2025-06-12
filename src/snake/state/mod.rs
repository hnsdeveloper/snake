use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum State {
    #[default]
    Main,
    Gameplay,
    Gameover,
    Pause,
}

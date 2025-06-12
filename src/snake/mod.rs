use atomic_counter_resource::AtomicCounter;
use bevy::prelude::*;
use gameplay::Apple;
use gameplay::PLAY_SIDE;
use gameplay::SnakeHead;
use gameplay::SnakePart;
use gameplay::collision;
use rng_resource::RngResource;
use snake_resource_manager::SnakeResourceManager;
use state::*;

mod atomic_counter_resource;
mod gameover;
mod gameplay;
mod rng_resource;
mod snake_resource_manager;
mod state;

pub use gameover::GameOverPlugin;
pub use gameplay::GameplayPlugin;
pub struct SnakeInitializePlugin;

impl Plugin for SnakeInitializePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(state::GameState::Gameplay);
    }
}

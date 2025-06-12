use atomic_counter_resource::AtomicCounter;
use bevy::prelude::*;
use rng_resource::RngResource;
use snake_resource_manager::SnakeResourceManager;
use state::State;

mod atomic_counter_resource;
mod gameplay;
mod rng_resource;
mod snake_resource_manager;
mod state;

pub use gameplay::SnakeGameplayPlugin;

pub struct SnakeInitializePlugin;

impl Plugin for SnakeInitializePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(state::State::Gameplay);
    }
}

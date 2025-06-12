use bevy::{math::FloatPow, prelude::*};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            check_game_over.run_if(in_state(super::GameState::Gameplay)),
        );
    }
}

fn check_game_over(
    head: Single<&super::SnakeHead>,
    parts: Query<(&Transform, &super::SnakePart)>,
    collidable_others: Query<
        &Transform,
        (
            Without<super::Apple>,
            Without<super::SnakePart>,
            Without<DirectionalLight>,
            Without<Camera3d>,
        ),
    >,
    mut next_state: ResMut<NextState<super::GameState>>,
) {
    let parts_count = parts.iter().count();
    let collision = super::collision(head, parts, collidable_others);
    if (parts_count == super::PLAY_SIDE.squared() as usize) || collision {
        next_state.set(super::GameState::Gameover);
    }
}

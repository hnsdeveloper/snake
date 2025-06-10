use bevy::prelude::*;
mod snake;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .init_resource::<snake::snake_res_manager::SnakeResourceManager>()
        .init_resource::<snake::rng_resource::RngResource>()
        .init_resource::<snake::atomic_counter_resource::AtomicCounter>()
        .add_systems(Startup, snake::spawn_map)
        .add_systems(Startup, snake::spawn_head)
        .add_systems(
            Update,
            (
                snake::set_player_direction,
                snake::move_player
                    .after(snake::set_player_direction)
                    .before(snake::check_eaten_apple),
            ),
        )
        .add_event::<snake::AppleEaten>()
        .add_event::<snake::SnakeGrow>()
        .init_resource::<snake::SnakeLast>()
        .add_systems(
            Update,
            snake::check_eaten_apple.before(snake::despawn_apple),
        )
        .add_systems(Update, snake::spawn_snake_part.after(snake::move_player))
        .add_systems(Update, (snake::despawn_apple, snake::spawn_apple))
        .run();
}

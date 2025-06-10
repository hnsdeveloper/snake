use bevy::{input::keyboard::KeyCode, prelude::*};

use crate::snake::rng_resource::RngResource;

pub mod atomic_counter_resource;
pub mod rng_resource;
pub mod snake_res_manager;

#[derive(Component)]
pub struct SnakeHead(Vec2);

#[derive(Component)]
pub struct SnakePart(usize);

const PLAY_SIDE: f32 = 30.;
const HALF_PLAY_SIDE: f32 = PLAY_SIDE / 2.;
const INITIAL_Z: f32 = -50.;

pub fn spawn_head(
    mut commands: Commands,
    snake_resources: Res<snake_res_manager::SnakeResourceManager>,
    rng: Res<rng_resource::RngResource>,
    id: Res<atomic_counter_resource::AtomicCounter>,
) {
    let x_snake_head = rng.random_in_range(0..PLAY_SIDE as u64) as f32 - HALF_PLAY_SIDE;
    let y_snake_head = rng.random_in_range(0..PLAY_SIDE as u64) as f32 - HALF_PLAY_SIDE;
    let (x, y) = match rng.random_in_range(0..4) {
        0 => (-1, 0),
        1 => (1, 0),
        2 => (0, -1),
        3 => (0, 1),
        _ => panic!("Range should be between 0 and 4"),
    };
    // Spawns the snake head
    commands.spawn((
        SnakeHead(Vec2::new(x as f32, y as f32)),
        SnakePart(id.get_id()),
        Transform::from_translation(Vec3::new(x_snake_head, y_snake_head, INITIAL_Z)),
        Mesh3d(snake_resources.ball_mesh()),
        MeshMaterial3d(snake_resources.ball_material(
            rng.random_in_range(0..(snake_resources.ball_materials_count() as u64)) as usize,
        )),
    ));
}

fn is_inside(x: i32, y: i32, side: i32) -> bool {
    if (x < 0 || x > side) || (y < 0 || y > side) {
        return false;
    }
    true
}

pub fn spawn_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(DirectionalLight::default());
    commands.spawn(Camera3d::default());
    let cube = meshes.add(Cuboid::from_length(1.));
    let material = materials.add(StandardMaterial {
        base_color: Color::hsl(360., 1., 0.),
        ..Default::default()
    });

    for y in -1..=PLAY_SIDE as i32 + 1 {
        for x in -1..=PLAY_SIDE as i32 + 1 {
            println!("x : {x}, y : {y}");
            if !is_inside(x, y, PLAY_SIDE as i32) {
                println!("Inserting!");
                commands.spawn((
                    Transform::from_translation(Vec3::new(
                        x as f32 - HALF_PLAY_SIDE,
                        y as f32 - HALF_PLAY_SIDE,
                        INITIAL_Z,
                    )),
                    Mesh3d(cube.clone()),
                    MeshMaterial3d(material.clone()),
                ));
            }
        }
    }
}

pub fn set_player_direction(
    mut snake_head: Single<&mut SnakeHead>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let mut direction_delta = snake_head.0;
    let pressed_count = input.get_pressed().count();
    if pressed_count == 1 {
        if input.pressed(KeyCode::KeyW) {
            direction_delta.x = 0.;
            direction_delta.y = 1.;
        }
        if input.pressed(KeyCode::KeyS) {
            direction_delta.x = 0.;
            direction_delta.y = -1.;
        }
        if input.pressed(KeyCode::KeyA) {
            direction_delta.x = -1.;
            direction_delta.y = 0.;
        }
        if input.pressed(KeyCode::KeyD) {
            direction_delta.x = 1.;
            direction_delta.y = 0.;
        }
        let dot = direction_delta.dot(snake_head.0);
        if dot != -1. {
            snake_head.0 = direction_delta;
        }
    }
}

pub fn move_player(
    head: Single<&SnakeHead>,
    parts: Query<(&mut Transform, &SnakePart)>,
    mut last_part: ResMut<SnakeLast>,
) {
    let mut v = Vec::new();
    for (t, s) in parts {
        v.push((s.0, t));
    }
    v.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    let mut last = Vec3::ZERO;
    let mut moved = true;
    for (idx, (_, mut part_transform)) in v.into_iter().enumerate() {
        let current = part_transform.translation;
        if idx == 0 {
            part_transform.translation.x += head.0.x;
            part_transform.translation.y += head.0.y;
            part_transform.translation.x = part_transform
                .translation
                .x
                .clamp(-HALF_PLAY_SIDE, HALF_PLAY_SIDE);
            part_transform.translation.y = part_transform
                .translation
                .y
                .clamp(-HALF_PLAY_SIDE, HALF_PLAY_SIDE);
            moved = !(current == part_transform.translation);
        } else if moved && idx != 0 {
            part_transform.translation = last;
        }
        last = current;
    }
    if moved {
        last_part.0.x = last.x;
        last_part.0.y = last.y;
    }
}

#[derive(Component)]
pub struct Apple;

#[derive(Event)]
pub struct AppleEaten;

#[derive(Event)]
pub struct SnakeGrow;

#[derive(Resource, Default)]
pub struct SnakeLast(Vec2);

pub fn check_eaten_apple(
    head: Single<&Transform, With<SnakeHead>>,
    apple: Option<Single<&Transform, (With<Apple>, Without<SnakeHead>)>>,
    mut apple_eaten_event: EventWriter<AppleEaten>,
    mut snake_grow_event: EventWriter<SnakeGrow>,
) {
    if let Some(apple) = apple {
        if apple.translation == head.translation {
            apple_eaten_event.write(AppleEaten);
            snake_grow_event.write(SnakeGrow);
        }
    }
}

pub fn spawn_snake_part(
    mut commands: Commands,
    mut snake_grow_event: EventReader<SnakeGrow>,
    snake_last: Res<SnakeLast>,
    snake_resources: Res<snake_res_manager::SnakeResourceManager>,
    rng: Res<rng_resource::RngResource>,
    id: Res<atomic_counter_resource::AtomicCounter>,
) {
    if let Some(_) = snake_grow_event.read().last() {
        commands.spawn((
            SnakePart(id.get_id()),
            Transform::from_translation(Vec3::new(snake_last.0.x, snake_last.0.y, INITIAL_Z)),
            Mesh3d(snake_resources.ball_mesh()),
            MeshMaterial3d(snake_resources.ball_material(
                rng.random_in_range(0..(snake_resources.ball_materials_count() as u64)) as usize,
            )),
        ));
    }
}

pub fn despawn_apple(
    mut commands: Commands,
    mut apple_eaten_event: EventReader<AppleEaten>,
    apple: Single<Entity, With<Apple>>,
) {
    if let Some(_) = apple_eaten_event.read().last() {
        commands.entity(*apple).despawn();
    }
}

pub fn spawn_apple(
    mut commands: Commands,
    apple: Option<Single<(Entity, &Apple)>>,
    snake_parts: Query<&Transform, With<SnakePart>>,
    rng: Res<rng_resource::RngResource>,
    snake_resources: Res<snake_res_manager::SnakeResourceManager>,
) {
    if let None = apple {
        'outer: loop {
            let x = rng.random_in_range(0..PLAY_SIDE as u64) as f32 - HALF_PLAY_SIDE;
            let y = rng.random_in_range(0..PLAY_SIDE as u64) as f32 - HALF_PLAY_SIDE;
            for (transform) in snake_parts {
                if transform.translation.x == x && transform.translation.y == y {
                    continue 'outer;
                }
            }
            commands.spawn((
                Apple,
                Transform::from_translation(Vec3::new(x, y, INITIAL_Z)),
                Mesh3d(snake_resources.apple_mesh()),
                MeshMaterial3d(snake_resources.apple_materials(0)),
            ));
            break;
        }
    }
}

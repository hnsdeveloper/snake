use bevy::prelude::*;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<super::SnakeResourceManager>()
            .init_resource::<super::RngResource>()
            .init_resource::<super::AtomicCounter>()
            .init_resource::<SnakeLast>()
            .add_event::<AppleEaten>()
            .add_systems(
                OnEnter(super::GameState::Gameplay),
                (spawn_map, spawn_head, initialize_fixed_step),
            )
            .add_systems(
                FixedUpdate,
                move_player
                    .after(process_input)
                    .before(check_eaten_apple)
                    .run_if(in_state(super::GameState::Gameplay))
                    .run_if(not(collision))
                    .run_if(not(in_state(super::GameplayState::Paused))),
            )
            .add_systems(
                Update,
                (
                    process_input,
                    check_eaten_apple.before(despawn_apple),
                    increase_fixed_update.after(check_eaten_apple),
                    spawn_snake_part.after(move_player),
                    (despawn_apple, spawn_apple),
                )
                    .run_if(in_state(super::GameState::Gameplay))
                    .run_if(not(in_state(super::GameplayState::Paused))),
            )
            .add_systems(OnExit(super::GameState::Gameplay), despawn_all);
    }
}

#[derive(Component)]
pub struct SnakeHead(Vec2);

#[derive(Component)]
pub struct SnakePart(usize);

pub const PLAY_SIDE: f32 = 30.;
const HALF_PLAY_SIDE: f32 = PLAY_SIDE / 2.;
const INITIAL_Z: f32 = -50.;
const MIN_HZ: f64 = 10.;
const MAX_HZ: f64 = 30.;

fn initialize_fixed_step(mut fixed_time: ResMut<Time<Fixed>>) {
    fixed_time.set_timestep_hz(MIN_HZ);
}

fn spawn_head(
    mut commands: Commands,
    snake_resources: Res<super::SnakeResourceManager>,
    rng: Res<super::RngResource>,
    id: Res<super::AtomicCounter>,
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

fn spawn_map(
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
            if !is_inside(x, y, PLAY_SIDE as i32) {
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

fn process_input(
    mut snake_head: Single<&mut SnakeHead>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
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

pub fn collision(
    head: Single<&SnakeHead>,
    parts: Query<(&Transform, &SnakePart)>,
    collidable_others: Query<
        &Transform,
        (
            Without<Apple>,
            Without<SnakePart>,
            Without<DirectionalLight>,
            Without<Camera3d>,
        ),
    >,
) -> bool {
    let mut v = Vec::new();
    for (t, s) in parts {
        v.push((s.0, t));
    }
    v.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    let mut v: Vec<_> = v
        .into_iter()
        .map(|(_, t)| (*t).translation.truncate())
        .collect();
    for i in (0..v.len()).rev() {
        if i != 0 {
            v[i] = v[i - 1];
        } else {
            v[i] += head.0;
        }
    }
    for l in v[1..].iter() {
        if *l == v[0] {
            return true;
        }
    }
    for collidable in collidable_others {
        if collidable.translation.truncate() == v[0] {
            return true;
        }
    }
    false
}

fn move_player(
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
    for (idx, (_, mut part_transform)) in v.into_iter().enumerate() {
        let current = part_transform.translation;
        if idx == 0 {
            part_transform.translation.x += head.0.x;
            part_transform.translation.y += head.0.y;
        } else {
            part_transform.translation = last;
        }
        last = current;
    }
    last_part.0.x = last.x;
    last_part.0.y = last.y;
}

#[derive(Component)]
pub struct Apple;

#[derive(Event)]
struct AppleEaten;

#[derive(Resource, Default)]
struct SnakeLast(Vec2);

fn check_eaten_apple(
    head: Single<&Transform, With<SnakeHead>>,
    apple: Option<Single<&Transform, (With<Apple>, Without<SnakeHead>)>>,
    mut apple_eaten_event: EventWriter<AppleEaten>,
) {
    if let Some(apple) = apple {
        if apple.translation == head.translation {
            apple_eaten_event.write(AppleEaten);
        }
    }
}

fn spawn_snake_part(
    mut commands: Commands,
    mut apple_eaten_event: EventReader<AppleEaten>,
    snake_last: Res<SnakeLast>,
    snake_resources: Res<super::SnakeResourceManager>,
    rng: Res<super::RngResource>,
    id: Res<super::AtomicCounter>,
) {
    if let Some(_) = apple_eaten_event.read().last() {
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

fn increase_fixed_update(
    snake_parts: Query<&SnakePart>,
    mut apple_eaten_event: EventReader<AppleEaten>,
    mut time: ResMut<Time<Fixed>>,
) {
    if let Some(_) = apple_eaten_event.read().last() {
        let parts = snake_parts.iter().count() as f64;
        let log_parts = parts.log2();
        let mut hz = (10. + log_parts * 2.).next_up();
        hz = hz.clamp(MIN_HZ, MAX_HZ);
        time.set_timestep_hz(hz);
    }
}

fn despawn_apple(
    mut commands: Commands,
    mut apple_eaten_event: EventReader<AppleEaten>,
    apple: Single<Entity, With<Apple>>,
) {
    if let Some(_) = apple_eaten_event.read().last() {
        commands.entity(*apple).despawn();
    }
}

fn spawn_apple(
    mut commands: Commands,
    apple: Option<Single<(Entity, &Apple)>>,
    snake_parts: Query<&Transform, With<SnakePart>>,
    rng: Res<super::RngResource>,
    snake_resources: Res<super::SnakeResourceManager>,
) {
    if let None = apple {
        'outer: loop {
            let x = rng.random_in_range(0..PLAY_SIDE as u64) as f32 - HALF_PLAY_SIDE;
            let y = rng.random_in_range(0..PLAY_SIDE as u64) as f32 - HALF_PLAY_SIDE;
            for transform in snake_parts {
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

fn despawn_all(
    mut commands: Commands,
    all: Query<(Entity, &Transform)>,
    camera: Single<Entity, With<Camera3d>>,
) {
    for (entity, transform) in all {
        if transform.translation.z == INITIAL_Z {
            commands.entity(entity).despawn();
        }
    }
    commands.entity(*camera).despawn();
}

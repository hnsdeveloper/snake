use bevy::{
    input::{
        common_conditions::input_just_released,
        mouse::{AccumulatedMouseMotion, MouseButtonInput},
    },
    prelude::*,
    window::{PrimaryWindow, WindowFocused},
};
use rand::prelude::*;
use std::sync::Mutex;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, (spawn_camera, spawn_map));
    app.add_systems(Update, (player_look, player_move.after(player_look)));
    app.add_systems(
        Update,
        (
            toggle_grab.run_if(input_just_released(KeyCode::Escape)),
            focus_events,
        ),
    );
    app.add_systems(Update, (shoot_ball, ball_spawn.after(shoot_ball)));
    app.add_systems(Update, (apply_gravity.before(apply_velocity), apply_velocity));
    app.add_observer(apply_grab);
    app.add_event::<BallSpawn>();
    app.init_resource::<BallData>();
    app.run();
}

fn player_look(
    mut player: Single<&mut Transform, With<Player>>,
    mut window: Single<&Window, With<PrimaryWindow>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
) {
    if window.focused {
        let dt = time.delta_secs();
        let sensitivity = 100. / window.width().min(window.height());
        use EulerRot::YXZ;
        let (mut yaw, mut pitch, _) = player.rotation.to_euler(YXZ);
        pitch -= mouse_motion.delta.y * dt * sensitivity;
        yaw -= mouse_motion.delta.x * dt * sensitivity;
        pitch = pitch.clamp(-1.57, 1.57);
        player.rotation = Quat::from_euler(YXZ, yaw, pitch, 0.);
    }
}

fn focus_events(mut events: EventReader<WindowFocused>, mut commands: Commands) {
    if let Some(event) = events.read().last() {
        commands.trigger(GrabEvent(event.focused));
    }
}

fn toggle_grab(mut window: Single<&mut Window, With<PrimaryWindow>>, mut commands: Commands) {
    window.focused = !window.focused;
    commands.trigger(GrabEvent(window.focused));
}

fn apply_grab(grab: Trigger<GrabEvent>, mut window: Single<&mut Window, With<PrimaryWindow>>) {
    use bevy::window::CursorGrabMode;
    if **grab {
        window.cursor_options.visible = false;
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
    } else {
        window.cursor_options.visible = true;
        window.cursor_options.grab_mode = CursorGrabMode::None;
    }
}

const SPEED: f32 = 50.;

fn player_move(
    mut player: Single<&mut Transform, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut delta = Vec3::ZERO;
    if input.pressed(KeyCode::KeyA) {
        delta.x -= 1.;
    }
    if input.pressed(KeyCode::KeyD) {
        delta.x += 1.;
    }
    if input.pressed(KeyCode::KeyW) {
        delta.z += 1.;
    }
    if input.pressed(KeyCode::KeyS) {
        delta.z -= 1.;
    }
    let forward = player.forward().as_vec3() * delta.z;
    let right = player.right().as_vec3() * delta.x;
    let mut to_move = forward + right;
    to_move.y = 0.;
    to_move = to_move.normalize_or_zero();
    player.translation += to_move * time.delta_secs() * SPEED;
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera3d::default(), Player));
}

fn spawn_map(
    mut commands: Commands,
    ball_data : Res<BallData>
) {
    commands.spawn(DirectionalLight::default());
    for h in 0..ball_data.materials.len() {
        commands.spawn((
            Transform::from_translation(Vec3::new((-8. + h as f32) * 2., 0., -50.0)),
            Mesh3d(ball_data.mesh()),
            MeshMaterial3d(ball_data.materials[h].clone()),
        ));
    }
}

fn ball_spawn(
    mut events: EventReader<BallSpawn>,
    mut commands: Commands,
    ball_data : Res<BallData>,
) {
    for spawn in events.read() {
        commands.spawn((
            Transform::from_translation(spawn.position),
            Mesh3d(ball_data.mesh()),
            MeshMaterial3d(ball_data.material()),
            Velocity(spawn.velocity)
        ));
    }
}

fn shoot_ball(
    player: Single<&Transform, With<Player>>,
    input: Res<ButtonInput<MouseButton>>,
    mut event_write: EventWriter<BallSpawn>,
) {
    if input.just_released(MouseButton::Left) {
        let pos = player.translation;
        let vel = player.forward().as_vec3() * 15.;
        event_write.write(BallSpawn { position: pos, velocity : vel });
    }
}

fn apply_velocity(mut objects : Query<(&mut Transform, &Velocity)>, time : Res<Time>)
{
    for (mut transform, velocity) in &mut objects
    {
        transform.translation += velocity.0 * time.delta_secs();
        transform.translation.y = transform.translation.y.max(0.);
    }
}


fn apply_gravity(mut objects : Query<&mut Velocity>, time : Res<Time>)
{
    let GRAVITY : Vec3 = Vec3::NEG_Y * 9.8;
    for mut object in objects 
    {
        let g = GRAVITY * time.delta_secs();
        object.0 += g;
    }
}

#[derive(Component)]
struct Player;

#[derive(Event, Deref)]
struct GrabEvent(bool);

#[derive(Event)]
struct BallSpawn {
    position: Vec3,
    velocity : Vec3,
}

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Resource)]
struct BallData {
    mesh: Handle<Mesh>,
    materials: Vec<Handle<StandardMaterial>>,
    rng: std::sync::Mutex<rand::rngs::StdRng>,
}

impl BallData {
    fn mesh(self: &Self) -> Handle<Mesh> {
        self.mesh.clone()
    }

    fn material(self: &Self) -> Handle<StandardMaterial> {
        use rand::seq::SliceRandom;
        let mut rng = self.rng.lock().unwrap();
        self.materials.choose(&mut *rng).unwrap().clone()
    }
}

impl FromWorld for BallData {
    fn from_world(world: &mut World) -> Self {
        let mesh = world.resource_mut::<Assets<Mesh>>().add(Sphere::new(1.));

        let mut material_resource = world.resource_mut::<Assets<StandardMaterial>>();
        let mut v: Vec<Handle<StandardMaterial>> = Vec::new();
        for i in 0..36 {
            let color = Color::hsl((i as f32 / 16.) * 360., 1., 0.5);
            v.push(material_resource.add(StandardMaterial {
                base_color: color,
                ..Default::default()
            }))
        }

        Self {
            mesh: mesh,
            materials: v,
            rng: Mutex::new(StdRng::seed_from_u64(123456789)),
        }
    }
}

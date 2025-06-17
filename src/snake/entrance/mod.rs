use bevy::prelude::*;
use std::time::Duration;
pub struct EntrancePlugin;

impl Plugin for EntrancePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(super::GameState::Main)
            .insert_resource(FadeTimer(Timer::new(
                Duration::from_secs(3),
                TimerMode::Repeating,
            )))
            .add_systems(
                OnEnter(super::GameState::Entrance),
                (spawn_ui, spawn_camera),
            )
            .add_systems(
                Update,
                tick_timer
                    .before(render_entrance)
                    .run_if(in_state(super::GameState::Entrance)),
            )
            .add_systems(
                Update,
                render_entrance.run_if(in_state(super::GameState::Entrance)),
            )
            .add_systems(
                OnExit(super::GameState::Entrance),
                (despawn_ui, despawn_camera),
            );
    }
}

#[derive(Resource)]
struct FadeTimer(Timer);

fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((Node {
            position_type: PositionType::Relative,
            width: Val::Percent(30.),
            height: Val::Percent(30.),
            left: Val::Percent(30.),
            top: Val::Percent(30.),
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn(ImageNode {
                image: asset_server.load("tupiniquim_logo.png"),
                ..Default::default()
            });
        });

    commands
        .spawn((Node {
            position_type: PositionType::Relative,
            width: Val::Percent(30.),
            height: Val::Percent(30.),
            left: Val::Percent(50.),
            top: Val::Percent(30.),
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn(ImageNode {
                image: asset_server.load("bevy_logo.png"),
                ..Default::default()
            });
        });
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn tick_timer(time: ResMut<Time>, mut timer: ResMut<FadeTimer>) {
    timer.0.tick(time.delta());
}

const ENTRANCE_TIME: f32 = 3.;

fn render_entrance(
    time: ResMut<Time>,
    timer: Res<FadeTimer>,
    mut next_state: ResMut<NextState<super::GameState>>,
) {
    if time.elapsed_secs() >= 3. {
        next_state.set(super::GameState::Main);
    }
}

fn despawn_ui(mut commands: Commands, node: Query<Entity, With<Node>>) {
    for node in node {
        commands.entity(node).despawn();
    }
}

fn despawn_camera(mut commands: Commands, camera: Query<Entity, With<Camera2d>>) {
    for camera in camera {
        commands.entity(camera).despawn();
    }
}

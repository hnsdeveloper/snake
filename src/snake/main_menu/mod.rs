use bevy::app::AppExit;
use bevy::prelude::*;

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(super::GameState::Main), (build_ui, spawn_camera))
            .add_systems(Update, update_ui.run_if(in_state(super::GameState::Main)))
            .add_systems(OnExit(super::GameState::Main), (despawn_camera, despawn_ui));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Component)]
enum ButtonType {
    PlayGame,
    Exit,
}

fn build_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(15.),
            height: Val::Percent(15.),
            left: Val::Percent(42.5),
            top: Val::Percent(42.5),
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![
            (
                Button,
                ButtonType::PlayGame,
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.),
                    height: Val::Percent(45.),
                    border: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                children![(
                    Text::new("Play game"),
                    TextColor(Color::srgb(1., 1., 1.)),
                    TextShadow::default()
                )]
            ),
            (
                Button,
                ButtonType::Exit,
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.),
                    height: Val::Percent(45.),
                    border: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    top: Val::Percent(55.),
                    ..default()
                },
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                children![(
                    Text::new("Exit"),
                    TextColor(Color::srgb(1., 1., 1.)),
                    TextShadow::default()
                )]
            )
        ],
    ));
}

fn update_ui(
    buttons: Query<(&Interaction, &ButtonType), Changed<Interaction>>,
    mut next_state: ResMut<NextState<super::GameState>>,
    mut e_writer: EventWriter<AppExit>,
) {
    for (interaction, button_type) in buttons {
        match interaction {
            Interaction::Pressed => match button_type {
                ButtonType::PlayGame => next_state.set(super::GameState::Gameplay),
                ButtonType::Exit => {
                    e_writer.write(AppExit::Success);
                }
            },
            _ => {}
        }
    }
}

fn despawn_camera(mut commands: Commands, camera: Single<(Entity, &Camera2d)>) {
    commands.entity(camera.0).despawn();
}

fn despawn_ui(mut commands: Commands, nodes: Query<(Entity, &Node)>) {
    for node in nodes {
        commands.entity(node.0).despawn();
    }
}

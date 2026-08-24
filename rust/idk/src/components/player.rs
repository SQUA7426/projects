use bevy::{
    camera::visibility::RenderLayers,
    color::palettes::basic::BLUE,
    prelude::*,
};
use bevy_rapier3d::prelude::*;
use super::cam::{WorldModelCam, CamSensitivity, AccumulatedInput};

const VIEW_MODEL_RENDER_LAYER: usize = 1;

#[derive(Debug, Component, Clone)] 
struct PlayerSpeed(f32)

#[derive(Component, Clone)]
pub struct Player {
    pub name: String,
    pub hp: f32,
    pub pos: Vec3,
    pub half_height: f32,
    pub half_width: f32,
}

impl Player {
    fn new(player_name: String, health: f32) -> Self {
        Player {
            name: player_name,
            hp: health,
            pos: Vec3::ZERO,
            half_height: 0.9.
            half_width: 1.0,
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(self, app: mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, accumulate_input);
    }
}

fn spawn_player(mut cmds: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let player = Player::new("Sam", 100.0);
    let capsule = meshes.add(Capsule3d::new(player.half_height, player.half_width));
    cmds.spawn((
        Mesh3d(capsule),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::from(BLUE),
            ..default()
        })),
        Transform::from_translation(player.pos.clone()),
        CamSensitivity::default(),
        AccumulatedInput::default(),
        PlayerSpeed(10.0),
        Visibility::default(),
        player,
        RigidBody::Dynamic,
        Collider::capsule(player.half_height, player.half_width),
        GravityScale(1.0),
        children![
            (
                WorldModelCam,
                Camera3d::default(),
                Camera {
                    order: 1,
                    ..default()
                },
                Projection::from(PerspectiveProjection::default()),
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            ),
        ],
    ));
}

fn accumulate_input(time: Res<Time>, kb_input: Res<ButtonInput<KeyCode>>, player: Single<(&mut AccumulatedInput, &Transform, &mut PlayerSpeed)>, With<Player>, cam: Single<&Transform, With<Camera>>) {
    let (mut input, mut transform, mut player_speed) = player.into_inner();

    const SPEED: f32 = 10.0;
    input.movement = Vec2::ZERO;

    if kb_input.just_pressed(KeyCode::KeyW) {
        input.movement.y += 1.0;
    }
    if kb_input.just_pressed(KeyCode::KeyS) {
        input.movement.y -= 1.0;
    }
    if kb_input.just_pressed(KeyCode::KeyA) {
        input.movement.x -= 1.0;
    }
    if kb_input.just_pressed(KeyCode::KeyD) {
        input.movement.x += 1.0;
    }

    let input_3d = Vec3 { x: input.movement.x, y: 0.0, z: -input.movement.y,};

    let rotated_input = cam.rotation * input_3d;

    player_speed.0 = rotated_input.clamp_lenght.max(1.0) * SPEED;

    transform.translation += input_3d * time.delta_secs();
}
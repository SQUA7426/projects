use super::cam::{AccumulatedInput, CamSensitivity, WorldModelCam};
use avian3d::prelude::LinearVelocity;
use bevy::{camera::visibility::RenderLayers, color::palettes::basic::BLUE, prelude::*};
use bevy_rapier3d::prelude::*;

const VIEW_MODEL_RENDER_LAYER: usize = 1;

#[derive(Debug, Component, PartialEq)]
struct PlayerSpeed(f32);

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
            pos: Vec3::new(0.0, 1.4, 0.0),
            half_height: 0.9,
            half_width: 1.0,
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, accumulate_input);
    }
}

fn spawn_player(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = Player::new("Sam".into(), 100.0);
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
        LinearVelocity::default(),
        Visibility::default(),
        RigidBody::Dynamic,
        Collider::cuboid(1.0, player.half_height, player.half_width),
        GravityScale(1.0),
        player,
        children![
            (
                WorldModelCam,
                Camera3d::default(),
                Projection::from(PerspectiveProjection {
                    fov: 90.0_f32.to_radians(),
                    ..default()
                }),
            ),
            (
                Camera3d::default(),
                Camera {
                    order: 1,
                    clear_color: ClearColorConfig::None,
                    ..default()
                },
                Projection::from(PerspectiveProjection {
                    fov: 70.0_f32.to_radians(),
                    ..default()
                }),
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            ),
        ],
    ));
}

fn accumulate_input(
    kb_input: Res<ButtonInput<KeyCode>>,
    player: Single<(&mut AccumulatedInput, &mut LinearVelocity), With<Player>>,
    cam: Single<&Transform, With<Camera>>,
) {
    let (mut input, mut velocity) = player.into_inner();

    const SPEED: f32 = 10.0;
    input.movement = Vec2::ZERO;

    if kb_input.pressed(KeyCode::KeyW) {
        input.movement.y += 1.0;
    }
    if kb_input.pressed(KeyCode::KeyS) {
        input.movement.y -= 1.0;
    }
    if kb_input.pressed(KeyCode::KeyA) {
        input.movement.x -= 1.0;
    }
    if kb_input.pressed(KeyCode::KeyD) {
        input.movement.x += 1.0;
    }

    let input_3d = Vec3 {
        x: input.movement.x,
        y: 0.0,
        z: -input.movement.y,
    };

    let (yaw, _pitch, _roll) = cam.rotation.to_euler(EulerRot::YXZ);
    let yaw_rot = Quat::from_rotation_y(yaw);
    let rotated_input = yaw_rot * input_3d;

    let horitontal = rotated_input.clamp_length_max(1.0) * SPEED;

    velocity.0.x = horitontal.x;
    velocity.0.z = horitontal.z;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_player() {
        let player = Player::new("Sam".into(), 100.0);
        assert_eq!(player.hp, 100.0);
        assert_eq!(player.pos, Vec3::new(0.0, 1.4, 0.0));
        assert_eq!(player.half_height, 0.9);
        assert_eq!(player.half_width, 1.0);

        let player_speed = PlayerSpeed(10.0);
        assert_eq!(player_speed, PlayerSpeed(10.));
    }

    #[test]
    fn player_plugin() {
        let mut app = App::new();
        app.add_plugins(PlayerPlugin).update();
        assert!(app.is_plugin_added::<PlayerPlugin>());
    }
}

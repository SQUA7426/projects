use super::cam::{CamSensitivity, WorldModelCam};
use bevy::{camera::visibility::RenderLayers, color::palettes::basic::BLUE, prelude::*};
use bevy_rapier3d::prelude::*;

const VIEW_MODEL_RENDER_LAYER: usize = 1;

#[derive(Debug, Component, PartialEq)]
struct PlayerSpeed(f32);

#[derive(Component, Clone, Debug)]
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
        app.add_systems(Startup, spawn_player);
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
        PlayerSpeed(10.0),
        Velocity {linear: Vec3::ZERO, angular: Vec3::ZERO},
        Visibility::default(),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
        Collider::cuboid(1.0, player.half_height, player.half_width),
        GravityScale(1.0),
        ColliderMassProperties::Mass(1.0),
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

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::{input::InputPlugin, state::app::StatesPlugin};

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
        app.add_plugins((MinimalPlugins, StatesPlugin, InputPlugin, AssetPlugin::default(), PlayerPlugin)).update();
        assert!(app.is_plugin_added::<PlayerPlugin>());
    }
}

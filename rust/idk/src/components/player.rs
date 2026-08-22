use bevy::{
    camera::visibility::RenderLayers,
    color::palettes::basic::BLUE,
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
};
use super::cam::{WorldModelCam, CamSensitivity};

const VIEW_MODEL_RENDER_LAYER: usize = 1;

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
        app.add_systems(spawn_player);
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
        Visibility::default(),
        player,
        children![
            (
                WorldModelCam,
                Camera3d::default(),
                Camera {
                    oreder: 1,
                    ..default()
                },
                Projection::from(PerspectiveProjection::default()),
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            ),
        ],
    ));
}

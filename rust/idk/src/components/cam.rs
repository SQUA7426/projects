use bevy::{
    camera::Viewport,
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};
use bevy_rapier3d::dynamics::Velocity;
use std::f32::consts::FRAC_PI_2;

use crate::components::player::Player;

#[derive(Debug, Component, Deref, DerefMut)]
pub struct CamSensitivity(Vec2);

impl Default for CamSensitivity {
    fn default() -> Self {
        Self(Vec2::new(0.5, 1.0))
    }
}

#[derive(Debug, Component)]
pub struct WorldModelCam;

#[derive(Debug)]
pub struct CamPlugin;

impl Plugin for CamPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_map, grab_cursor))
            .add_systems(Update, (rotate_cam, accumulate_input).chain());
    }
}

fn grab_cursor(mut cursor_options: Query<&mut CursorOptions, With<PrimaryWindow>>) {
    if let Ok(mut cursor) = cursor_options.single_mut() {
        cursor.grab_mode = CursorGrabMode::Locked;
        cursor.visible = false;
    }
}

fn rotate_cam(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player: Single<(&mut Transform, &CamSensitivity), With<Player>>,
) {
    let delta = accumulated_mouse_motion.delta;
    if delta == Vec2::ZERO {
        return;
    }

    const ROT_SPEED: f32 = 0.1;

    let (mut transform, cam_sensitivity) = player.into_inner();
    let dy = -delta.x * cam_sensitivity.x * ROT_SPEED;
    let _dp = -delta.y * cam_sensitivity.y * ROT_SPEED;

    let (yaw, _pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
    let accumulated_yaw = yaw + dy;

    transform.rotation = Quat::from_euler(EulerRot::YXZ, accumulated_yaw, 0.0, roll);
}

fn accumulate_input(
    kb_input: Res<ButtonInput<KeyCode>>,
    player: Single<(&mut Velocity, &Transform), With<Player>>,
) {
    let (mut velocity, transform) = player.into_inner();

    const SPEED: f32 = 10.0;
    let mut input = Vec2::ZERO;

    if kb_input.pressed(KeyCode::KeyW) {
        input.y += 1.0;
    }
    if kb_input.pressed(KeyCode::KeyS) {
        input.y -= 1.0;
    }
    if kb_input.pressed(KeyCode::KeyA) {
        input.x -= 1.0;
    }
    if kb_input.pressed(KeyCode::KeyD) {
        input.x += 1.0;
    }

    let input_3d = Vec3 {
        x: input.x,
        y: 0.0,
        z: -input.y,
    };

    let (yaw, _pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
    let yaw_rot = Quat::from_rotation_y(yaw);
    let rotated_input = yaw_rot * input_3d;

    let horitontal = rotated_input.clamp_length_max(1.0) * SPEED;

    velocity.linear.x = horitontal.x;
    velocity.linear.z = horitontal.z;
}

fn spawn_map(mut cmds: Commands) {
    cmds.spawn((
        Camera3d::default(),
        Camera {
            order: 2,
            clear_color: ClearColorConfig::None,
            viewport: Some(Viewport {
                physical_position: UVec2::ZERO,
                physical_size: UVec2::new(150, 150),
                ..default()
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 100.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

#[cfg(test)]
mod tests {
    use bevy::state::app::StatesPlugin;

    use super::*;

    #[test]
    fn cam_plugin() {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            StatesPlugin,
            AssetPlugin::default(),
            CamPlugin,
        ))
        .update();
        assert!(app.is_plugin_added::<CamPlugin>());
    }
}

use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};
use std::f32::consts::FRAC_PI_2;

use crate::components::player::Player;

#[derive(Debug, Component, Clone, PartialEq, Default, Deref, DerefMut)]
pub struct AccumulatedInput {
    pub movement: Vec2,
}

#[derive(Debug, Component, Deref, DerefMut)]
pub struct CamSensitivity(Vec2);

impl Default for CamSensitivity {
    fn default() -> Self {
        Self(Vec2::new(0.5, 1.0))
    }
}

#[derive(Debug, Component)]
pub struct WorldModelCam;

pub struct CamPlugin;

impl Plugin for CamPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_cam)
            .add_systems(Update, rotate_cam);
    }
}

fn spawn_cam(mut cmds: Commands) {
    cmds.spawn((
            Camera::default(),
            Camera3d::default(),
            Transform::from_xyz(0.0, 100.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn rotate_cam(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player: Single<&CamSensitivity, With<Player>>,
    mut cam: Single<&mut Transform, With<Camera>>,
) {
    let cam_sensitivity = player.into_inner();
    let delta = accumulated_mouse_motion.delta;
    if delta != Vec2::ZERO {
        let delta_yaw = -delta.x * cam_sensitivity.x;
        let delta_pitch = -delta.y * cam_sensitivity.y;

        let (yaw, pitch, roll) = cam.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        cam.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cam_plugin() {
        let mut app = App::new();
        app.add_plugins(CamPlugin).update();
        assert!(app.is_plugin_added::<CamPlugin>());
    }
}

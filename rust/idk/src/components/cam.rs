use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
};
use std::f32::const::FRAC_PI_2;

#[derive(Debug, Component, Clone, PartitialEq, Default, Deref, DerefMut)]
pub struct AccumulatedInput {
    movement: Vec2,
}

#[derive(Debug, Component, Deref, DerefMut)]
pub struct CamSensitivity(Vec2);

impl Default for CamSensitivity {
    fn default() -> Self {
        Self(Vec2::new(0.5, 1.0)),
    }
}

#[derive(Debug, Component)]
pub struct WorldModelCam;

pub struct CamPlugin;

impl Plugin for CamPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (rotate_camm, translate_cam));
    }
}

fn rotate_cam(accumulated_mouse_motion: Res<AccumulatedMouseMotion>, player: Single<(&mut Transform, &CamSensitivity), With<Camera>>) {
    let (mut transform, cam_sensititvity) player.into_inner();
    let delta = accumulated_mouse_motion.delta;
    if delta != Vec2::ZERO {
        let delta_yaw = -delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

fn translate_cam(mut cam: Single<&mut Transform. With<Camera>>, player: Single<&Transform, (With<AccumulatedInput>, Without<Camera>)>) {
    cam.translation = player.translation;
}
use super::room::Rooms;
use bevy::{color::palettes::css::ORANGE, prelude::*};
use bevy_rapier3d::prelude::*;

pub struct ApartmentPlugin;

impl Plugin for ApartmentPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<Rooms>()
            .add_systems(Startup, (setup_apartment, spawn_light));
    }
}

fn spawn_light(mut cmds: Commands) {
    cmds.spawn((
        PointLight {
            radius: 15.0,
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 5.0, 0.0)),
    ));
}

fn setup_apartment(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let normal = Vec3::new(0.0, 10.0, 0.0);
    cmds.spawn((
        Mesh3d(meshes.add(Plane3d::new(normal, Vec2::splat(20.0)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::from(ORANGE),
            ..default()
        })),
        Visibility::Visible,
        RigidBody::Fixed,
        ColliderMassProperties::Mass(1.0),
        Collider::cuboid(10.0, 0.1, 10.0),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apartment_plugin() {
        let mut app = App::new();
        app.add_plugins(ApartmentPlugin);
        assert!(app.is_plugin_added::<ApartmentPlugin>());
    }
}

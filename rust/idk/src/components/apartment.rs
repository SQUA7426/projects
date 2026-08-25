use bevy::{
    color::palettes::basic::ORANGE,
    prelude::*,
};
use bevy_rapier3d::prelude::*;
use super::Rooms;

pub struct ApartmentPlugin;

impl Plugin for ApartmentPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<Rooms>()
            .add_systems(startup, (setup_apartment, spawn_light));
    }
}

fn spawn_light(mut cmds: Commands) {
    cmds.spawn((
        PointLight {
            radius: 5.0,
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 2.0, 0.0)),
    ));
}

fn setup_apartment(mut cmds: Commands, mut meshes: ResMut<Assets<Meshes>>, mut materials: ResMut<Asstes<StandardMaterial>>) {
    let normal = Vec3::new(0.0, 1.0, 0.0);
    let half_plane_size = Vec2::new(5.0, 2.0);
    cmds.spawn((
        Mesh3d(meshes.add(Plane3d::new(normal, half_plane_size))),
        MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::from(Orange),
                ..default()
            },
        ))
        Visibility::default(),
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
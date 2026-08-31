use super::room::Rooms;
use bevy::{
    animation::graph::AnimationGraph,
    // camera::visibility::RenderLayers,
    color::palettes::css::{BLACK, BLUE, GREEN, ORANGE, RED},
    prelude::*,
};
use bevy_rapier3d::prelude::*;

#[derive(Debug)]
pub struct ApartmentPlugin;

impl Plugin for ApartmentPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<AnimationGraph>()
            .init_asset::<AnimationClip>()
            .init_asset::<WorldAsset>()
            .init_state::<Rooms>()
            .add_systems(Startup, setup_apartment);
    }
}

fn setup_apartment(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let normal = Vec3::new(0.0, 10.0, 0.0);
    cmds.spawn((
        Mesh3d(meshes.add(Plane3d::new(normal, Vec2::splat(50.0)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::from(ORANGE),
            ..default()
        })),
        Visibility::Visible,
        RigidBody::Fixed,
        ColliderMassProperties::Mass(1.0),
        Collider::cuboid(50.0, 0.1, 50.0),
    ));

    let colors = [RED, BLUE, GREEN, BLACK];

    [
        Vec3::new(10.0, 0.5, 10.0),
        Vec3::new(-10.0, 0.5, 10.0),
        Vec3::new(10.0, 0.5, -10.0),
        Vec3::new(-10.0, 0.5, -10.0),
    ]
    .iter()
    .enumerate()
    .for_each(|(i, v)| {
        let material_color: Color = Color::from(colors[i]);
        cmds.spawn((
            Transform::from_translation(*v).with_scale(Vec3::new(4.0, 4.0, 4.0)),
            Mesh3d(meshes.add(Cuboid::default())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: material_color,
                ..default()
            })),
            RigidBody::Fixed,
            ColliderMassProperties::Mass(1.0),
            Collider::cuboid(2.0, 2.0, 0.0),
        ));
    });
}

#[cfg(test)]
mod tests {
    use bevy::state::app::StatesPlugin;

    use super::*;

    #[test]
    fn apartment_plugin() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, StatesPlugin, ApartmentPlugin));
        assert!(app.is_plugin_added::<ApartmentPlugin>());
    }
}

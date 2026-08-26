use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use idk::components::{apartment::ApartmentPlugin, cam::CamPlugin, player::PlayerPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Screen".into(),
                        ..default()
                    }),
                    ..default()
                }),
            RapierPhysicsPlugin::<NoUserData>::default(),
        ))
        .add_plugins((ApartmentPlugin, CamPlugin, PlayerPlugin))
        .run();
}

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use idk::components::{animations::AnimationsPlugin, apartment::ApartmentPlugin, cam::CamPlugin, items::inventory::InventoryPlugin, player::PlayerPlugin, sound::SoundPlugin};

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
        .add_plugins((AnimationsPlugin,ApartmentPlugin, CamPlugin, PlayerPlugin, SoundPlugin, InventoryPlugin))
        .run();
}

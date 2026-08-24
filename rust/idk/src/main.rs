use bevy::prelude::*;

use idk::components::{ApartmentPlugin, CamPlugin, PlayerPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugin,
            RapierPhysicsPlugin::<NoUserData>::default()
        )),
        .add_systems((ApartmentPlugin, CamPlugin, PlayerPlugin))
        .run();
}
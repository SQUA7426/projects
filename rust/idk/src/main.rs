use bevy::prelude::*;

use idk::components::{CamPlugin, PlayerPlugin};

fn main() {
    App::new()
    .add_systems((PlayerPlugin, CamPlugin))
    .run();
}
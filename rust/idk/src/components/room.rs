use bevy::prelude::*;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
pub enum Rooms {
    #[default]
    Hallway,
    Room_X,
}
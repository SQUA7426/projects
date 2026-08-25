use bevy::prelude::*;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
pub enum Rooms {
    #[default]
    Hallway,
    Room_X,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_room() {
        let default_room = Rooms::default();
        assert_eq!(default_room, Rooms::Hallway);
    }
}
pub mod components {
    pub mod items {
        pub mod inventory;
    }
    pub mod animations;
    pub mod apartment;
    pub mod cam;
    pub mod player;
    pub mod room;
    pub mod sound;
}

#[allow(unused_imports)]
use components::{items::{inventory::*,},animations::*, apartment::*, cam::*, player::*, room::*, sound::*};

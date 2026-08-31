use bevy::prelude::*;
use bevy_audio::AudioSource;

#[derive(Resource, Deref)]
pub struct SoundEffect {
    handle: Handle<AudioSource>,
}

impl FromWorld for SoundEffect {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        SoundEffect {
            handle: asset_server.load("sounds/flashlight.mp3"),
        }
    }
}

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<AudioSource>()
            .init_resource::<SoundEffect>();
    }
}

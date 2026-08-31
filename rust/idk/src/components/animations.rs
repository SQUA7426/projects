use bevy::{
    animation::{AnimationPlayer, graph::AnimationGraph},
    input::common_conditions::input_just_pressed,
    platform::collections::HashMap,
    prelude::AnimationNodeIndex,
    prelude::*,
    world_serialization::WorldInstanceReady,
};

use crate::components::items::inventory::flashlight::{FlashLightOn, PlayerFlashLight};

pub struct AnimationsPlugin;

impl Plugin for AnimationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            flashlight_player.run_if(input_just_pressed(KeyCode::KeyF)),
        );
    }
}

#[derive(Component, Clone)]
pub struct FlashLightAnimation {
    pub handle: Handle<AnimationGraph>,
    pub node_indices: HashMap<String, AnimationNodeIndex>,
}

pub fn flashlight_animation_ready(
    scene_ready: On<WorldInstanceReady>,
    mut cmds: Commands,
    children: Query<&Children>,
    animations: Query<&FlashLightAnimation>,
    players: Query<&AnimationPlayer>,
) {
    let Ok(animation_data) = animations.get(scene_ready.entity) else {
        return;
    };

    for child in children.iter_descendants(scene_ready.entity) {
        if players.get(child).is_ok() {
            cmds.entity(child)
                .insert(AnimationGraphHandle(animation_data.handle.clone()))
                .insert(animation_data.clone());
        }
    }
}

pub fn flashlight_player(
    flashlight_query: Single<(&mut AnimationPlayer, &FlashLightAnimation), With<PlayerFlashLight>>,
    flashlight_on: Res<FlashLightOn>,
) {
    info!("Flashlight PLAYER...");
    let (mut player, animations) = flashlight_query.into_inner();
    let Some(&flashlight_idx1) = animations.node_indices.get("ON_OFF") else {
        return;
    };

    let Some(&flashlight_idx2) = animations.node_indices.get("FlashlightAction") else {
        return;
    };

    play_animation(flashlight_idx1, &mut player, flashlight_on.0);
    play_animation(flashlight_idx2, &mut player, flashlight_on.0);
}

fn play_animation(flashlight_idx: AnimationNodeIndex, player: &mut AnimationPlayer, on: bool) {
    if let Some(action) = player.animation(flashlight_idx)
        && (action.is_finished() || action.is_paused())
    {
        if on {
            info!("Flashlight ON");
            player.play(flashlight_idx).replay();
            player.adjust_speeds(-1.0);
        } else {
            info!("Flashlight OFF");
            player.play(flashlight_idx);
            player.adjust_speeds(1.0);
        }
        return;
    }

    player.play(flashlight_idx);
    player.adjust_speeds(-1.0);
}

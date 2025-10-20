use super::NPC;
use bevy::prelude::*;
pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, npc_behavior_system);
    }
}

// TODO: fire once per second
fn npc_behavior_system(query: Query<&Name, With<NPC>>) {
    // TODO: redo goap priority here.
    for name in &query {
        info!("NPC '{}' is thinking...", name.as_str());
    }
}

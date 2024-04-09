use bevy::prelude::*;

pub enum AIMode {
    Hunter, // hunts the player - default
    Tracker, // tracking the player
    Wander, // wanders the map
    Follower, // follows other bots
    Howler, // attracts other bots when spotting
    Patrol, // will pick two points to patrol between
    Stalker, // silent enemy will only watch until another is near

    Overseer, // hovers above the map searching for players above the walls

    Cheating, // knows where the player is
}

#[derive(Component)]
pub struct Bot {
    mode: AIMode,
    temper: f32,
    view_distance: f32,
}


pub struct BotPlugin;
impl Plugin for BotPlugin {
    fn build(&self, app: &mut App) {
        
    }
}

fn spawn_bot() {

}
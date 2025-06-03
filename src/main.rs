use bevy::{DefaultPlugins, app::App};

use cellular_automata::CellularAutomataPlugin;
use game_of_life::GameOfLifePlugin;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(GameOfLifePlugin)
    .run();
}

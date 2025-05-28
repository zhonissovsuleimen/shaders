use bevy::{DefaultPlugins, app::App};

use cellular_automata::CellularAutomataPlugin;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(CellularAutomataPlugin(10))
    .run();
}

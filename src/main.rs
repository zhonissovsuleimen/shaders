use bevy::app::PluginGroup;
use bevy::{
  DefaultPlugins,
  app::App,
  render::texture::ImagePlugin,
  utils::default,
  window::{Window, WindowPlugin},
};

use cellular_automata::CellularAutomataPlugin;
use game_of_life::GameOfLifePlugin;

fn main() {
  App::new()
    .add_plugins((DefaultPlugins
      .set(WindowPlugin {
        primary_window: Some(Window {
          resizable: false,
          present_mode: bevy::window::PresentMode::AutoNoVsync,
          ..default()
        }),
        ..default()
      })
      .set(ImagePlugin::default_nearest()),))
    .add_plugins(GameOfLifePlugin)
    .run();
}

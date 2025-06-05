use std::sync::{Arc, Mutex};

use bevy::{
  asset::Handle,
  ecs::resource::Resource,
  image::Image,
  math::{IVec2, Vec2},
  render::{
    extract_resource::ExtractResource,
    render_resource::{Buffer, ShaderType},
  },
};
use bytemuck::{Pod, Zeroable};

#[derive(Resource, ExtractResource, Clone)]
pub struct MainImage(pub Handle<Image>);

#[repr(C)]
#[derive(Resource, Clone, Copy, ShaderType, Pod, Zeroable, ExtractResource)]
pub struct Params {
  pub buffer_size_x: u32,
  pub buffer_size_y: u32,
  pub center_x: f32,
  pub center_y: f32,
  pub resolution_x: u32,
  pub resolution_y: u32,
  pub random_seed: u32,
  pub zoom: f32,
}

#[derive(Resource)]
pub struct GpuParamsHandle(pub Buffer);

#[derive(Resource, ExtractResource, Clone, Default, PartialEq)]
pub enum ComputeState {
  #[default]
  INITIAL,
  RANDOMIZE,
  STEP,
  WAIT,
}

#[derive(Resource, Default)]
pub struct MouseData {
  pub pos: Option<Vec2>,
}

#[derive(Resource, Default)]
pub struct WindowData {
  pub pos: Option<IVec2>,
}

#[derive(Resource, Clone, ExtractResource)]
pub struct Telemetry {
  pub ticks_len: usize,
  pub ticks: Arc<Mutex<Vec<f32>>>,
}

impl Default for Telemetry {
  fn default() -> Self {
    let len = 1000;
    Self {
      ticks_len: len,
      ticks: Arc::new(Mutex::new(Vec::<f32>::with_capacity(len))),
    }
  }
}

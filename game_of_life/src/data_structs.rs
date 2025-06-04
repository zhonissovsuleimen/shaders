use std::sync::{Arc, Mutex};

use bevy::{
  asset::Handle,
  ecs::resource::Resource,
  image::Image,
  render::{extract_resource::ExtractResource, render_resource::{Buffer, ShaderType}},
};
use bytemuck::{Pod, Zeroable};

#[derive(Resource, ExtractResource, Clone)]
pub struct MainImage(pub Handle<Image>);

#[repr(C)]
#[derive(Resource, Clone, Copy, ShaderType, Pod, Zeroable, ExtractResource)]
pub struct Params {
  pub buffer_size_x: u32,
  pub buffer_size_y: u32,
  pub buffer_size: u32,
  pub center_x: u32,
  pub center_y: u32,
  pub resolution_x: u32,
  pub resolution_y: u32,
  pub zoom: f32,
}

#[derive(Resource)]
pub struct GpuParamsHandle(pub Buffer);

#[derive(Resource, ExtractResource, Clone, Default, PartialEq)]
pub enum ComputeState {
  #[default]
  STEP,
  WAIT,
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
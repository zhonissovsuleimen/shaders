use bevy::{
  asset::Handle,
  ecs::resource::Resource,
  image::Image,
  render::{extract_resource::ExtractResource, render_resource::ShaderType},
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
  pub offset_x: u32,
  pub offset_y: u32,
  pub zoom: f32,
}

#[derive(Resource, Clone, ExtractResource)]
pub struct Resolution(pub u32, pub u32);

#[derive(Resource, ExtractResource, Clone, Default, PartialEq)]
pub enum ComputeState {
  #[default]
  STEP,
  WAIT,
}

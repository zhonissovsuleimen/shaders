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
  pub cell_size: u32,
  pub count_x: u32,
  pub count_y: u32,
}

#[repr(C)]
#[derive(Resource, Clone, ExtractResource)]
pub struct CellValues(pub Vec<u32>);

#[derive(Resource)]
pub struct CellSize(pub u32);

#[derive(Resource, Clone, ExtractResource)]
pub struct Resolution(pub u32, pub u32);

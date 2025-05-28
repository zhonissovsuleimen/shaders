use bevy::{
  ecs::{
    resource::Resource,
    system::{Commands, Res},
  },
  render::{
    render_asset::RenderAssets,
    render_resource::{BindGroup, BindGroupEntries, BufferInitDescriptor, BufferUsages},
    renderer::RenderDevice,
    texture::GpuImage,
  },
};

use crate::{
  data_structs::{CellValues, MainImage, Params},
  pipeline::CAPipeline,
};

#[derive(Resource)]
pub struct CABindGroup(pub BindGroup);

pub fn prepare_bind_group(
  mut commands: Commands,
  pipeline: Res<CAPipeline>,
  gpu_images: Res<RenderAssets<GpuImage>>,
  device: Res<RenderDevice>,
  ca_params: Res<Params>,
  ca_image: Res<MainImage>,
  ca_values: Res<CellValues>,
) {
  if let Some(main_image) = gpu_images.get(&ca_image.0) {
    let params_buffer = device.create_buffer_with_data(&BufferInitDescriptor {
      label: None,
      contents: bytemuck::bytes_of(&*ca_params),
      usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
    });

    let values_buffer = device.create_buffer_with_data(&BufferInitDescriptor {
      label: None,
      contents: bytemuck::cast_slice(&*ca_values.0),
      usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
    });

    let bind_group = device.create_bind_group(
      None,
      &pipeline.layout,
      &BindGroupEntries::sequential((
        params_buffer.as_entire_binding(),
        &main_image.texture_view,
        values_buffer.as_entire_binding(),
      )),
    );

    commands.insert_resource(CABindGroup(bind_group));
  }
}

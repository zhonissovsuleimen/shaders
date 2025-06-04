use bevy::{
  ecs::{
    resource::Resource,
    system::{Commands, Res},
  },
  render::{
    render_asset::RenderAssets,
    render_resource::{BindGroup, BindGroupEntries, BufferInitDescriptor, BufferUsages},
    renderer::{RenderDevice, RenderQueue},
    texture::GpuImage,
  },
};

use crate::{
  data_structs::{GpuParamsHandle, MainImage, Params},
  pipeline::GLPipeline,
};

#[derive(Resource)]
pub struct GLBindGroup(pub BindGroup);

pub fn prepare_bind_group(
  mut commands: Commands,
  pipeline: Res<GLPipeline>,
  gpu_images: Res<RenderAssets<GpuImage>>,
  device: Res<RenderDevice>,
  params: Res<Params>,
  main_image: Res<MainImage>,
) {
  if let Some(main_image) = gpu_images.get(&main_image.0) {
    let params_buffer = device.create_buffer_with_data(&BufferInitDescriptor {
      label: None,
      contents: bytemuck::bytes_of(&*params),
      usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
    });

    let buffer_size = params.buffer_size_x * params.buffer_size_y;
    let data_buffer = vec![0u32; buffer_size as usize];

    let buffer = device.create_buffer_with_data(&BufferInitDescriptor {
      label: None,
      contents: bytemuck::cast_slice(&data_buffer.clone()),
      usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
    });

    let bind_group = device.create_bind_group(
      None,
      &pipeline.layout,
      &BindGroupEntries::sequential((
        params_buffer.as_entire_binding(),
        &main_image.texture_view,
        buffer.as_entire_binding(),
      )),
    );

    commands.insert_resource(GLBindGroup(bind_group));
    commands.insert_resource(GpuParamsHandle(params_buffer));
  }
}

pub fn sync_params(
  params: Res<Params>,
  gpu_params_handle: Res<GpuParamsHandle>,
  render_queue: Res<RenderQueue>,
) {
  render_queue.write_buffer(&gpu_params_handle.0, 0, bytemuck::bytes_of(&*params));
}

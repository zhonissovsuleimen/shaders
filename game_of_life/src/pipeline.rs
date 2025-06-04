use bevy::{
  asset::DirectAssetAccessExt,
  ecs::{
    resource::Resource,
    world::{FromWorld, World},
  },
  render::{
    render_resource::{
      BindGroupLayout, BindGroupLayoutEntries, CachedComputePipelineId, ComputePipelineDescriptor,
      PipelineCache, ShaderStages, StorageTextureAccess, TextureFormat,
      binding_types::{storage_buffer, texture_storage_2d, uniform_buffer},
    },
    renderer::RenderDevice,
  },
};

use crate::data_structs::Params;

#[derive(Resource)]
pub struct GLPipeline {
  pub layout: BindGroupLayout,
  pub update_pipeline: CachedComputePipelineId,
  pub randomize_pipeline: CachedComputePipelineId,
  pub display_pipeline: CachedComputePipelineId,
}

impl FromWorld for GLPipeline {
  fn from_world(world: &mut World) -> Self {
    let device = world.resource::<RenderDevice>();
    let pipeline_cache = world.resource::<PipelineCache>();

    let layout = device.create_bind_group_layout(
      None,
      &BindGroupLayoutEntries::sequential(
        ShaderStages::COMPUTE,
        (
          uniform_buffer::<Params>(false),
          texture_storage_2d(TextureFormat::Rgba8Unorm, StorageTextureAccess::ReadWrite),
          storage_buffer::<Vec<u32>>(false),
        ),
      ),
    );

    const SHADER_PATH: &str = "shaders/game_of_life.wgsl";
    let shader = world.load_asset(SHADER_PATH);

    let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
      label: None,
      layout: vec![layout.clone()],
      push_constant_ranges: vec![],
      shader: shader.clone(),
      shader_defs: vec![],
      entry_point: "update".into(),
      zero_initialize_workgroup_memory: false,
    });

    let randomize_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
      label: None,
      layout: vec![layout.clone()],
      push_constant_ranges: vec![],
      shader: shader.clone(),
      shader_defs: vec![],
      entry_point: "randomize".into(),
      zero_initialize_workgroup_memory: false,
    });

    let display_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
      label: None,
      layout: vec![layout.clone()],
      push_constant_ranges: vec![],
      shader: shader.clone(),
      shader_defs: vec![],
      entry_point: "display".into(),
      zero_initialize_workgroup_memory: false,
    });

    GLPipeline {
      layout,
      update_pipeline,
      randomize_pipeline,
      display_pipeline,
    }
  }
}

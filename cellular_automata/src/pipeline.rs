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
pub struct CAPipeline {
  pub layout: BindGroupLayout,
  pub compute_pipeline: CachedComputePipelineId,
  pub display_pipeline: CachedComputePipelineId,
}

impl FromWorld for CAPipeline {
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

    const SHADER_PATH: &str = "shaders/cellular_automata.wgsl";
    let shader = world.load_asset(SHADER_PATH);
    let compute_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
      label: None,
      layout: vec![layout.clone()],
      push_constant_ranges: vec![],
      shader: shader.clone(),
      shader_defs: vec![],
      entry_point: "compute".into(),
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

    CAPipeline {
      layout,
      compute_pipeline,
      display_pipeline,
    }
  }
}

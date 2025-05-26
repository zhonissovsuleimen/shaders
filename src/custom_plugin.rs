use bevy::{
  app::Plugin,
  asset::{DirectAssetAccessExt, Handle},
  ecs::{
    resource::Resource,
    schedule::IntoScheduleConfigs,
    system::{Commands, Res},
    world::FromWorld,
  },
  image::Image,
  render::{
    Render, RenderApp, RenderSet,
    extract_resource::{ExtractResource, ExtractResourcePlugin},
    graph::CameraDriverLabel,
    render_asset::RenderAssets,
    render_graph::{self, RenderGraph, RenderLabel},
    render_resource::{
      BindGroup, BindGroupEntries, BindGroupLayout, BindGroupLayoutEntries, BufferInitDescriptor,
      BufferUsages, CachedComputePipelineId, ComputePassDescriptor, ComputePipelineDescriptor,
      PipelineCache, ShaderStages, StorageTextureAccess, TextureFormat,
      binding_types::{texture_storage_2d, uniform_buffer},
    },
    renderer::RenderDevice,
    texture::GpuImage,
  },
};

use crate::Resolution;

const WORKGROUP_SIZE: u32 = 8;
pub struct CustomPlugin;

#[derive(Resource)]
pub struct CustomBindGroup(BindGroup);

impl Plugin for CustomPlugin {
  fn build(&self, app: &mut bevy::app::App) {
    app.add_plugins(ExtractResourcePlugin::<StorageHandles>::default());
    app.add_plugins(ExtractResourcePlugin::<Resolution>::default());

    let render_app = app.sub_app_mut(RenderApp);
    render_app.add_systems(
      Render,
      prepare_bind_group.in_set(RenderSet::PrepareBindGroups),
    );

    let mut render_graph = render_app.world_mut().resource_mut::<RenderGraph>();
    render_graph.add_node(CustomShaderLabel, CustomShaderNode);
    render_graph.add_node_edge(CustomShaderLabel, CameraDriverLabel);
  }

  fn finish(&self, app: &mut bevy::app::App) {
    app.sub_app_mut(RenderApp).init_resource::<CustomPipeline>();
  }
}

fn prepare_bind_group(
  mut commands: Commands,
  pipeline: Res<CustomPipeline>,
  gpu_images: Res<RenderAssets<GpuImage>>,
  storage_handles: Res<StorageHandles>,
  render_device: Res<RenderDevice>,
) {
  if let Some(main_view) = gpu_images.get(&storage_handles.main_image) {
    let time_bytes = storage_handles.time.to_ne_bytes();
    let time_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
      label: None,
      contents: &time_bytes,
      usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    });

    let bind_group = render_device.create_bind_group(
      None,
      &pipeline.layout,
      &BindGroupEntries::sequential((
        time_buffer.as_entire_buffer_binding(),
        &main_view.texture_view,
      )),
    );

    commands.insert_resource(CustomBindGroup(bind_group));
  }
}

#[derive(Resource, ExtractResource, Clone)]
pub struct StorageHandles {
  pub main_image: Handle<Image>,
  pub time: f32,
}

#[derive(Resource)]
struct CustomPipeline {
  layout: BindGroupLayout,
  pipeline: CachedComputePipelineId,
}

impl FromWorld for CustomPipeline {
  fn from_world(world: &mut bevy::ecs::world::World) -> Self {
    let device = world.resource::<RenderDevice>();
    let pipeline_cache = world.resource::<PipelineCache>();

    let layout = device.create_bind_group_layout(
      None,
      &BindGroupLayoutEntries::sequential(
        ShaderStages::COMPUTE,
        (
          uniform_buffer::<f32>(false),
          texture_storage_2d(TextureFormat::Rgba8Unorm, StorageTextureAccess::ReadWrite),
        ),
      ),
    );

    const SHADER_PATH: &str = "shaders/shader.wgsl";
    let shader = world.load_asset(SHADER_PATH);
    let pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
      label: None,
      layout: vec![layout.clone()],
      push_constant_ranges: vec![],
      shader,
      shader_defs: vec![],
      entry_point: "main".into(),
      zero_initialize_workgroup_memory: false,
    });

    CustomPipeline { layout, pipeline }
  }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct CustomShaderLabel;
struct CustomShaderNode;
impl render_graph::Node for CustomShaderNode {
  fn run<'w>(
    &self,
    _graph: &mut render_graph::RenderGraphContext,
    render_context: &mut bevy::render::renderer::RenderContext<'w>,
    world: &'w bevy::ecs::world::World,
  ) -> Result<(), render_graph::NodeRunError> {
    let bind_group = &world.resource::<CustomBindGroup>().0;
    let pipeline_cache = world.resource::<PipelineCache>();
    let pipeline = world.resource::<CustomPipeline>();
    let resolution = world.resource::<Resolution>();

    let mut pass = render_context
      .command_encoder()
      .begin_compute_pass(&ComputePassDescriptor::default());

    if let Some(pipeline) = pipeline_cache.get_compute_pipeline(pipeline.pipeline) {
      pass.set_bind_group(0, bind_group, &[]);
      pass.set_pipeline(pipeline);

      let x = (resolution.0 as u32 + WORKGROUP_SIZE - 1) / WORKGROUP_SIZE;
      let y = (resolution.1 as u32 + WORKGROUP_SIZE - 1) / WORKGROUP_SIZE;
      
      pass.dispatch_workgroups(x, y, 1);
    }

    Ok(())
  }
}

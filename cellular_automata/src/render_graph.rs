use bevy::{
  ecs::world::World,
  render::{
    render_graph::{self, RenderLabel},
    render_resource::{ComputePassDescriptor, PipelineCache},
    renderer::RenderContext,
  },
};

use crate::{
  bind_group::CABindGroup,
  data_structs::{Params, Resolution},
  pipeline::CAPipeline,
};

const WORKGROUP_SIZE: u32 = 16;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct CANodeLabel;

#[derive(Default)]
pub struct CANode;

impl render_graph::Node for CANode {
  fn run(
    &self,
    _graph: &mut render_graph::RenderGraphContext,
    render_context: &mut RenderContext,
    world: &World,
  ) -> Result<(), render_graph::NodeRunError> {
    let pipeline_cache = world.resource::<PipelineCache>();
    let pipeline = world.resource::<CAPipeline>();
    let bind_group = world.resource::<CABindGroup>();

    if let Some(compute_pipeline) = pipeline_cache.get_compute_pipeline(pipeline.compute_pipeline) {
      let mut pass = render_context
        .command_encoder()
        .begin_compute_pass(&ComputePassDescriptor::default());

      pass.set_bind_group(0, &bind_group.0, &[]);
      pass.set_pipeline(compute_pipeline);

      if let Some(params) = world.get_resource::<Params>() {
        let wg_x = (params.count_x + WORKGROUP_SIZE - 1) / (WORKGROUP_SIZE);
        let wg_y = (params.count_y + WORKGROUP_SIZE - 1) / (WORKGROUP_SIZE);
        pass.dispatch_workgroups(wg_x, wg_y, 1);
      }
    }

    if let Some(display_pipeline) = pipeline_cache.get_compute_pipeline(pipeline.display_pipeline) {
      let mut pass = render_context
        .command_encoder()
        .begin_compute_pass(&ComputePassDescriptor::default());

      pass.set_bind_group(0, &bind_group.0, &[]);
      pass.set_pipeline(display_pipeline);

      if let Some(res) = world.get_resource::<Resolution>() {
        let wg_x = (res.0 + WORKGROUP_SIZE - 1) / (WORKGROUP_SIZE);
        let wg_y = (res.1 + WORKGROUP_SIZE - 1) / (WORKGROUP_SIZE);
        pass.dispatch_workgroups(wg_x, wg_y, 1);
      }
    }
    Ok(())
  }
}

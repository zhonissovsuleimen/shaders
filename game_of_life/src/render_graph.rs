use bevy::{
  ecs::world::World,
  render::{
    render_graph::{self, RenderLabel},
    render_resource::{ComputePassDescriptor, PipelineCache},
    renderer::RenderContext,
  },
  time::Time,
};

use crate::{
  bind_group::GLBindGroup,
  data_structs::{ComputeState, Params, Telemetry},
  pipeline::GLPipeline,
};

const COMPUTE_WG_SIZE: u32 = 1024;
const DISPLAY_WG_SIZE: u32 = 32;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct GLNodeLabel;

pub struct GLNode {
  last_step_time: Option<f32>,
  target_tps: u32,
}

impl Default for GLNode {
  fn default() -> Self {
    Self {
      last_step_time: None,
      target_tps: 10,
    }
  }
}

impl render_graph::Node for GLNode {
  fn run(
    &self,
    _graph: &mut render_graph::RenderGraphContext,
    render_context: &mut RenderContext,
    world: &World,
  ) -> Result<(), render_graph::NodeRunError> {
    let pipeline_cache = world.resource::<PipelineCache>();
    let pipeline = world.resource::<GLPipeline>();
    let bind_group = world.resource::<GLBindGroup>();

    let Some(state) = world.get_resource::<ComputeState>() else {
      return Ok(());
    };
    let Some(params) = world.get_resource::<Params>() else {
      return Ok(());
    };

    let compute_wg =
      (params.buffer_size_x * params.buffer_size_y + COMPUTE_WG_SIZE - 1) / (COMPUTE_WG_SIZE);
    let display_wg_x = (params.resolution_x + DISPLAY_WG_SIZE - 1) / (DISPLAY_WG_SIZE);
    let display_wg_y = (params.resolution_y + DISPLAY_WG_SIZE - 1) / (DISPLAY_WG_SIZE);

    let mut pass = render_context
      .command_encoder()
      .begin_compute_pass(&ComputePassDescriptor::default());
    pass.set_bind_group(0, &bind_group.0, &[]);

    match state {
      ComputeState::RANDOMIZE => {
        let Some(randomize_pipeline) =
          pipeline_cache.get_compute_pipeline(pipeline.randomize_pipeline)
        else {
          return Ok(());
        };

        pass.set_pipeline(randomize_pipeline);
        pass.dispatch_workgroups(compute_wg, 1, 1);
      }
      ComputeState::STEP => {
        let Some(update_pipeline) = pipeline_cache.get_compute_pipeline(pipeline.update_pipeline)
        else {
          return Ok(());
        };

        pass.set_pipeline(update_pipeline);
        pass.dispatch_workgroups(compute_wg, 1, 1);
      }
      ComputeState::WAIT => {}
    }

    if let Some(display_pipeline) = pipeline_cache.get_compute_pipeline(pipeline.display_pipeline) {
      pass.set_pipeline(display_pipeline);
      pass.dispatch_workgroups(display_wg_x, display_wg_y, 1);
    }

    Ok(())
  }

  fn update(&mut self, world: &mut World) {
    let elapsed_secs = world.resource::<Time>().elapsed_secs();

    match world.get_resource_mut::<ComputeState>() {
      Some(mut state) => match *state {
        ComputeState::RANDOMIZE => {
          *state = ComputeState::STEP;
        }
        ComputeState::STEP => {
          self.last_step_time = Some(elapsed_secs);
          *state = ComputeState::WAIT;
        }
        ComputeState::WAIT => {
          let delta_t = elapsed_secs - self.last_step_time.unwrap();
          if self.target_tps == 0 || delta_t > (1.0 / self.target_tps as f32) {
            *state = ComputeState::STEP;

            let Some(telemetry) = world.get_resource::<Telemetry>() else {
              return;
            };
            let mut data = telemetry.ticks.lock().unwrap();
            if data.len() == telemetry.ticks_len {
              data.remove(0);
            }
            data.push(delta_t);
          }
        }
      },
      None => world.insert_resource(ComputeState::default()),
    }
  }
}

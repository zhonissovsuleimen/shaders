mod bind_group;
mod data_structs;
mod pipeline;
mod render_graph;

use bind_group::{GLBindGroup, prepare_bind_group};
use data_structs::{ComputeState, MainImage, Params, Resolution};

use bevy::{
  app::{Plugin, Startup},
  asset::{Assets, RenderAssetUsages},
  core_pipeline::core_2d::Camera2d,
  ecs::{
    schedule::{
      IntoScheduleConfigs,
      common_conditions::{not, resource_exists},
    },
    system::{Commands, ResMut, Single},
  },
  image::Image,
  log::info,
  math::Vec2,
  render::{
    Render, RenderApp, RenderSet,
    extract_resource::ExtractResourcePlugin,
    render_graph::RenderGraph,
    render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
  },
  sprite::Sprite,
  utils::default,
  window::Window,
};
use pipeline::GLPipeline;
use render_graph::{GLNode, GLNodeLabel};

pub struct GameOfLifePlugin;

impl Plugin for GameOfLifePlugin {
  fn build(&self, app: &mut bevy::app::App) {
    info!("Building pipeline");

    app.add_systems(Startup, setup);

    app.world_mut().commands().spawn(Camera2d);

    app.add_plugins(ExtractResourcePlugin::<Params>::default());
    app.add_plugins(ExtractResourcePlugin::<MainImage>::default());
    app.add_plugins(ExtractResourcePlugin::<Resolution>::default());
    app.add_plugins(ExtractResourcePlugin::<ComputeState>::default());

    let render_app = app.sub_app_mut(RenderApp);

    info!("Preparing bind groups");
    render_app.add_systems(
      Render,
      prepare_bind_group
        .in_set(RenderSet::PrepareBindGroups)
        .run_if(not(resource_exists::<GLBindGroup>)),
    );

    info!("Preparing render graph node");
    let mut render_graph = render_app.world_mut().resource_mut::<RenderGraph>();
    render_graph.add_node(GLNodeLabel, GLNode::default());
    render_graph.add_node_edge(GLNodeLabel, bevy::render::graph::CameraDriverLabel);

    info!("Building pipeline done");
  }

  fn finish(&self, app: &mut bevy::app::App) {
    let render_app = app.sub_app_mut(RenderApp);
    render_app.init_resource::<GLPipeline>();
  }
}

fn setup(mut commands: Commands, window: Single<&Window>, mut image_assets: ResMut<Assets<Image>>) {
  let window_width = window.width();
  let window_height = window.height();
  commands.insert_resource(Resolution(window_width as u32, window_height as u32));

  let cell_count_x = 3200;
  let cell_count_y = 3200;
  let buffer_size_x = (cell_count_x + 31) / 32;
  let buffer_size_y = cell_count_y;
  let buffer_size = buffer_size_x * buffer_size_y;

  commands.insert_resource(Params {
    buffer_size,
    buffer_size_x,
    buffer_size_y,
    offset_x: 0,
    offset_y: 0,
    zoom: 1.0,
  });

  let mut image = Image::new_fill(
    Extent3d {
      width: window_width as u32,
      height: window_height as u32,
      depth_or_array_layers: 1,
    },
    TextureDimension::D2,
    &[0, 0, 0, 255],
    TextureFormat::Rgba8Unorm,
    RenderAssetUsages::RENDER_WORLD,
  );
  image.texture_descriptor.usage =
    TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

  let image_handle = image_assets.add(image.clone());

  commands.spawn((Sprite {
    image: image_handle.clone(),
    custom_size: Some(Vec2 {
      x: window_width as f32,
      y: window_height as f32,
    }),
    ..default()
  },));
  commands.insert_resource(MainImage(image_handle));
}

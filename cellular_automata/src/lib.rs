mod bind_group;
mod data_structs;
mod pipeline;
mod render_graph;
use bind_group::prepare_bind_group;
use data_structs::{CellSize, CellValues, MainImage, Params, Resolution};

use bevy::{
  app::{Plugin, Startup},
  asset::{Assets, RenderAssetUsages},
  core_pipeline::core_2d::Camera2d,
  ecs::{
    schedule::IntoScheduleConfigs,
    system::{Commands, Query, Res, ResMut, Single},
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
  window::{Monitor, Window},
};
use pipeline::CAPipeline;
use render_graph::{CANode, CANodeLabel};

pub struct CellularAutomataPlugin(pub u32);

impl Plugin for CellularAutomataPlugin {
  fn build(&self, app: &mut bevy::app::App) {
    info!("Building pipeline");

    app.insert_resource(CellSize(self.0));
    app.add_systems(Startup, setup);
    app.world_mut().commands().spawn(Camera2d);

    app.add_plugins(ExtractResourcePlugin::<Params>::default());
    app.add_plugins(ExtractResourcePlugin::<MainImage>::default());
    app.add_plugins(ExtractResourcePlugin::<CellValues>::default());
    app.add_plugins(ExtractResourcePlugin::<Resolution>::default());

    let render_app = app.sub_app_mut(RenderApp);

    info!("Preparing bind groups");
    render_app.add_systems(
      Render,
      prepare_bind_group.in_set(RenderSet::PrepareBindGroups),
    );

    info!("Preparing render graph node");
    let mut render_graph = render_app.world_mut().resource_mut::<RenderGraph>();
    render_graph.add_node(CANodeLabel, CANode::default());
    render_graph.add_node_edge(CANodeLabel, bevy::render::graph::CameraDriverLabel);

    info!("Building pipeline done");
  }

  fn finish(&self, app: &mut bevy::app::App) {
    let render_app = app.sub_app_mut(RenderApp);
    render_app.init_resource::<CAPipeline>();
  }
}

fn setup(
  mut commands: Commands,
  // monitors: Query<&Monitor>,
  window: Single<&Window>,
  mut image_assets: ResMut<Assets<Image>>,
  square_size: Res<CellSize>,
) {
  // let (monitor_width, monitor_height): (u32, u32);
  // match monitors.iter().next() {
  //   Some(monitor) => {
  //     (monitor_width, monitor_height) = monitor.physical_size().into();
  //     info!("Monitor was found with resolution {monitor_width}x{monitor_height}");
  //   }
  //   None => {
  //     error!("No motinor is found");
  //     return;
  //   }
  // }
  let window_width = window.physical_width();
  let window_height = window.physical_height();

  let cell_size = square_size.0;
  let count_x = (window_width + cell_size - 1) / cell_size;
  let count_y = (window_height + cell_size - 1) / cell_size;

  commands.insert_resource(Params {
    cell_size,
    count_x,
    count_y,
  });

  commands.insert_resource(Resolution(window_width, window_height));
  commands.insert_resource(CellValues(vec![0; (count_x * count_y) as usize]));

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

mod custom_plugin;

use bevy::{
  app::{App, FixedUpdate, Startup, Update}, asset::{Assets, Handle, RenderAssetUsages}, core_pipeline::core_2d::Camera2d, ecs::{
    resource::Resource,
    system::{Commands, Query, Res, ResMut, Single},
  }, image::Image, math::Vec2, render::{
    extract_resource::ExtractResource,
    render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
  }, sprite::Sprite, time::{Fixed, Time}, utils::default, window::Window, DefaultPlugins
};
use custom_plugin::{CustomPlugin, StorageHandles};

#[derive(Resource, ExtractResource, Clone)]
struct Resolution(f32, f32);

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(CustomPlugin)
    .add_systems(Startup, startup)
    .add_systems(FixedUpdate, update_time)
    .add_systems(Update, resize)
    .insert_resource(Time::<Fixed>::from_hz(60.0))
    .run();
}

fn startup(
  mut commands: Commands,
  mut images: ResMut<Assets<Image>>,
  time: Res<Time>,
  window: Single<&mut Window>,
) {
  let width = window.width();
  let height = window.height();
  commands.insert_resource(Resolution(width, height));

  let mut image = Image::new_fill(
    Extent3d {
      width: width as u32,
      height: height as u32,
      depth_or_array_layers: 1,
    },
    TextureDimension::D2,
    &[0, 0, 0, 255],
    TextureFormat::Rgba8Unorm,
    RenderAssetUsages::RENDER_WORLD,
  );
  image.texture_descriptor.usage =
    TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

  let handle = images.add(image.clone());

  commands.spawn((Sprite {
    image: handle.clone(),
    custom_size: Some(Vec2 {
      x: width,
      y: height,
    }),
    ..default()
  },));
  commands.spawn(Camera2d);

  commands.insert_resource(StorageHandles {
    main_image: handle.clone(),
    time: time.elapsed_secs(),
  });
}

fn update_time(time: Res<Time>, mut storage: ResMut<StorageHandles>) {
  storage.time = time.elapsed_secs();
}
fn resize(
  window: Single<&mut Window>,
  mut images: ResMut<Assets<Image>>,
  mut resolution: ResMut<Resolution>,
  storage: Res<StorageHandles>,
  mut sprite: Single<&mut Sprite>,
) {
  if resolution.0 != window.width() || resolution.1 != window.height() {
    resolution.0 = window.width();
    resolution.1 = window.height();

    if let Some(main_image) = images.get_mut(&storage.main_image) {
      let new_size = Extent3d {
        width: resolution.0 as u32,
        height: resolution.1 as u32,
        depth_or_array_layers: 1,
      };

      main_image.resize(new_size);
    }

    sprite.custom_size = Some(Vec2::new(resolution.0, resolution.1));
  }
}

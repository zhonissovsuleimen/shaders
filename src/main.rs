mod custom_plugin;

use bevy::{
  app::{App, FixedUpdate, Startup}, asset::{Assets, RenderAssetUsages}, core_pipeline::core_2d::Camera2d, ecs::system::{Commands, Res, ResMut}, image::Image, math::Vec2, render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages}, sprite::Sprite, time::{Fixed, Time}, utils::default, DefaultPlugins
};
use custom_plugin::{CustomPlugin, StorageHandles};

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(CustomPlugin)
    .add_systems(Startup, startup)
    .add_systems(FixedUpdate, update_time)
    .insert_resource(Time::<Fixed>::from_hz(60.0))
    .run();
}

fn startup(mut commands: Commands, mut images: ResMut<Assets<Image>>, time: Res<Time>) {
  let mut image = Image::new_fill(
    Extent3d {
      width: 1280,
      height: 720,
      depth_or_array_layers: 1,
    },
    TextureDimension::D2,
    &[0, 128, 255, 255],
    TextureFormat::Rgba8Unorm,
    RenderAssetUsages::RENDER_WORLD,
  );
  image.texture_descriptor.usage =
    TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

  let handle = images.add(image.clone());

  commands.spawn((Sprite {
    image: handle.clone(),
    custom_size: Some(Vec2 {
      x: 1280.0,
      y: 720.0,
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

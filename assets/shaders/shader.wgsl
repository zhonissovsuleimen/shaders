@group(0) @binding(0) var<uniform> time: f32;
@group(0) @binding(1) var input: texture_storage_2d<rgba8unorm, read_write>;

@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
  let location = vec2<i32>(i32(global_id.x), i32(global_id.y));
  let pos = vec2<f32>(f32(location.x), f32(location.y));

  let wave = sin(pos.x * 0.01 + time) + cos(pos.y * 0.01 + time);

  let r = 0.5 + 0.5 * sin(wave + time);
  let g = 0.5 + 0.5 * sin(wave + time + 0.5);
  let b = 0.5 + 0.5 * sin(wave + time + 1.0);

  let new_value = vec4<f32>(r, g, b, 1.0);

  textureStore(input, location, new_value);
}
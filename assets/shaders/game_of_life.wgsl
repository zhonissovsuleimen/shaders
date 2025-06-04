struct Params {
  buffer_size_x: u32,
  buffer_size_y: u32,
  center_x: f32,
  center_y: f32,
  resolution_x: u32,
  resolution_y: u32,
  random_seed: u32,
  zoom: f32,
}

@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var main_image: texture_storage_2d<rgba8unorm, read_write>;
@group(0) @binding(2) var<storage, read_write> buffer: array<u32>;

const COMPUTE_WG_SIZE: u32 = 1024;
const DISPLAY_WG_SIZE: u32 = 32;

@compute @workgroup_size(COMPUTE_WG_SIZE)
fn update(
  @builtin(global_invocation_id) id: vec3<u32>,
) {
  let left_check = id.x % params.buffer_size_x > 0;
  let top_check = id.x / params.buffer_size_x > 0;
  let right_check = id.x % params.buffer_size_x < params.buffer_size_x - 1u;
  let bottom_check = id.x / params.buffer_size_x < params.buffer_size_y - 1u;

  let y_offset = params.buffer_size_x;
  let me = buffer[id.x];
  let left = ternary(left_check, buffer[id.x - 1], 0u);
  let top = ternary(top_check, buffer[id.x - y_offset], 0u);
  let right = ternary(right_check, buffer[id.x + 1], 0u);
  let bottom = ternary(bottom_check, buffer[id.x + y_offset], 0u);
  let top_left = ternary(top_check && left_check, buffer[id.x - y_offset - 1], 0u);
  let top_right = ternary(top_check && right_check, buffer[id.x - y_offset + 1], 0u);
  let bottom_left = ternary(bottom_check && left_check, buffer[id.x + y_offset - 1], 0u);
  let bottom_right = ternary(bottom_check && right_check, buffer[id.x + y_offset + 1], 0u);

  storageBarrier();

  for (var i = 0u; i < 32u; i++) {
    let mask = 1u << i;
    let left_mask = 1u << ((i + 1) % 32u);
    let right_mask = 1u << ((i + 31) % 32u);
    var count = 0u;

    count += u32((top & mask) > 0);
    count += u32((bottom & mask) > 0);

    count += u32((ternary(i == 31u, left, me) & left_mask) > 0);
    count += u32((ternary(i == 0u, right, me) & right_mask) > 0);

    count += u32((ternary(i == 31, top_left, top) & left_mask) > 0);
    count += u32((ternary(i == 0u, top_right, top) & right_mask) > 0);

    count += u32((ternary(i == 31u, bottom_left, bottom) & left_mask) > 0);
    count += u32((ternary(i == 0u, bottom_right, bottom) & right_mask) > 0);


    if (count < 2 || count > 3) {
      buffer[id.x] &= ~mask;
    } else if count == 3 {
      buffer[id.x] |= mask;
    }
  }
}

@compute @workgroup_size(COMPUTE_WG_SIZE)
fn randomize(
  @builtin(global_invocation_id) id: vec3<u32>,
) {
  buffer[id.x] = random_u32(id);
}

@compute @workgroup_size(DISPLAY_WG_SIZE, DISPLAY_WG_SIZE)
fn display(
  @builtin(global_invocation_id) id: vec3<u32>,
) {
  let texture_dim: vec2<u32> = textureDimensions(main_image);

  if (id.x >= texture_dim.x || id.y >= texture_dim.y) {
    return;
  }

  let adjusted_x = params.center_x + (f32(id.x) - f32(params.resolution_x) * 0.5) / params.zoom;
  let adjusted_y = params.center_y + (f32(id.y) - f32(params.resolution_y) * 0.5) / params.zoom;
  let outside_bounds = adjusted_x < 0.0 
    || adjusted_y < 0.0 
    || adjusted_x >= 32.0 * f32(params.buffer_size_x) 
    || adjusted_y >= f32(params.buffer_size_y);

  let id_x = u32(adjusted_x) / 32;
  let id_y = u32(adjusted_y);
  let offset = u32(adjusted_x) % 32;
  let mask = 1u << (31u - offset);
  let cell_alive = (buffer[id_x + id_y * params.buffer_size_x] & mask) > 0;

  var color = vec4<f32>(0.0, 0.0, 0.0, 1.0);

  if (outside_bounds) {
    color = vec4<f32>(1.0, 0.0, 0.0, 1.0);
  } else if (cell_alive) {
    color = vec4<f32>(1.0, 1.0, 1.0, 1.0);
  }

  let location = vec2<i32>(i32(id.x), i32(id.y));
  textureStore(main_image, location, color);
}

fn ternary(cond: bool, a: u32, b: u32) -> u32 {
  if (cond) {
    return a;
  } else {
    return b;
  }
}

fn random_u32(id: vec3<u32>) -> u32 {
  var input = params.random_seed + id.x;
  input ^= 2747636419u;
  input *= 2654435769u;
  input ^= (input >> 16u);
  input *= 2654435769u;
  input ^= (input >> 16u);
  input *= 2654435769u;

  return input;
}
struct Params {
  cell_size: u32,
  count_x: u32,
  count_y: u32,
};

struct Cell {
  state: u32,
};

@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var main_image: texture_storage_2d<rgba8unorm, read_write>;
@group(0) @binding(2) var<storage, read_write> cells_data: array<Cell>;

const WORKGROUP_SIZE: u32 = 16;

@compute @workgroup_size(WORKGROUP_SIZE, WORKGROUP_SIZE, 1)
fn compute(
  @builtin(global_invocation_id) global_id: vec3<u32>,
) {
  if (global_id.x >= params.count_x || global_id.y >= params.count_y) {
    return;
  }

  let checker_pattern = u32(global_id.x + global_id.y) % 2u;

  cells_data[global_id.x + params.count_x * global_id.y].state = checker_pattern;
}

@compute @workgroup_size(WORKGROUP_SIZE, WORKGROUP_SIZE, 1)
fn display(
  @builtin(global_invocation_id) global_id: vec3<u32>,
) {
  let image_dims = textureDimensions(main_image);
  if (global_id.x >= image_dims.x || global_id.y >= image_dims.y) {
    return;
  }

  let id_x = global_id.x / params.cell_size;
  let id_y = global_id.y / params.cell_size;

  let cell_id = id_x + params.count_x * id_y;
  var color = vec4<f32>(1.0, 1.0, 1.0, 1.0);

  if cells_data[cell_id].state == 0 {
    color = vec4<f32>(0.0, 0.0, 0.0, 1.0);
  } 

  let location = vec2<i32>(i32(global_id.x), i32(global_id.y));
  textureStore(main_image, location, color);
}
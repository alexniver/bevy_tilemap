#import bevy_sprite::{
    mesh2d_functions as mesh_functions,
    mesh2d_view_bindings::view,
}

struct VertexIn {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
}

struct VertexOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

struct TileInfoUniform {
    col: u32,
    row: u32,
};

@group(2) @binding(0) var<uniform> tile_info: TileInfoUniform;
@group(2) @binding(1) var texture: texture_2d<f32>;
@group(2) @binding(2) var texture_sampler: sampler;


@vertex
fn vertex(in: VertexIn) -> VertexOut {
    var out: VertexOut;
    // out.clip_position = mesh_functions::mesh2d_position_world_to_clip(vec4<f32>(in.position, 1.0));
    out.clip_position = vec4f(in.position, 1.0);
    out.uv = in.uv;
    return out;
}


@fragment
fn fragment(in: VertexOut) -> @location(0) vec4<f32> {
    var uv = in.uv / vec2f(8.0, 2.0);
    return textureSample(texture, texture_sampler, uv);
}

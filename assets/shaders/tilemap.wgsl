#import bevy_sprite::{
    mesh2d_functions as mesh_functions,
    mesh2d_view_bindings::view,
}

struct VertexIn {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) relative_position: vec2<f32>,
}

struct VertexOut {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) relative_position: vec2<f32>,
}

struct TileInfoUniform {
    col: u32,
    row: u32,
};

@group(2) @binding(0) var<uniform> tile_size: TileInfoUniform;
@group(2) @binding(1) var base_color_texture: texture_2d<f32>;
@group(2) @binding(2) var base_color_sampler: sampler;


@vertex
fn vertex(in: VertexIn) -> VertexOut {
    var out: VertexOut;
    out.clip_position = mesh_functions::mesh2d_position_world_to_clip(vec4<f32>(in.position, 1.0));
    out.relative_position = in.relative_position;
    return out;
}


@fragment
fn fragment(in: VertexOut) -> @location(0) vec4<f32> {
    return vec4<f32>(in.relative_position, 0.0, 1.0);
    // return in.clip_position;
    // return vec4f(0.5, 0.5, 0.0, 1.0);
    // return vec4f(1.0, 1.0, 0.0, 1.0);
}

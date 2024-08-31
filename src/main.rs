//! A shader and a material that uses it.

use bevy::{
    prelude::*,
    reflect::TypePath,
    render::{
        mesh::{Indices, MeshVertexAttribute, PrimitiveTopology},
        render_asset::RenderAssetUsages,
        render_resource::{AsBindGroup, ShaderRef, ShaderType, VertexFormat},
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<TileMapMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}
pub const ATTRIBUTE_RELATIVE_POSITION: MeshVertexAttribute =
    MeshVertexAttribute::new("Relative_Position", 1, VertexFormat::Float32x2);

// Setup a simple 2d scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<TileMapMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // camera
    commands.spawn(Camera2dBundle::default());

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );

    let width = 16. * 16.;
    let height = 16. * 16.;

    // These vertices are specified in 3D space.
    let v_pos = vec![
        [0.0 * width, 0.0 * height, 0.0],
        [1.0 * width, 0.0 * height, 0.0],
        [1.0 * width, 1.0 * height, 0.0],
        [0.0 * width, 1.0 * height, 0.0],
    ];

    let v_relative_pos = vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];

    // Set the position attribute
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
    mesh.insert_attribute(ATTRIBUTE_RELATIVE_POSITION, v_relative_pos);

    let indices = vec![0, 1, 3, 1, 2, 3];
    mesh.insert_indices(Indices::U32(indices));

    // quad
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(mesh).into(),
        material: materials.add(TileMapMaterial {
            tile_size: TileSizeUniform { col: 5, row: 1 },
            color_texture: Some(asset_server.load("images/tilemap.png")),
        }),
        ..default()
    });
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct TileMapMaterial {
    #[uniform(0)]
    tile_size: TileSizeUniform,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
}

#[derive(Debug, ShaderType, Clone)]
#[repr(C)]
struct TileSizeUniform {
    col: u32,
    row: u32,
}

/// The Material2d trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material2d api docs for details!
impl Material2d for TileMapMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/tilemap.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/tilemap.wgsl".into()
    }
}

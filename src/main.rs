//! A shader and a material that uses it.

use bevy::{
    prelude::*,
    reflect::TypePath,
    render::{
        mesh::{Indices, MeshVertexAttribute, PrimitiveTopology},
        render_asset::RenderAssetUsages,
        render_resource::{AsBindGroup, ShaderRef, ShaderType, VertexFormat},
        texture::{
            ImageAddressMode, ImageFilterMode, ImageLoaderSettings, ImageSampler,
            ImageSamplerDescriptor,
        },
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

pub const ATTRIBUTE_POS: MeshVertexAttribute =
    MeshVertexAttribute::new("Position", 0, VertexFormat::Float32x3);

pub const ATTRIBUTE_UV: MeshVertexAttribute =
    MeshVertexAttribute::new("UV", 1, VertexFormat::Float32x2);

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

    // let width = 16. * 16.;
    // let height = 16. * 16.;

    // let width = 16. * 4.;
    // let height = 16. * 4.;
    // let width = 16.;
    // let height = 16.;
    // let width = 1.;
    // let height = 1.;

    // These vertices are specified in 3D space.
    // let v_pos = vec![
    //     [0.0 * width, 0.0 * height, 0.0],
    //     [1.0 * width, 0.0 * height, 0.0],
    //     [1.0 * width, 1.0 * height, 0.0],
    //     [0.0 * width, 1.0 * height, 0.0],
    // ];

    // let v_pos = vec![
    //     [-0.5 * width, -0.5 * height, 0.0],
    //     [0.5 * width, -0.5 * height, 0.0],
    //     [0.5 * width, 0.5 * height, 0.0],
    //     [-0.5 * width, 0.5 * height, 0.0],
    // ];
    let v_pos = vec![
        [-0.5, -0.5, 0.0],
        [0.5, -0.5, 0.0],
        [0.5, 0.5, 0.0],
        [-0.5, 0.5, 0.0],
    ];

    let uvs = vec![[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]];

    // Set the position attribute
    mesh.insert_attribute(ATTRIBUTE_POS, v_pos);
    mesh.insert_attribute(ATTRIBUTE_UV, uvs);

    let indices = vec![0, 1, 3, 1, 2, 3];
    mesh.insert_indices(Indices::U32(indices));
    let image_handle = asset_server.load_with_settings(
        "images/tilemap.png",
        |settings: &mut ImageLoaderSettings| {
            settings.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                mag_filter: ImageFilterMode::Nearest,
                min_filter: ImageFilterMode::Nearest,
                mipmap_filter: ImageFilterMode::Nearest,
                address_mode_u: ImageAddressMode::ClampToEdge,
                address_mode_v: ImageAddressMode::ClampToEdge,
                address_mode_w: ImageAddressMode::ClampToEdge,
                ..Default::default()
            });
        },
    );

    // quad
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(mesh).into(),
        material: materials.add(TileMapMaterial {
            tile_size: TileSizeUniform { col: 8, row: 2 },
            texture: Some(image_handle),
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
    texture: Option<Handle<Image>>,
}

#[derive(Debug, ShaderType, Clone)]
#[repr(C)]
struct TileSizeUniform {
    col: u32,
    row: u32,
}

impl Material2d for TileMapMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/tilemap.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/tilemap.wgsl".into()
    }
}

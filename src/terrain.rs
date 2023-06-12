use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_mesh.on_startup());
    }
}

fn spawn_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::from("Plane"),
        MaterialMeshBundle {
            material: materials.add(StandardMaterial::from(Color::GREEN)),
            mesh: meshes.add(Terrain::new().into()),
            ..Default::default()
        },
    ));
}

struct Terrain {
    indices: Vec<u16>,
    normals: Vec<Vec3>,
    positions: Vec<Vec3>,
}

impl Terrain {
    fn new() -> Self {
        Self {
            positions: vec![Vec3::ZERO, Vec3::X, Vec3::Y],
            indices: vec![0, 1, 2],
            normals: vec![Vec3::Z, Vec3::Z, Vec3::Z],
        }
    }
}

impl From<Terrain> for Mesh {
    fn from(terrain: Terrain) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, terrain.positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, terrain.normals);
        mesh.set_indices(Some(Indices::U16(terrain.indices)));
        mesh
    }
}

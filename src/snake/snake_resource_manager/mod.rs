use bevy::prelude::*;

/// The meshes and materials will always be in memory, but it is not an issue given that
/// A) They don't take much memory
/// B) It is only a snake game that will only be about snake
#[derive(Resource)]
pub struct SnakeResourceManager {
    apple_mesh: Handle<Mesh>,
    apple_materials: Vec<Handle<StandardMaterial>>,
    ball_mesh: Handle<Mesh>,
    ball_materials: Vec<Handle<StandardMaterial>>,
}

impl SnakeResourceManager {
    pub fn apple_mesh(self: &Self) -> Handle<Mesh> {
        self.apple_mesh.clone()
    }

    pub fn apple_materials(self: &Self, material_idx: usize) -> Handle<StandardMaterial> {
        self.apple_materials[material_idx].clone()
    }

    pub fn apple_materials_count(self: &Self) -> usize {
        self.apple_materials.len()
    }

    pub fn ball_mesh(self: &Self) -> Handle<Mesh> {
        self.ball_mesh.clone()
    }

    pub fn ball_material(self: &Self, material_idx: usize) -> Handle<StandardMaterial> {
        self.ball_materials[material_idx].clone()
    }

    pub fn ball_materials_count(self: &Self) -> usize {
        self.ball_materials.len()
    }
}

impl FromWorld for SnakeResourceManager {
    fn from_world(world: &mut World) -> Self {
        let mut mesh_resources = world.resource_mut::<Assets<Mesh>>();
        // TODO : LOAD REAL APPLE MESH
        let apple_mesh = mesh_resources.add(Sphere { radius: 0.5 });
        let ball_mesh = mesh_resources.add(Sphere { radius: 0.5 });

        let mut material_resources = world.resource_mut::<Assets<StandardMaterial>>();
        let mut apple_materials = Vec::new();
        // TODO: LOAD DIFFERENT APPLE MATERIALS
        apple_materials.push(material_resources.add(StandardMaterial {
            base_color: Color::hsl(360., 1., 0.5),
            ..Default::default()
        }));

        let mut ball_materials = Vec::new();
        for i in 0..16 {
            let color = Color::hsl((i as f32 / 16.) * 360., 1.0, 0.5);
            ball_materials.push(material_resources.add(StandardMaterial {
                base_color: color,
                ..Default::default()
            }))
        }

        Self {
            apple_mesh,
            apple_materials,
            ball_mesh,
            ball_materials,
        }
    }
}

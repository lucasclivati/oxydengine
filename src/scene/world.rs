use crate::scene::actor::{Actor, PrimitiveType};
use glam::Vec3;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub actors: Vec<Actor>,
    pub selected_actor_id: Option<u64>,
    pub is_playing: bool,
    pub search_filter: String,
    next_id: u64,
}

impl World {
    pub fn new_empty_scene() -> Self {
        let mut world = Self {
            actors: Vec::new(),
            selected_actor_id: None,
            is_playing: false,
            search_filter: String::new(),
            next_id: 1,
        };

        // Chão e iluminação básica
        world.add_actor("Floor_Plane", PrimitiveType::Cube, Vec3::new(0.0, -1.0, 0.0), [0.3, 0.35, 0.4, 1.0]);
        if let Some(floor) = world.actors.last_mut() {
            floor.transform.scale = Vec3::new(20.0, 0.2, 20.0);
        }

        world.add_actor("Directional_Light", PrimitiveType::PointLight, Vec3::new(2.0, 6.0, 2.0), [1.0, 0.95, 0.8, 1.0]);

        world
    }

    pub fn new_third_person_level() -> Self {
        let mut world = Self::new_empty_scene();

        // Personagem de Terceira Pessoa (Player Actor Azul)
        world.add_actor("BP_ThirdPersonCharacter", PrimitiveType::Cube, Vec3::new(0.0, 0.6, 0.0), [0.1, 0.5, 1.0, 1.0]);
        if let Some(player) = world.actors.last_mut() {
            player.transform.scale = Vec3::new(0.8, 1.6, 0.8);
        }

        // Arena e Obstáculos
        world.add_actor("Arena_Wall_A", PrimitiveType::Cube, Vec3::new(-4.0, 1.0, 0.0), [0.5, 0.5, 0.6, 1.0]);
        world.add_actor("Arena_Wall_B", PrimitiveType::Cube, Vec3::new(4.0, 1.0, 0.0), [0.5, 0.5, 0.6, 1.0]);
        world.add_actor("Collectible_Sphere", PrimitiveType::Sphere, Vec3::new(0.0, 1.0, 3.0), [1.0, 0.8, 0.1, 1.0]);

        world
    }

    pub fn new_first_person_level() -> Self {
        let mut world = Self::new_empty_scene();

        // FPS Player Camera & Target Dummies
        world.add_actor("BP_FirstPersonPlayer", PrimitiveType::Cube, Vec3::new(0.0, 0.9, -2.0), [0.2, 0.8, 0.3, 1.0]);
        world.add_actor("Target_Dummy_1", PrimitiveType::Cube, Vec3::new(-2.0, 1.0, 4.0), [0.9, 0.2, 0.2, 1.0]);
        world.add_actor("Target_Dummy_2", PrimitiveType::Sphere, Vec3::new(2.0, 1.0, 4.0), [0.9, 0.2, 0.2, 1.0]);

        world
    }

    pub fn new_default_scene() -> Self {
        Self::new_third_person_level()
    }

    pub fn add_actor(&mut self, name: &str, primitive: PrimitiveType, position: Vec3, color: [f32; 4]) {
        let actor = match primitive {
            PrimitiveType::Cube => Actor::new_cube(self.next_id, name, position, color),
            PrimitiveType::Sphere => Actor::new_sphere(self.next_id, name, position, color),
            PrimitiveType::PointLight => Actor::new_light(self.next_id, name, position, color, 3.0),
            _ => Actor::new_cube(self.next_id, name, position, color),
        };
        self.next_id += 1;
        self.actors.push(actor);
    }

    pub fn delete_selected(&mut self) {
        if let Some(id) = self.selected_actor_id {
            self.actors.retain(|a| a.id != id);
            self.selected_actor_id = None;
        }
    }

    pub fn get_selected_actor_mut(&mut self) -> Option<&mut Actor> {
        let id = self.selected_actor_id?;
        self.actors.iter_mut().find(|a| a.id == id)
    }

    pub fn update(&mut self, dt: f32) {
        if self.is_playing {
            for actor in &mut self.actors {
                if actor.name.contains("ThirdPerson") || actor.name.contains("FirstPerson") {
                    actor.transform.rotation.y += 45.0 * dt;
                }
            }
        }
    }

    pub fn update_simulation(&mut self, dt: f32) {
        self.update(dt);
    }
}

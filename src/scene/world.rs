use crate::scene::actor::{Actor, PrimitiveType};
use crate::scene::physics::update_physics_for_actor;
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

        // Chão de Terreno Extenso para Paisagem 3D de Mundo Aberto
        world.add_actor("Terrain_Landscape_Floor", PrimitiveType::Cube, Vec3::new(0.0, -0.2, 0.0), [0.18, 0.22, 0.18, 1.0]);
        if let Some(floor) = world.actors.last_mut() {
            floor.transform.scale = Vec3::new(100.0, 0.4, 100.0);
            floor.physics.use_gravity = false;
        }

        // Iluminação Solar Direcional
        world.add_actor("Sun_Directional_Light", PrimitiveType::PointLight, Vec3::new(10.0, 15.0, 10.0), [1.0, 0.95, 0.8, 1.0]);

        world
    }

    pub fn new_main_menu_scene() -> Self {
        let mut world = Self::new_empty_scene();

        // 🏞️ PAISAGEM 3D DE MUNDO ABERTO PARA O MENU INICIAL (Map_MainMenu de AlchemySurvival57old)
        // 1. Centro: Altar Alquímico 3D
        world.add_actor("Alchemical_Altar_Center", PrimitiveType::Cube, Vec3::new(0.0, 0.8, -5.0), [0.85, 0.35, 0.2, 1.0]);
        if let Some(altar) = world.actors.last_mut() {
            altar.transform.scale = Vec3::new(2.5, 1.6, 2.5);
            altar.physics.use_gravity = false;
        }

        // 2. Tochas com Luzes Pontuais Quentes e Frias
        world.add_actor("Torch_Light_Rust_Left", PrimitiveType::PointLight, Vec3::new(-4.0, 3.0, -4.0), [1.0, 0.4, 0.15, 1.0]);
        world.add_actor("Torch_Light_Mystic_Right", PrimitiveType::PointLight, Vec3::new(4.0, 3.0, -4.0), [0.1, 0.6, 1.0, 1.0]);

        // 3. Paisagem de Fundo: Montanhas e Ruínas Alquímicas
        world.add_actor("Mountain_Peak_A", PrimitiveType::Sphere, Vec3::new(-15.0, 5.0, -25.0), [0.12, 0.14, 0.18, 1.0]);
        if let Some(m) = world.actors.last_mut() {
            m.transform.scale = Vec3::new(18.0, 12.0, 18.0);
            m.physics.use_gravity = false;
        }

        world.add_actor("Mountain_Peak_B", PrimitiveType::Sphere, Vec3::new(15.0, 6.0, -28.0), [0.10, 0.12, 0.16, 1.0]);
        if let Some(m) = world.actors.last_mut() {
            m.transform.scale = Vec3::new(22.0, 14.0, 22.0);
            m.physics.use_gravity = false;
        }

        world.add_actor("Ruins_Ancient_Tower", PrimitiveType::Cube, Vec3::new(-8.0, 6.0, -15.0), [0.28, 0.30, 0.36, 1.0]);
        if let Some(t) = world.actors.last_mut() {
            t.transform.scale = Vec3::new(3.0, 12.0, 3.0);
            t.physics.use_gravity = false;
        }

        world.add_actor("Ruins_Arch_Entrance", PrimitiveType::Cube, Vec3::new(8.0, 4.0, -14.0), [0.28, 0.30, 0.36, 1.0]);
        if let Some(a) = world.actors.last_mut() {
            a.transform.scale = Vec3::new(6.0, 8.0, 2.0);
            a.physics.use_gravity = false;
        }

        world
    }

    pub fn new_third_person_level() -> Self {
        let mut world = Self::new_empty_scene();

        world.add_actor("BP_ThirdPersonCharacter", PrimitiveType::Cube, Vec3::new(0.0, 4.0, -6.0), [0.1, 0.5, 1.0, 1.0]);
        if let Some(player) = world.actors.last_mut() {
            player.transform.scale = Vec3::new(0.8, 1.6, 0.8);
            player.physics.use_gravity = true;
        }

        world.add_actor("Arena_Wall_A", PrimitiveType::Cube, Vec3::new(-6.0, 1.5, -6.0), [0.5, 0.5, 0.6, 1.0]);
        world.add_actor("Arena_Wall_B", PrimitiveType::Cube, Vec3::new(6.0, 1.5, -6.0), [0.5, 0.5, 0.6, 1.0]);
        world.add_actor("Collectible_Sphere", PrimitiveType::Sphere, Vec3::new(0.0, 2.0, -2.0), [1.0, 0.8, 0.1, 1.0]);

        world
    }

    pub fn new_first_person_level() -> Self {
        let mut world = Self::new_empty_scene();

        world.add_actor("BP_FirstPersonPlayer", PrimitiveType::Cube, Vec3::new(0.0, 2.0, -5.0), [0.2, 0.8, 0.3, 1.0]);
        world.add_actor("Target_Dummy_1", PrimitiveType::Cube, Vec3::new(-3.0, 1.0, 2.0), [0.9, 0.2, 0.2, 1.0]);
        world.add_actor("Target_Dummy_2", PrimitiveType::Sphere, Vec3::new(3.0, 1.0, 2.0), [0.9, 0.2, 0.2, 1.0]);

        world
    }

    pub fn new_default_scene() -> Self {
        Self::new_main_menu_scene()
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

    pub fn duplicate_selected(&mut self) {
        if let Some(id) = self.selected_actor_id {
            if let Some(actor) = self.actors.iter().find(|a| a.id == id).cloned() {
                let mut dup = actor;
                dup.id = self.next_id;
                self.next_id += 1;
                dup.name = format!("{}_Copy", dup.name);
                dup.transform.position += Vec3::new(1.0, 0.0, 1.0);
                self.actors.push(dup);
                self.selected_actor_id = Some(self.next_id - 1);
            }
        }
    }

    pub fn get_selected_actor_mut(&mut self) -> Option<&mut Actor> {
        let id = self.selected_actor_id?;
        self.actors.iter_mut().find(|a| a.id == id)
    }

    pub fn update(&mut self, dt: f32) {
        if self.is_playing {
            for actor in &mut self.actors {
                update_physics_for_actor(
                    &mut actor.transform.position,
                    &actor.transform.scale,
                    &mut actor.physics,
                    dt,
                );

                if actor.name.contains("ThirdPerson") || actor.name.contains("Orbital") {
                    actor.transform.rotation.y += 45.0 * dt;
                }
            }
        }
    }

    pub fn update_simulation(&mut self, dt: f32) {
        self.update(dt);
    }
}

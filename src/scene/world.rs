use crate::scene::actor::{Actor, PrimitiveType};
use crate::scene::physics::update_physics_for_actor;
use glam::Vec3;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSnapshot {
    pub actors: Vec<Actor>,
    pub selected_actor_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub actors: Vec<Actor>,
    pub selected_actor_id: Option<u64>,
    pub is_playing: bool,
    pub search_filter: String,
    pub next_actor_id: u64,
    #[serde(skip)]
    pub undo_stack: Vec<WorldSnapshot>,
    #[serde(skip)]
    pub redo_stack: Vec<WorldSnapshot>,
}

impl World {
    pub fn create_snapshot(&self) -> WorldSnapshot {
        WorldSnapshot {
            actors: self.actors.clone(),
            selected_actor_id: self.selected_actor_id,
        }
    }

    pub fn push_undo_state(&mut self) {
        let snap = self.create_snapshot();
        self.undo_stack.push(snap);
        if self.undo_stack.len() > 50 {
            self.undo_stack.remove(0);
        }
        self.redo_stack.clear();
    }

    pub fn undo(&mut self) {
        if let Some(prev) = self.undo_stack.pop() {
            let current = self.create_snapshot();
            self.redo_stack.push(current);
            self.actors = prev.actors;
            self.selected_actor_id = prev.selected_actor_id;
        }
    }

    pub fn redo(&mut self) {
        if let Some(next) = self.redo_stack.pop() {
            let current = self.create_snapshot();
            self.undo_stack.push(current);
            self.actors = next.actors;
            self.selected_actor_id = next.selected_actor_id;
        }
    }

    pub fn new_empty_scene() -> Self {
        let mut world = Self {
            actors: Vec::new(),
            selected_actor_id: None,
            is_playing: false,
            search_filter: String::new(),
            next_actor_id: 1,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        };

        // Chão de Terreno Extenso para Paisagem 3D de Mundo Aberto
        world.add_actor("Terrain_Landscape_Floor", PrimitiveType::Cube, Vec3::new(0.0, -0.2, 0.0), [0.18, 0.22, 0.18, 1.0]);
        if let Some(floor) = world.actors.last_mut() {
            floor.transform.scale = Vec3::new(100.0, 0.4, 100.0);
            floor.physics.use_gravity = false;
        }

        // Iluminação Solar Direcional e Atmosfera
        world.actors.push(Actor::new_directional_light(world.next_actor_id, "Sun_Directional_Light", Vec3::new(10.0, 15.0, 10.0)));
        world.next_actor_id += 1;

        world.actors.push(Actor::new_fog(world.next_actor_id, "ExponentialHeightFog", Vec3::ZERO));
        world.next_actor_id += 1;

        world.actors.push(Actor::new_sky_atmosphere(world.next_actor_id, "SkyAtmosphere", Vec3::ZERO));
        world.next_actor_id += 1;

        world.actors.push(Actor::new_volumetric_cloud(world.next_actor_id, "VolumetricCloud", Vec3::ZERO));
        world.next_actor_id += 1;

        world
    }

    pub fn new_main_menu_scene() -> Self {
        let mut world = Self::new_empty_scene();

        // 🏞️ PAISAGEM 3D DE MUNDO ABERTO PARA O MENU INICIAL (Map_MainMenu de AlchemySurvival57old)
        world.add_actor("Alchemical_Altar_Center", PrimitiveType::Cube, Vec3::new(0.0, 0.8, -5.0), [0.85, 0.35, 0.2, 1.0]);
        if let Some(altar) = world.actors.last_mut() {
            altar.transform.scale = Vec3::new(2.5, 1.6, 2.5);
            altar.physics.use_gravity = false;
        }

        world.add_actor("Torch_Light_Rust_Left", PrimitiveType::PointLight, Vec3::new(-4.0, 3.0, -4.0), [1.0, 0.4, 0.15, 1.0]);
        world.add_actor("Torch_Light_Mystic_Right", PrimitiveType::PointLight, Vec3::new(4.0, 3.0, -4.0), [0.1, 0.6, 1.0, 1.0]);

        world.add_actor("Mountain_Peak_A", PrimitiveType::Sphere, Vec3::new(-15.0, 5.0, -25.0), [0.12, 0.14, 0.18, 1.0]);
        if let Some(m) = world.actors.last_mut() {
            m.transform.scale = Vec3::new(18.0, 12.0, 18.0);
            m.physics.use_gravity = false;
        }

        world.add_actor("Mountain_Peak_B", PrimitiveType::Sphere, Vec3::new(15.0, 6.0, -28.0), [0.10, 0.12, 0.16, 1.0]);
        if let Some(m) = world.actors.last_mut() {
            m.transform.scale = Vec3::new(20.0, 14.0, 20.0);
            m.physics.use_gravity = false;
        }

        world.add_actor("Ruins_Ancient_Tower", PrimitiveType::Cube, Vec3::new(-8.0, 4.0, -12.0), [0.35, 0.30, 0.28, 1.0]);
        if let Some(t) = world.actors.last_mut() {
            t.transform.scale = Vec3::new(3.0, 8.0, 3.0);
            t.physics.use_gravity = false;
        }

        world.add_actor("Ruins_Arch_Entrance", PrimitiveType::Cube, Vec3::new(8.0, 3.0, -10.0), [0.38, 0.32, 0.29, 1.0]);
        if let Some(a) = world.actors.last_mut() {
            a.transform.scale = Vec3::new(5.0, 6.0, 1.5);
            a.physics.use_gravity = false;
        }

        world
    }

    pub fn new_default_scene() -> Self {
        let mut world = Self::new_empty_scene();

        world.add_actor("Cube", PrimitiveType::Cube, Vec3::new(0.0, 1.0, 0.0), [0.8, 0.4, 0.2, 1.0]);
        world.add_actor("PointLight", PrimitiveType::PointLight, Vec3::new(2.0, 4.0, 2.0), [1.0, 0.9, 0.6, 1.0]);
        world.add_actor("Camera_Actor", PrimitiveType::CameraActor, Vec3::new(0.0, 2.0, 5.0), [0.4, 0.6, 1.0, 1.0]);

        world
    }

    pub fn new_third_person_level() -> Self {
        let mut world = Self::new_empty_scene();

        world.add_actor("Player_Pawn", PrimitiveType::Sphere, Vec3::new(0.0, 1.0, 0.0), [0.1, 0.8, 0.4, 1.0]);
        if let Some(p) = world.actors.last_mut() {
            p.physics.use_gravity = true;
            p.physics.mass = 75.0;
        }

        world.add_actor("Lobby_Building_A", PrimitiveType::Cube, Vec3::new(-6.0, 3.0, -8.0), [0.4, 0.4, 0.45, 1.0]);
        if let Some(b) = world.actors.last_mut() {
            b.transform.scale = Vec3::new(6.0, 6.0, 8.0);
            b.physics.use_gravity = false;
        }

        world.add_actor("Camera_Actor", PrimitiveType::CameraActor, Vec3::new(0.0, 3.0, 8.0), [0.4, 0.6, 1.0, 1.0]);

        world
    }

    pub fn new_first_person_level() -> Self {
        let mut world = Self::new_empty_scene();

        world.add_actor("City_Building_Block_1", PrimitiveType::Cube, Vec3::new(-10.0, 8.0, -15.0), [0.3, 0.32, 0.35, 1.0]);
        if let Some(b) = world.actors.last_mut() {
            b.transform.scale = Vec3::new(8.0, 16.0, 8.0);
            b.physics.use_gravity = false;
        }

        world.add_actor("Zombie_Spawner_Alpha", PrimitiveType::Cube, Vec3::new(5.0, 0.5, -5.0), [0.9, 0.1, 0.1, 1.0]);
        world.add_actor("Camera_Actor", PrimitiveType::CameraActor, Vec3::new(0.0, 2.0, 4.0), [0.4, 0.6, 1.0, 1.0]);

        world
    }

    pub fn add_actor(&mut self, name: &str, primitive: PrimitiveType, position: Vec3, color: [f32; 4]) {
        self.push_undo_state();
        let id = self.next_actor_id;
        self.next_actor_id += 1;
        let actor = Actor::new(id, name, primitive, position, color);
        self.actors.push(actor);
        self.selected_actor_id = Some(id);
    }

    pub fn get_selected_actor_mut(&mut self) -> Option<&mut Actor> {
        if let Some(id) = self.selected_actor_id {
            self.actors.iter_mut().find(|a| a.id == id)
        } else {
            None
        }
    }

    pub fn delete_selected(&mut self) {
        if let Some(id) = self.selected_actor_id {
            self.push_undo_state();
            self.actors.retain(|a| a.id != id);
            self.selected_actor_id = None;
        }
    }

    pub fn update_simulation(&mut self, dt: f32) {
        if self.is_playing {
            for actor in &mut self.actors {
                update_physics_for_actor(&mut actor.transform.position, &actor.transform.scale, &mut actor.physics, dt);
            }
        }
    }
}

use crate::scene::actor::{Actor, PrimitiveType};
use crate::scene::physics::update_physics_for_actor;
use glam::Vec3;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSnapshot {
    pub actors: Vec<Actor>,
    pub selected_actor_id: Option<u64>,
    pub selected_actor_ids: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub actors: Vec<Actor>,
    pub selected_actor_id: Option<u64>,
    #[serde(default)]
    pub selected_actor_ids: Vec<u64>,
    pub is_playing: bool,
    pub search_filter: String,
    pub next_actor_id: u64,
    #[serde(skip)]
    pub last_selected_index: Option<usize>,
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
            selected_actor_ids: self.selected_actor_ids.clone(),
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
            self.selected_actor_ids = prev.selected_actor_ids;
        }
    }

    pub fn redo(&mut self) {
        if let Some(next) = self.redo_stack.pop() {
            let current = self.create_snapshot();
            self.undo_stack.push(current);
            self.actors = next.actors;
            self.selected_actor_id = next.selected_actor_id;
            self.selected_actor_ids = next.selected_actor_ids;
        }
    }

    pub fn is_actor_selected(&self, id: u64) -> bool {
        self.selected_actor_id == Some(id) || self.selected_actor_ids.contains(&id)
    }

    pub fn select_single_actor(&mut self, id: u64, idx: usize) {
        self.selected_actor_id = Some(id);
        self.selected_actor_ids = vec![id];
        self.last_selected_index = Some(idx);
    }

    pub fn toggle_select_actor(&mut self, id: u64, idx: usize) {
        if let Some(pos) = self.selected_actor_ids.iter().position(|&x| x == id) {
            self.selected_actor_ids.remove(pos);
            if self.selected_actor_id == Some(id) {
                self.selected_actor_id = self.selected_actor_ids.first().copied();
            }
        } else {
            self.selected_actor_ids.push(id);
            self.selected_actor_id = Some(id);
        }
        self.last_selected_index = Some(idx);
    }

    pub fn select_range_actors(&mut self, target_idx: usize) {
        let start_idx = self.last_selected_index.unwrap_or(0);
        let (min, max) = if start_idx <= target_idx { (start_idx, target_idx) } else { (target_idx, start_idx) };

        self.selected_actor_ids.clear();
        for i in min..=max {
            if let Some(actor) = self.actors.get(i) {
                self.selected_actor_ids.push(actor.id);
            }
        }
        if let Some(actor) = self.actors.get(target_idx) {
            self.selected_actor_id = Some(actor.id);
        }
    }

    pub fn clear_selection(&mut self) {
        self.selected_actor_id = None;
        self.selected_actor_ids.clear();
        self.last_selected_index = None;
    }

    pub fn new_empty_scene() -> Self {
        Self {
            actors: Vec::new(),
            selected_actor_id: None,
            selected_actor_ids: Vec::new(),
            is_playing: false,
            search_filter: String::new(),
            next_actor_id: 1,
            last_selected_index: None,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn new_default_scene() -> Self {
        let mut world = Self::new_empty_scene();

        world.add_actor("Floor_Terrain", PrimitiveType::Cube, Vec3::new(0.0, -1.0, 0.0), [0.22, 0.25, 0.30, 1.0]);
        if let Some(floor) = world.actors.last_mut() {
            floor.transform.scale = Vec3::new(20.0, 1.0, 20.0);
        }

        world.add_actor("Alchemical_Rust_Cube", PrimitiveType::Cube, Vec3::new(0.0, 0.5, 0.0), [0.8, 0.4, 0.1, 1.0]);
        world.add_actor("Light_Point_Rust", PrimitiveType::PointLight, Vec3::new(2.0, 3.0, 1.0), [1.0, 0.9, 0.6, 1.0]);

        let sun_id = world.next_actor_id;
        world.next_actor_id += 1;
        world.actors.push(Actor::new_directional_light(sun_id, "Sun_Directional_Light", Vec3::new(0.0, 10.0, 0.0)));

        let fog_id = world.next_actor_id;
        world.next_actor_id += 1;
        world.actors.push(Actor::new_fog(fog_id, "ExponentialHeightFog", Vec3::ZERO));

        let sky_id = world.next_actor_id;
        world.next_actor_id += 1;
        world.actors.push(Actor::new_sky_atmosphere(sky_id, "SkyAtmosphere", Vec3::ZERO));

        let cloud_id = world.next_actor_id;
        world.next_actor_id += 1;
        world.actors.push(Actor::new_volumetric_cloud(cloud_id, "VolumetricCloud", Vec3::ZERO));

        let cam_id = world.next_actor_id;
        world.next_actor_id += 1;
        world.actors.push(Actor::new_camera(cam_id, "MainCameraActor", Vec3::new(0.0, 3.0, 8.0)));

        world
    }

    pub fn new_main_menu_scene() -> Self {
        let mut world = Self::new_empty_scene();

        world.add_actor("MainMenu_Landscape", PrimitiveType::Cube, Vec3::new(0.0, -2.0, 0.0), [0.18, 0.20, 0.25, 1.0]);
        if let Some(floor) = world.actors.last_mut() {
            floor.transform.scale = Vec3::new(50.0, 1.0, 50.0);
        }

        world.add_actor("Alchemical_Altar", PrimitiveType::Cube, Vec3::new(0.0, 0.0, -5.0), [0.85, 0.45, 0.15, 1.0]);
        if let Some(altar) = world.actors.last_mut() {
            altar.transform.scale = Vec3::new(3.0, 4.0, 3.0);
        }

        let sun_id = world.next_actor_id;
        world.next_actor_id += 1;
        world.actors.push(Actor::new_directional_light(sun_id, "Sun_Directional_Light", Vec3::new(0.0, 15.0, 0.0)));

        let fog_id = world.next_actor_id;
        world.next_actor_id += 1;
        world.actors.push(Actor::new_fog(fog_id, "ExponentialHeightFog", Vec3::ZERO));

        let sky_id = world.next_actor_id;
        world.next_actor_id += 1;
        world.actors.push(Actor::new_sky_atmosphere(sky_id, "SkyAtmosphere", Vec3::ZERO));

        let cloud_id = world.next_actor_id;
        world.next_actor_id += 1;
        world.actors.push(Actor::new_volumetric_cloud(cloud_id, "VolumetricCloud", Vec3::ZERO));

        world
    }

    pub fn new_first_person_level() -> Self {
        let mut world = Self::new_default_scene();
        world.add_actor("Zombie_Arena_Floor", PrimitiveType::Cube, Vec3::new(0.0, -1.0, 0.0), [0.25, 0.28, 0.35, 1.0]);
        if let Some(floor) = world.actors.last_mut() {
            floor.transform.scale = Vec3::new(60.0, 1.0, 60.0);
        }
        world.add_actor("Ruins_Wall_North", PrimitiveType::Cube, Vec3::new(0.0, 2.0, -25.0), [0.5, 0.4, 0.3, 1.0]);
        if let Some(wall) = world.actors.last_mut() {
            wall.transform.scale = Vec3::new(50.0, 6.0, 2.0);
        }
        world.add_actor("Ruins_Arch_Entrance", PrimitiveType::Cube, Vec3::new(-10.0, 3.0, 5.0), [0.7, 0.35, 0.15, 1.0]);
        if let Some(arch) = world.actors.last_mut() {
            arch.transform.scale = Vec3::new(4.0, 8.0, 4.0);
        }
        world
    }

    pub fn new_third_person_level() -> Self {
        let mut world = Self::new_default_scene();
        world.add_actor("Lobby_Hall_Floor", PrimitiveType::Cube, Vec3::new(0.0, -1.0, 0.0), [0.15, 0.18, 0.24, 1.0]);
        if let Some(floor) = world.actors.last_mut() {
            floor.transform.scale = Vec3::new(40.0, 1.0, 40.0);
        }
        world.add_actor("Torch_Light_Rust_Left", PrimitiveType::PointLight, Vec3::new(-6.0, 3.0, 0.0), [1.0, 0.6, 0.2, 1.0]);
        world.add_actor("Torch_Light_Mystic_Right", PrimitiveType::PointLight, Vec3::new(6.0, 3.0, 0.0), [0.3, 0.7, 1.0, 1.0]);
        world
    }

    pub fn add_actor(&mut self, name: &str, primitive: PrimitiveType, position: Vec3, color: [f32; 4]) {
        self.push_undo_state();
        let id = self.next_actor_id;
        self.next_actor_id += 1;
        let mut actor = Actor::new(id, name, primitive, position, color);
        
        if primitive == PrimitiveType::Sphere {
            actor.transform.scale = Vec3::new(1.0, 1.0, 1.0);
        }

        self.actors.push(actor);
        self.selected_actor_id = Some(id);
        self.selected_actor_ids = vec![id];
        self.last_selected_index = Some(self.actors.len() - 1);
    }

    pub fn get_selected_actor(&self) -> Option<&Actor> {
        if let Some(id) = self.selected_actor_id {
            self.actors.iter().find(|a| a.id == id)
        } else {
            None
        }
    }

    pub fn get_selected_actor_mut(&mut self) -> Option<&mut Actor> {
        if let Some(id) = self.selected_actor_id {
            self.actors.iter_mut().find(|a| a.id == id)
        } else {
            None
        }
    }

    pub fn delete_selected(&mut self) {
        if !self.selected_actor_ids.is_empty() || self.selected_actor_id.is_some() {
            self.push_undo_state();
            let ids_to_delete = self.selected_actor_ids.clone();
            let single_id = self.selected_actor_id;

            self.actors.retain(|a| !ids_to_delete.contains(&a.id) && single_id != Some(a.id));
            self.clear_selection();
        }
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
            }
        }
    }
}

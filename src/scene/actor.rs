use glam::Vec3;
use serde::{Serialize, Deserialize};
use crate::scene::physics::PhysicsComponent;
use crate::scene::material::MaterialInstance;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PrimitiveType {
    Cube,
    Sphere,
    Plane,
    PointLight,
    DirectionalLight,
    SkyLight,
    ExponentialHeightFog,
    SkyAtmosphere,
    VolumetricCloud,
    CameraActor,
    DecalActor,
    CharacterBP,
    StaticMesh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
    pub lock_scale_aspect: bool,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,
            lock_scale_aspect: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecalComponent {
    pub decal_size: Vec3,
    pub fade_screen_size: f32,
    pub sort_order: i32,
    pub blend_mode: String,
    pub texture_name: String,
}

impl Default for DecalComponent {
    fn default() -> Self {
        Self {
            decal_size: Vec3::new(2.0, 2.0, 2.0),
            fade_screen_size: 0.01,
            sort_order: 0,
            blend_mode: "Translucent".to_string(),
            texture_name: "T_Decal_AlchemicalSymbol".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterComponent {
    pub character_class: String,
    pub max_health: f32,
    pub current_health: f32,
    pub walk_speed: f32,
    pub jump_z_velocity: f32,
    pub animation_state: String,
    pub equipped_weapon: String,
}

impl Default for CharacterComponent {
    fn default() -> Self {
        Self {
            character_class: "BP_DoctorCharacter".to_string(),
            max_health: 100.0,
            current_health: 100.0,
            walk_speed: 600.0,
            jump_z_velocity: 420.0,
            animation_state: "Idle_Alchemist".to_string(),
            equipped_weapon: "W_AlchemicalFlask".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraComponent {
    pub field_of_view: f32,
    pub near_clip_plane: f32,
    pub far_clip_plane: f32,
    pub is_active_camera: bool,
}

impl Default for CameraComponent {
    fn default() -> Self {
        Self {
            field_of_view: 90.0,
            near_clip_plane: 0.1,
            far_clip_plane: 1000.0,
            is_active_camera: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtmosphereComponent {
    pub fog_density: f32,
    pub fog_height_falloff: f32,
    pub rayleigh_scattering: f32,
    pub mie_scattering: f32,
    pub cloud_coverage: f32,
    pub cloud_altitude: f32,
}

impl Default for AtmosphereComponent {
    fn default() -> Self {
        Self {
            fog_density: 0.04,
            fog_height_falloff: 0.2,
            rayleigh_scattering: 0.033,
            mie_scattering: 0.004,
            cloud_coverage: 0.5,
            cloud_altitude: 6000.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actor {
    pub id: u64,
    pub name: String,
    pub primitive: PrimitiveType,
    pub transform: Transform,
    pub color: [f32; 4],
    pub material: MaterialInstance,
    pub intensity: f32,
    pub visible: bool,
    pub is_visible: bool,
    pub physics: PhysicsComponent,
    pub camera_component: Option<CameraComponent>,
    pub atmosphere_component: Option<AtmosphereComponent>,
    pub decal_component: Option<DecalComponent>,
    pub character_component: Option<CharacterComponent>,
}

impl Actor {
    pub fn new(id: u64, name: &str, primitive: PrimitiveType, position: Vec3, color: [f32; 4]) -> Self {
        let mut physics = PhysicsComponent::default();
        if primitive == PrimitiveType::PointLight || primitive == PrimitiveType::DirectionalLight || primitive == PrimitiveType::CameraActor || primitive == PrimitiveType::DecalActor {
            physics.use_gravity = false;
        }

        Self {
            id,
            name: name.to_string(),
            primitive,
            transform: Transform {
                position,
                rotation: Vec3::ZERO,
                scale: Vec3::ONE,
                lock_scale_aspect: false,
            },
            color,
            material: MaterialInstance::alchemical_rust(),
            intensity: 1.0,
            visible: true,
            is_visible: true,
            physics,
            camera_component: None,
            atmosphere_component: None,
            decal_component: None,
            character_component: None,
        }
    }

    pub fn new_decal(id: u64, name: &str, position: Vec3) -> Self {
        let mut actor = Self::new(id, name, PrimitiveType::DecalActor, position, [0.95, 0.55, 0.15, 0.8]);
        actor.decal_component = Some(DecalComponent::default());
        actor
    }

    pub fn new_doctor_character(id: u64, name: &str, position: Vec3) -> Self {
        let mut actor = Self::new(id, name, PrimitiveType::CharacterBP, position, [0.2, 0.8, 0.4, 1.0]);
        actor.transform.scale = Vec3::new(1.0, 2.0, 1.0);
        actor.character_component = Some(CharacterComponent::default());
        actor
    }

    pub fn new_camera(id: u64, name: &str, position: Vec3) -> Self {
        let mut actor = Self::new(id, name, PrimitiveType::CameraActor, position, [0.4, 0.6, 1.0, 1.0]);
        actor.camera_component = Some(CameraComponent::default());
        actor
    }

    pub fn new_directional_light(id: u64, name: &str, position: Vec3) -> Self {
        let mut actor = Self::new(id, name, PrimitiveType::DirectionalLight, position, [1.0, 0.95, 0.8, 1.0]);
        actor.intensity = 10.0;
        actor
    }

    pub fn new_fog(id: u64, name: &str, position: Vec3) -> Self {
        let mut actor = Self::new(id, name, PrimitiveType::ExponentialHeightFog, position, [0.6, 0.7, 0.8, 0.5]);
        actor.atmosphere_component = Some(AtmosphereComponent::default());
        actor
    }

    pub fn new_sky_atmosphere(id: u64, name: &str, position: Vec3) -> Self {
        let mut actor = Self::new(id, name, PrimitiveType::SkyAtmosphere, position, [0.3, 0.5, 0.9, 1.0]);
        actor.atmosphere_component = Some(AtmosphereComponent::default());
        actor
    }

    pub fn new_volumetric_cloud(id: u64, name: &str, position: Vec3) -> Self {
        let mut actor = Self::new(id, name, PrimitiveType::VolumetricCloud, position, [0.9, 0.95, 1.0, 0.8]);
        actor.atmosphere_component = Some(AtmosphereComponent::default());
        actor
    }
}

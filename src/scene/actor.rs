use glam::Vec3;
use serde::{Serialize, Deserialize};
use crate::scene::physics::PhysicsComponent;

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
    pub intensity: f32,
    pub visible: bool,
    pub is_visible: bool,
    pub physics: PhysicsComponent,
    pub camera_component: Option<CameraComponent>,
    pub atmosphere_component: Option<AtmosphereComponent>,
}

impl Actor {
    pub fn new(id: u64, name: &str, primitive: PrimitiveType, position: Vec3, color: [f32; 4]) -> Self {
        let mut physics = PhysicsComponent::default();
        let is_env = matches!(
            primitive,
            PrimitiveType::PointLight | PrimitiveType::DirectionalLight | PrimitiveType::SkyLight |
            PrimitiveType::ExponentialHeightFog | PrimitiveType::SkyAtmosphere | PrimitiveType::VolumetricCloud |
            PrimitiveType::Plane | PrimitiveType::CameraActor
        );

        if is_env {
            physics.use_gravity = false;
        }

        let camera_component = if primitive == PrimitiveType::CameraActor {
            Some(CameraComponent::default())
        } else {
            None
        };

        let atmosphere_component = if matches!(
            primitive,
            PrimitiveType::ExponentialHeightFog | PrimitiveType::SkyAtmosphere | PrimitiveType::VolumetricCloud
        ) {
            Some(AtmosphereComponent::default())
        } else {
            None
        };

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
            intensity: 1.0,
            visible: true,
            is_visible: true,
            physics,
            camera_component,
            atmosphere_component,
        }
    }

    pub fn new_cube(id: u64, name: &str, position: Vec3, color: [f32; 4]) -> Self {
        Self::new(id, name, PrimitiveType::Cube, position, color)
    }

    pub fn new_sphere(id: u64, name: &str, position: Vec3, color: [f32; 4]) -> Self {
        Self::new(id, name, PrimitiveType::Sphere, position, color)
    }

    pub fn new_light(id: u64, name: &str, position: Vec3, color: [f32; 4], intensity: f32) -> Self {
        let mut actor = Self::new(id, name, PrimitiveType::PointLight, position, color);
        actor.intensity = intensity;
        actor
    }

    pub fn new_directional_light(id: u64, name: &str, position: Vec3) -> Self {
        let mut actor = Self::new(id, name, PrimitiveType::DirectionalLight, position, [1.0, 0.95, 0.8, 1.0]);
        actor.intensity = 2.5;
        actor
    }

    pub fn new_fog(id: u64, name: &str, position: Vec3) -> Self {
        Self::new(id, name, PrimitiveType::ExponentialHeightFog, position, [0.7, 0.8, 0.95, 1.0])
    }

    pub fn new_sky_atmosphere(id: u64, name: &str, position: Vec3) -> Self {
        Self::new(id, name, PrimitiveType::SkyAtmosphere, position, [0.4, 0.65, 0.95, 1.0])
    }

    pub fn new_volumetric_cloud(id: u64, name: &str, position: Vec3) -> Self {
        Self::new(id, name, PrimitiveType::VolumetricCloud, position, [0.9, 0.9, 0.95, 1.0])
    }

    pub fn new_camera(id: u64, name: &str, position: Vec3) -> Self {
        Self::new(id, name, PrimitiveType::CameraActor, position, [0.4, 0.6, 1.0, 1.0])
    }
}

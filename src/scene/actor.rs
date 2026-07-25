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
    CameraActor,
    StaticMesh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,
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
}

impl Actor {
    pub fn new(id: u64, name: &str, primitive: PrimitiveType, position: Vec3, color: [f32; 4]) -> Self {
        let mut physics = PhysicsComponent::default();
        if primitive == PrimitiveType::PointLight || primitive == PrimitiveType::DirectionalLight || primitive == PrimitiveType::Plane {
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
            },
            color,
            intensity: 1.0,
            visible: true,
            is_visible: true,
            physics,
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
        actor.physics.use_gravity = false;
        actor
    }
}

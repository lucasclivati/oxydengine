use glam::{Vec3, Quat, Mat4};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PrimitiveType {
    Cube,
    Sphere,
    Plane,
    DirectionalLight,
    PointLight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3, // Graus (Pitch, Yaw, Roll)
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

impl Transform {
    pub fn matrix(&self) -> Mat4 {
        let rot_quat = Quat::from_euler(
            glam::EulerRot::YXZ,
            self.rotation.y.to_radians(),
            self.rotation.x.to_radians(),
            self.rotation.z.to_radians(),
        );
        Mat4::from_scale_rotation_translation(self.scale, rot_quat, self.position)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actor {
    pub id: u64,
    pub name: String,
    pub visible: bool,
    pub transform: Transform,
    pub primitive: PrimitiveType,
    pub color: [f32; 4],
    pub intensity: f32,
    pub light_intensity: f32,
}

impl Actor {
    pub fn new_cube(id: u64, name: &str, position: Vec3, color: [f32; 4]) -> Self {
        Self {
            id,
            name: name.to_string(),
            visible: true,
            transform: Transform {
                position,
                ..Default::default()
            },
            primitive: PrimitiveType::Cube,
            color,
            intensity: 1.0,
            light_intensity: 1.0,
        }
    }

    pub fn new_sphere(id: u64, name: &str, position: Vec3, color: [f32; 4]) -> Self {
        Self {
            id,
            name: name.to_string(),
            visible: true,
            transform: Transform {
                position,
                ..Default::default()
            },
            primitive: PrimitiveType::Sphere,
            color,
            intensity: 1.0,
            light_intensity: 1.0,
        }
    }

    pub fn new_light(id: u64, name: &str, position: Vec3, color: [f32; 4], intensity: f32) -> Self {
        Self {
            id,
            name: name.to_string(),
            visible: true,
            transform: Transform {
                position,
                ..Default::default()
            },
            primitive: PrimitiveType::PointLight,
            color,
            intensity,
            light_intensity: intensity,
        }
    }
}

use glam::Vec3;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsComponent {
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub mass: f32,
    pub use_gravity: bool,
    pub is_grounded: bool,
    pub bounding_box_min: Vec3,
    pub bounding_box_max: Vec3,
}

impl Default for PhysicsComponent {
    fn default() -> Self {
        Self {
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            mass: 1.0,
            use_gravity: true,
            is_grounded: false,
            bounding_box_min: Vec3::new(-0.5, -0.5, -0.5),
            bounding_box_max: Vec3::new(0.5, 0.5, 0.5),
        }
    }
}

pub fn update_physics_for_actor(
    position: &mut Vec3,
    scale: &Vec3,
    physics: &mut PhysicsComponent,
    dt: f32,
) {
    if !physics.use_gravity && physics.velocity.length_squared() < 0.0001 {
        return;
    }

    // Aplicação da gravidade (-9.8 m/s²) se o objeto não estiver no chão
    if physics.use_gravity && !physics.is_grounded {
        physics.velocity.y += -9.8 * dt;
    }

    // Atualização da posição com base na velocidade
    let mut next_pos = *position + physics.velocity * dt;

    // Detecção e resposta de colisão com o Chão (Y = 0.0)
    let half_height = scale.y * 0.5;
    let ground_level = 0.0;

    if next_pos.y - half_height <= ground_level {
        next_pos.y = ground_level + half_height;
        physics.velocity.y = 0.0;
        physics.is_grounded = true;
    } else {
        physics.is_grounded = false;
    }

    *position = next_pos;
}

pub fn check_aabb_collision(
    pos_a: Vec3, scale_a: Vec3,
    pos_b: Vec3, scale_b: Vec3,
) -> bool {
    let min_a = pos_a - scale_a * 0.5;
    let max_a = pos_a + scale_a * 0.5;
    let min_b = pos_b - scale_b * 0.5;
    let max_b = pos_b + scale_b * 0.5;

    min_a.x <= max_b.x && max_a.x >= min_b.x &&
    min_a.y <= max_b.y && max_a.y >= min_b.y &&
    min_a.z <= max_b.z && max_a.z >= min_b.z
}

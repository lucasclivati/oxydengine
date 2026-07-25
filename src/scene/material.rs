use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    pub name: String,
    pub base_color: [f32; 4],
    pub metallic: f32,
    pub roughness: f32,
    pub specular: f32,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            name: "M_AlchemicalRust".to_string(),
            base_color: [0.85, 0.35, 0.2, 1.0],
            metallic: 0.8,
            roughness: 0.45,
            specular: 0.5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialInstance {
    pub name: String,
    pub parent_material_name: String,
    pub color_tint: [f32; 4],
    pub metallic_override: f32,
    pub roughness_override: f32,
    pub specular_override: f32,
}

impl Default for MaterialInstance {
    fn default() -> Self {
        Self {
            name: "MI_AlchemicalRust_Inst".to_string(),
            parent_material_name: "M_AlchemicalRust".to_string(),
            color_tint: [0.85, 0.35, 0.2, 1.0],
            metallic_override: 0.8,
            roughness_override: 0.45,
            specular_override: 0.5,
        }
    }
}

impl MaterialInstance {
    pub fn alchemical_rust() -> Self {
        Self {
            name: "MI_AlchemicalRust".to_string(),
            parent_material_name: "M_AlchemicalRust".to_string(),
            color_tint: [0.85, 0.35, 0.2, 1.0],
            metallic_override: 0.85,
            roughness_override: 0.4,
            specular_override: 0.6,
        }
    }

    pub fn terrain_grass() -> Self {
        Self {
            name: "MI_TerrainGrass".to_string(),
            parent_material_name: "M_TerrainGrass".to_string(),
            color_tint: [0.18, 0.24, 0.18, 1.0],
            metallic_override: 0.05,
            roughness_override: 0.9,
            specular_override: 0.2,
        }
    }

    pub fn ancient_stone() -> Self {
        Self {
            name: "MI_AncientStone".to_string(),
            parent_material_name: "M_AncientStone".to_string(),
            color_tint: [0.35, 0.30, 0.28, 1.0],
            metallic_override: 0.1,
            roughness_override: 0.8,
            specular_override: 0.3,
        }
    }

    pub fn industrial_metal() -> Self {
        Self {
            name: "MI_IndustrialMetal".to_string(),
            parent_material_name: "M_IndustrialMetal".to_string(),
            color_tint: [0.4, 0.45, 0.5, 1.0],
            metallic_override: 0.95,
            roughness_override: 0.25,
            specular_override: 0.8,
        }
    }
}

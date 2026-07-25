use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

// 24 Vértices com tons de pedra, pedra alquímica e ferrugem industrial
pub const CUBE_VERTICES: &[Vertex] = &[
    // Front face (Ferrugem Alquímica Quente)
    Vertex { position: [-0.5, -0.5,  0.5], color: [0.65, 0.32, 0.22] },
    Vertex { position: [ 0.5, -0.5,  0.5], color: [0.65, 0.32, 0.22] },
    Vertex { position: [ 0.5,  0.5,  0.5], color: [0.65, 0.32, 0.22] },
    Vertex { position: [-0.5,  0.5,  0.5], color: [0.65, 0.32, 0.22] },

    // Back face (Cinza Pedra de Ruínas)
    Vertex { position: [-0.5, -0.5, -0.5], color: [0.22, 0.25, 0.30] },
    Vertex { position: [-0.5,  0.5, -0.5], color: [0.22, 0.25, 0.30] },
    Vertex { position: [ 0.5,  0.5, -0.5], color: [0.22, 0.25, 0.30] },
    Vertex { position: [ 0.5, -0.5, -0.5], color: [0.22, 0.25, 0.30] },

    // Top face (Ouro / Âmbar Alquímico Escuro)
    Vertex { position: [-0.5,  0.5, -0.5], color: [0.55, 0.40, 0.18] },
    Vertex { position: [-0.5,  0.5,  0.5], color: [0.55, 0.40, 0.18] },
    Vertex { position: [ 0.5,  0.5,  0.5], color: [0.55, 0.40, 0.18] },
    Vertex { position: [ 0.5,  0.5, -0.5], color: [0.55, 0.40, 0.18] },

    // Bottom face (Charcoal Ardoisia)
    Vertex { position: [-0.5, -0.5, -0.5], color: [0.15, 0.16, 0.20] },
    Vertex { position: [ 0.5, -0.5, -0.5], color: [0.15, 0.16, 0.20] },
    Vertex { position: [ 0.5, -0.5,  0.5], color: [0.15, 0.16, 0.20] },
    Vertex { position: [-0.5, -0.5,  0.5], color: [0.15, 0.16, 0.20] },

    // Right face (Terracota Escuro)
    Vertex { position: [ 0.5, -0.5, -0.5], color: [0.50, 0.24, 0.16] },
    Vertex { position: [ 0.5,  0.5, -0.5], color: [0.50, 0.24, 0.16] },
    Vertex { position: [ 0.5,  0.5,  0.5], color: [0.50, 0.24, 0.16] },
    Vertex { position: [ 0.5, -0.5,  0.5], color: [0.50, 0.24, 0.16] },

    // Left face (Cinza Grafite)
    Vertex { position: [-0.5, -0.5, -0.5], color: [0.20, 0.22, 0.26] },
    Vertex { position: [-0.5, -0.5,  0.5], color: [0.20, 0.22, 0.26] },
    Vertex { position: [-0.5,  0.5,  0.5], color: [0.20, 0.22, 0.26] },
    Vertex { position: [-0.5,  0.5, -0.5], color: [0.20, 0.22, 0.26] },
];

pub const CUBE_INDICES: &[u16] = &[
    0, 1, 2, 2, 3, 0,       // Front
    4, 5, 6, 6, 7, 4,       // Back
    8, 9, 10, 10, 11, 8,    // Top
    12, 13, 14, 14, 15, 12, // Bottom
    16, 17, 18, 18, 19, 16, // Right
    20, 21, 22, 22, 23, 20, // Left
];

pub struct CubeMesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

impl CubeMesh {
    pub fn new(device: &wgpu::Device) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Cube Vertex Buffer"),
            contents: bytemuck::cast_slice(CUBE_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Cube Index Buffer"),
            contents: bytemuck::cast_slice(CUBE_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            vertex_buffer,
            index_buffer,
            num_indices: CUBE_INDICES.len() as u32,
        }
    }
}

pub struct SphereMesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

impl SphereMesh {
    pub fn new(device: &wgpu::Device) -> Self {
        let latitude_bands = 20;
        let longitude_bands = 20;
        let radius = 0.5;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for lat in 0..=latitude_bands {
            let theta = lat as f32 * std::f32::consts::PI / latitude_bands as f32;
            let sin_theta = theta.sin();
            let cos_theta = theta.cos();

            for lon in 0..=longitude_bands {
                let phi = lon as f32 * 2.0 * std::f32::consts::PI / longitude_bands as f32;
                let sin_phi = phi.sin();
                let cos_phi = phi.cos();

                let x = cos_phi * sin_theta;
                let y = cos_theta;
                let z = sin_phi * sin_theta;

                // Cor gradiente suave para destacar a curvatura da esfera 3D
                let r = 0.6 + 0.3 * x;
                let g = 0.4 + 0.3 * y;
                let b = 0.2 + 0.3 * z;

                vertices.push(Vertex {
                    position: [x * radius, y * radius, z * radius],
                    color: [r, g, b],
                });
            }
        }

        for lat in 0..latitude_bands {
            for lon in 0..longitude_bands {
                let first = (lat * (longitude_bands + 1) + lon) as u16;
                let second = first + longitude_bands as u16 + 1 ;

                indices.push(first);
                indices.push(second);
                indices.push(first + 1);

                indices.push(second);
                indices.push(second + 1);
                indices.push(first + 1);
            }
        }

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Sphere Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Sphere Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            vertex_buffer,
            index_buffer,
            num_indices: indices.len() as u32,
        }
    }
}

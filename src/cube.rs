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

// 24 Vértices (4 por face para normais/cores distintas por face do cubo 3D)
pub const VERTICES: &[Vertex] = &[
    // Front face (Azul Claro / Ciano Elétrico)
    Vertex { position: [-0.5, -0.5,  0.5], color: [0.1, 0.6, 1.0] },
    Vertex { position: [ 0.5, -0.5,  0.5], color: [0.1, 0.6, 1.0] },
    Vertex { position: [ 0.5,  0.5,  0.5], color: [0.1, 0.6, 1.0] },
    Vertex { position: [-0.5,  0.5,  0.5], color: [0.1, 0.6, 1.0] },

    // Back face (Azul Escuro / Profundo)
    Vertex { position: [-0.5, -0.5, -0.5], color: [0.0, 0.2, 0.6] },
    Vertex { position: [-0.5,  0.5, -0.5], color: [0.0, 0.2, 0.6] },
    Vertex { position: [ 0.5,  0.5, -0.5], color: [0.0, 0.2, 0.6] },
    Vertex { position: [ 0.5, -0.5, -0.5], color: [0.0, 0.2, 0.6] },

    // Top face (Azul Néon Vivo)
    Vertex { position: [-0.5,  0.5, -0.5], color: [0.2, 0.5, 1.0] },
    Vertex { position: [-0.5,  0.5,  0.5], color: [0.2, 0.5, 1.0] },
    Vertex { position: [ 0.5,  0.5,  0.5], color: [0.2, 0.5, 1.0] },
    Vertex { position: [ 0.5,  0.5, -0.5], color: [0.2, 0.5, 1.0] },

    // Bottom face (Azul Marinho)
    Vertex { position: [-0.5, -0.5, -0.5], color: [0.05, 0.15, 0.4] },
    Vertex { position: [ 0.5, -0.5, -0.5], color: [0.05, 0.15, 0.4] },
    Vertex { position: [ 0.5, -0.5,  0.5], color: [0.05, 0.15, 0.4] },
    Vertex { position: [-0.5, -0.5,  0.5], color: [0.05, 0.15, 0.4] },

    // Right face (Azul Royal)
    Vertex { position: [ 0.5, -0.5, -0.5], color: [0.15, 0.4, 0.95] },
    Vertex { position: [ 0.5,  0.5, -0.5], color: [0.15, 0.4, 0.95] },
    Vertex { position: [ 0.5,  0.5,  0.5], color: [0.15, 0.4, 0.95] },
    Vertex { position: [ 0.5, -0.5,  0.5], color: [0.15, 0.4, 0.95] },

    // Left face (Azul Cobalto)
    Vertex { position: [-0.5, -0.5, -0.5], color: [0.0, 0.35, 0.8] },
    Vertex { position: [-0.5, -0.5,  0.5], color: [0.0, 0.35, 0.8] },
    Vertex { position: [-0.5,  0.5,  0.5], color: [0.0, 0.35, 0.8] },
    Vertex { position: [-0.5,  0.5, -0.5], color: [0.0, 0.35, 0.8] },
];

pub const INDICES: &[u16] = &[
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
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Cube Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            vertex_buffer,
            index_buffer,
            num_indices: INDICES.len() as u32,
        }
    }
}

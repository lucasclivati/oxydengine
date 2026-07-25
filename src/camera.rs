use glam::{Mat4, Vec3};
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub mvp_matrix: [[f32; 4]; 4],
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            mvp_matrix: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }
}

pub struct CameraController {
    pub aspect_ratio: f32,
    pub rotation_x: f32,
    pub rotation_y: f32,
    pub uniforms: Uniforms,
    pub uniform_buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl CameraController {
    pub fn new(device: &wgpu::Device, aspect_ratio: f32) -> Self {
        let uniforms = Uniforms::new();

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Camera Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let mut controller = Self {
            aspect_ratio,
            rotation_x: 0.0,
            rotation_y: 0.0,
            uniforms,
            uniform_buffer,
            bind_group_layout,
            bind_group,
        };

        controller.update_matrix();
        controller
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if height > 0 {
            self.aspect_ratio = width as f32 / height as f32;
            self.update_matrix();
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Velocidade de rotação nos eixos X e Y
        self.rotation_x += 0.8 * dt;
        self.rotation_y += 1.2 * dt;
        self.update_matrix();
    }

    fn update_matrix(&mut self) {
        let proj = Mat4::perspective_rh(
            45.0f32.to_radians(),
            self.aspect_ratio,
            0.1,
            100.0,
        );

        let view = Mat4::look_at_rh(
            Vec3::new(0.0, 1.5, 3.0), // Posição da Câmera
            Vec3::ZERO,               // Alvo (centro)
            Vec3::Y,                  // Vetor Up
        );

        let model = Mat4::from_rotation_x(self.rotation_x) * Mat4::from_rotation_y(self.rotation_y);

        let mvp = proj * view * model;
        self.uniforms.mvp_matrix = mvp.to_cols_array_2d();
    }

    pub fn write_buffer(&self, queue: &wgpu::Queue) {
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[self.uniforms]));
    }
}

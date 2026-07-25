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
    pub camera_pos: Vec3,
    pub pitch: f32,
    pub yaw: f32,
    pub move_speed: f32,
    pub look_sensitivity: f32,
    pub fov_degrees: f32,
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
            camera_pos: Vec3::new(0.0, 6.0, 15.0),
            pitch: -18.0f32.to_radians(),
            yaw: -90.0f32.to_radians(),
            move_speed: 12.0,
            look_sensitivity: 0.003,
            fov_degrees: 45.0,
            uniforms,
            uniform_buffer,
            bind_group_layout,
            bind_group,
        };

        controller.update_matrix();
        controller
    }

    pub fn forward(&self) -> Vec3 {
        let cos_pitch = self.pitch.cos();
        Vec3::new(
            self.yaw.cos() * cos_pitch,
            self.pitch.sin(),
            self.yaw.sin() * cos_pitch,
        ).normalize()
    }

    pub fn right(&self) -> Vec3 {
        self.forward().cross(Vec3::Y).normalize()
    }

    pub fn up(&self) -> Vec3 {
        self.right().cross(self.forward()).normalize()
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if height > 0 {
            self.aspect_ratio = width as f32 / height as f32;
            self.update_matrix();
        }
    }

    // NAVEGAÇÃO DE VÔO NO VIEWPORT (Botão Direito + WASD / QE + Mouse Drag)
    pub fn process_input(&mut self, ctx: &egui::Context, dt: f32) {
        let input = ctx.input(|i| i.clone());

        if input.pointer.button_down(egui::PointerButton::Secondary) {
            let delta = input.pointer.delta();
            if delta.length_sq() > 0.0 {
                self.yaw += delta.x * self.look_sensitivity;
                self.pitch -= delta.y * self.look_sensitivity;
                self.pitch = self.pitch.clamp(-89.0f32.to_radians(), 89.0f32.to_radians());
            }

            let speed = self.move_speed * dt;
            let fwd = self.forward();
            let rgt = self.right();

            if input.key_down(egui::Key::W) { self.camera_pos += fwd * speed; }
            if input.key_down(egui::Key::S) { self.camera_pos -= fwd * speed; }
            if input.key_down(egui::Key::A) { self.camera_pos -= rgt * speed; }
            if input.key_down(egui::Key::D) { self.camera_pos += rgt * speed; }
            if input.key_down(egui::Key::E) { self.camera_pos += Vec3::Y * speed; }
            if input.key_down(egui::Key::Q) { self.camera_pos -= Vec3::Y * speed; }
        }

        if input.pointer.button_down(egui::PointerButton::Middle) {
            let delta = input.pointer.delta();
            let pan_speed = 0.025;
            self.camera_pos -= self.right() * delta.x * pan_speed;
            self.camera_pos += self.up() * delta.y * pan_speed;
        }

        if input.smooth_scroll_delta.y != 0.0 {
            let zoom_speed = 0.6;
            self.camera_pos += self.forward() * input.smooth_scroll_delta.y * zoom_speed;
        }

        self.update_matrix();
    }

    // FOCO AUTOMÁTICO NA CÂMERA COM DISTÂNCIA EXPANDIDA PARA VER O OBJETO TOTALMENTE VISÍVEL
    pub fn focus_target(&mut self, target: Vec3) {
        self.camera_pos = target + Vec3::new(0.0, 6.0, 14.0);
        let dir = (target - self.camera_pos).normalize();
        self.pitch = dir.y.asin();
        self.yaw = dir.z.atan2(dir.x);
        self.update_matrix();
    }

    pub fn update(&mut self, _dt: f32) {
        self.update_matrix();
    }

    pub fn get_view_projection(&self) -> Mat4 {
        let proj = Mat4::perspective_rh(
            self.fov_degrees.to_radians(),
            self.aspect_ratio,
            0.1,
            300.0,
        );

        let target = self.camera_pos + self.forward();
        let view = Mat4::look_at_rh(
            self.camera_pos,
            target,
            Vec3::Y,
        );

        proj * view
    }

    fn update_matrix(&mut self) {
        let vp = self.get_view_projection();
        self.uniforms.mvp_matrix = vp.to_cols_array_2d();
    }

    pub fn update_actor_matrix(&mut self, queue: &wgpu::Queue, model_matrix: Mat4) {
        let vp = self.get_view_projection();
        let mvp = vp * model_matrix;
        let uniforms = Uniforms {
            mvp_matrix: mvp.to_cols_array_2d(),
        };
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
    }
}

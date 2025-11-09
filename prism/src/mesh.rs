use glam::Mat4;

use crate::{Texture, Vertex};

pub struct Mesh {
    pub(crate) vertices: Vec<Vertex>,
    pub(crate) indices: Vec<u32>,
    pub(crate) textures: Vec<Texture>,

    pub(crate) vao: u32,
    pub(crate) vbo: u32,
    pub(crate) ebo: u32,

    pub(crate) position: glam::Vec3,
    pub(crate) rotation: glam::Vec3,
    pub(crate) scale: glam::Vec3,

    pub(crate) model: glam::Mat4,
}

impl Mesh {
    pub fn set_rotation_x(&mut self, angle_rad: f32) {
        self.rotation.x = angle_rad;
        self.update_model_matrix();
    }

    pub fn set_rotation_y(&mut self, angle_rad: f32) {
        self.rotation.y = angle_rad;
        self.update_model_matrix();
    }

    pub fn set_rotation_z(&mut self, angle_rad: f32) {
        self.rotation.z = angle_rad;
        self.update_model_matrix();
    }

    pub fn set_translation(&mut self, position: glam::Vec3) {
        self.position = position;
        self.update_model_matrix();
    }

    pub fn set_scale(&mut self, scale: glam::Vec3) {
        self.scale = scale;
        self.update_model_matrix();
    }

    fn update_model_matrix(&mut self) {
        let translation = Mat4::from_translation(self.position);
        let rotation_x = Mat4::from_rotation_x(self.rotation.x);
        let rotation_y = Mat4::from_rotation_y(self.rotation.y);
        let rotation_z = Mat4::from_rotation_z(self.rotation.z);
        let scale = Mat4::from_scale(self.scale);

        self.model = translation * rotation_z * rotation_y * rotation_x * scale;
    }
}
use prism::{Camera, glm::Vec3};

use crate::components::Component;

pub struct CameraComponent {
    prism_camera: Camera
}

impl Component for CameraComponent {

}

impl CameraComponent {
    pub(crate) fn get_prism_camera(&self) -> Camera {
        self.prism_camera
    }

    pub fn new() -> Self {
        Self {
            prism_camera: Camera::new(Vec3::ZERO, Vec3::Y, -90.0f32.to_radians(), 0.0f32.to_radians())
        }
    }

    pub fn translate(&mut self, vector: Vec3) {
        self.prism_camera.adjust_position(vector);
    }
}
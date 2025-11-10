use std::collections::HashMap;

use prism::{Camera, PrismRenderer, Shader};
use uuid::Uuid;

use crate::{components::camera::CameraComponent, objects::Object};

pub struct Scene {
    pub(crate) id: Uuid,
    pub(crate) upper_screen_camera_id: Option<Uuid>,
    pub(crate) lower_screen_camera_id: Option<Uuid>, // Camera is optional for bottom screen if you don't want to render anything there
    pub(crate) upper_screen_objects: HashMap<Uuid, Object>,
    pub(crate) lower_screen_objects: HashMap<Uuid, Object>,
}

impl Scene {
    pub(crate) fn update_upper(&mut self, delta_time: f32) {
        //todo!()
    }

    pub(crate) fn render_upper(&mut self, delta_time: f32, renderer: &mut PrismRenderer, shader: &mut Shader) {
        let camera_object = self.upper_screen_objects.get(&self.upper_screen_camera_id.unwrap()).unwrap();
        let prism_camera: Camera = camera_object.try_get_component::<CameraComponent>().unwrap().get_prism_camera();
        for object in self.upper_screen_objects.values_mut() {
            object.render(delta_time, renderer, &prism_camera, shader);
        }
    }

    pub(crate) fn update_lower(&mut self, delta_time: f32) {
        //todo!()
    }

    pub(crate) fn render_lower(&mut self, delta_time: f32, renderer: &mut PrismRenderer) {
        //todo!()
    }
    
    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn add_object_upper(&mut self, object: Object) {
        self.upper_screen_objects.insert(object.id, object);
    }

    pub fn set_upper_camera(&mut self, id: Uuid) {
        self.upper_screen_camera_id = Some(id);
    }
}
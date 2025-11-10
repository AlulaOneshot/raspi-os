use std::{any::{Any, TypeId}, collections::HashMap};

use prism::{Shader, glm::{Mat4, Vec3}};
use uuid::Uuid;

use crate::components::{Component, mesh::MeshComponent};

pub struct Object {
    pub(crate) id: Uuid,
    name: String,
    components: HashMap<TypeId, Box<dyn Any>>,
}

impl Object {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    pub fn try_get_component<T: Component + 'static>(&self) -> Option<&T> {
        self.components
            .get(&TypeId::of::<T>())
            .and_then(|component| Some(component.as_ref().downcast_ref::<T>().expect(format!("ShiotaEngine: Type Error: Type {} was found in Component Table. Component Table should only contain dyn Components.", std::any::type_name::<T>()).as_str())))
    }

    pub fn add_component<T: Component + 'static>(&mut self, component: T) {
        self.components
            .insert(TypeId::of::<T>(), Box::new(component));
    }

    pub(crate) fn update(&mut self, delta_time: f32) {
        todo!("Implement Object Update Logic");
    }

    pub(crate) fn render(&self, delta_time: f32, renderer: &mut prism::PrismRenderer, camera: &prism::Camera, shader: &mut Shader) {
        if let Some(mesh_component) = self.try_get_component::<MeshComponent>() {
            let projection_transform = Mat4::perspective_rh(45.0f32.to_radians(), 800.0 / 480.0, 0.1, 100.0);
            shader.set_uniform_mat4("view", camera.get_view_matrix());
            shader.set_uniform_mat4("projection", projection_transform);
            shader.set_uniform_vec3("viewPos", camera.get_position());
            shader.set_uniform_vec3("material.ambient", Vec3::new(0.725, 0.949, 1.0));
            shader.set_uniform_vec3("material.diffuse", Vec3::new(0.745, 0.949, 1.0));
            shader.set_uniform_vec3("material.specular", Vec3::new(0.5, 0.5, 0.5));
            shader.set_uniform_float("material.shininess", 32.0);
            shader.set_uniform_vec3("light.position", Vec3::new(1.2, 1.0, 8.0));
            shader.set_uniform_vec3("light.ambient", Vec3::new(0.2, 0.2, 0.2));
            shader.set_uniform_vec3("light.diffuse", Vec3::new(0.5, 0.5, 0.5));
            shader.set_uniform_vec3("light.specular", Vec3::new(1.0, 1.0, 1.0));
            renderer.draw_mesh(&mesh_component.mesh, shader);
        }
    }

    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "New Object".to_string(),
            components: HashMap::new(),
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }
}
use std::collections::HashMap;

use prism::{PrismRenderer, Shader, glm::Vec4};
use uuid::{Uuid, uuid};

use crate::scene::Scene;

pub mod components;
pub mod objects;
pub mod scene;

pub use prism::glm;

pub struct Engine {
    application_name: String,
    rendering_context: PrismRenderer,
    scenes: HashMap<Uuid, Scene>,
    main_scene_id: Option<Uuid>,
    current_scene: Option<Uuid>,
    shader: Option<Shader>
}

impl Engine {
    pub fn run(&mut self) -> Result<(), String> {
        let vert_shader = 
        r#"#version 330 core

        layout (location = 0) in vec3 aPos;
        layout (location = 1) in vec3 aNormal;
        layout (location = 2) in vec2 aTexCoords;

        out vec3 FragPos;
        out vec3 Normal;
        out vec2 TexCoords;

        uniform mat4 model;
        uniform mat4 view;
        uniform mat4 projection;

        void main() {
            FragPos = vec3(model * vec4(aPos, 1.0));
            Normal = mat3(transpose(inverse(model))) * aNormal;  
            TexCoords = aTexCoords;

            gl_Position = projection * view * vec4(FragPos, 1.0);
        }"#;

        let frag_shader = 
        r#"#version 330 core
        out vec4 FragColor;

        in vec3 FragPos;
        in vec3 Normal;
        in vec2 TexCoords;

        uniform sampler2D texture_0;
        uniform vec3 viewPos;

        struct Light {
            vec3 position;

            vec3 ambient;
            vec3 diffuse;
            vec3 specular;
        };

        struct Material {
            vec3 ambient;
            vec3 diffuse;
            vec3 specular;
            float shininess;
        };

        uniform Material material;
        uniform Light light;

        void main() {
            vec3 ambient = light.ambient * material.ambient;

            // diffuse
            vec3 norm = normalize(Normal);
            vec3 lightDir = normalize(light.position - FragPos);
            float diff = max(dot(norm, lightDir), 0.0);
            vec3 diffuse = light.diffuse * (diff * material.diffuse);

            // specular
            vec3 viewDir = normalize(viewPos - FragPos);
            vec3 reflectDir = reflect(-lightDir, norm);
            float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
            vec3 specular = light.specular * (spec * material.specular);

            vec3 result = (ambient + diffuse + specular) * texture(texture_0, TexCoords).rgb;
            FragColor = vec4(result, 1.0);
        }
        "#;

        if let Some(id) = self.main_scene_id {
            if let Some(_) = self.scenes.get(&id) {
                self.current_scene = Some(id);
            }
        }
        else {
            return Err("No main scene was set".to_string())
        }

        let ctx = &mut self.rendering_context;

        let shader = ctx.create_shader_from_source(vert_shader, frag_shader).unwrap();
        self.shader = Some(shader);

        while !ctx.should_close() {
            let current_scene = self.scenes.get_mut(&self.current_scene.expect("The scene should have been loaded.")).unwrap();
            ctx.handle_events();
            ctx.begin_upper_screen();
            ctx.clear_screen(Vec4::new(0.0, 0.0, 0.0, 1.0));
            current_scene.update_upper(ctx.get_delta());
            current_scene.render_upper(ctx.get_delta(), ctx, &mut self.shader.as_mut().unwrap());
            ctx.end_upper_screen();
            ctx.begin_lower_screen();
            current_scene.update_lower(ctx.get_delta());
            current_scene.render_lower(ctx.get_delta(), ctx);
            ctx.end_lower_screen();
        }

        Ok(())
    }

    pub fn create_scene(&mut self) -> &mut Scene {
        let scene = Scene {
            id: Uuid::new_v4(),
            upper_screen_camera_id: None,
            lower_screen_camera_id: None,
            upper_screen_objects: HashMap::new(),
            lower_screen_objects: HashMap::new(),
        };
        let id = scene.id;
        self.scenes.insert(id, scene);
        self.scenes.get_mut(&id).unwrap()
    }

    pub fn set_main_scene(&mut self, scene: Uuid) {
        //TODO: Validate scene exists
        self.main_scene_id = Some(scene);
    }
}

pub struct EngineBuilder {
    application_name: String,
}

impl EngineBuilder {
    pub fn new() -> Self {
        Self {
            application_name: String::from("Shiota Application"),
        }
    }

    pub fn application_name(mut self, name: &str) -> Self {
        self.application_name = String::from(name);
        self
    }

    pub fn build(self) -> Result<Engine, String> {
        let mut context = PrismRenderer::new();
        match context.init() {
            Ok(()) => {
                Ok(Engine {
                    application_name: self.application_name,
                    rendering_context: context,
                    scenes: HashMap::new(),
                    main_scene_id: None,
                    current_scene: None,
                    shader: None
                })
            },
            Err(s) => {
                return Err(s);
            }
        }
    }
}
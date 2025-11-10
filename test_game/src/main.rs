use shiota_engine::{EngineBuilder, components::camera::CameraComponent, glm::Vec3, objects::Object};

fn main() {
    let mut engine = EngineBuilder::new().application_name("Test Application").build().expect("Failed to build engine");

    let id;
    {
        let main_scene = engine.create_scene();
        id = main_scene.get_id();
        let mut camera = Object::new();
        let camera_id = camera.get_id();
        camera.set_name("Camera");
        let mut camera_component = CameraComponent::new();
        camera_component.translate(Vec3::new(0.0, 0.0, 5.0));
        camera.add_component(camera_component);
        main_scene.add_object_upper(camera);
        main_scene.set_upper_camera(camera_id);
    }
    engine.set_main_scene(id);

    match engine.run() {
        Ok(()) => {

        },
        Err(e) => {
            eprintln!("Engine encountered an error: {}", e);
        }
    }
}

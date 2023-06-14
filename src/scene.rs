use math;
use std::vec::Vec;
use crate::model;
use crate::camera::Camera;
use sdl2;

pub struct Scene
{
    models: Vec<model::Mesh>,
    projection: math::matrix::Matrix4,
    pub camera: Camera
}

impl Scene
{
    pub fn new(models: Vec<model::Mesh>, camera: Camera) -> Self
    {
        let projection = math::matrix::Matrix4::new_perspective(45.0f32.to_radians(), 900.0/700.0, 0.1, 100.0);

        Self {
            models,
            projection,
            camera
        }
    }

    pub fn process_input(&mut self, event: &sdl2::event::Event)
    {
        self.camera.update_camera(event);
        for model in &mut self.models
        {
            model.update_pos(event);
        }
    }

    pub fn draw(&self)
    {
        for model in &self.models
        {
            model.render(self.camera.view(), &self.projection);
        }
    }
}